use crate::core::dbal::query::fragments;
use async_trait::async_trait;
use shaku::Component;
use sqlx::{MySqlPool, Row};

use crate::core::{errors::AppError, model::sys_dept::SysDeptModel};

use super::interface::ISysDeptRepository;

#[derive(Component, Clone)]
#[shaku(interface = ISysDeptRepository)]
pub(crate) struct SysDeptRepository {
    pool: MySqlPool,
}

impl SysDeptRepository {
    pub(crate) async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysDeptModel>, AppError> {
        let (kw, like) = fragments::keyword_args(keyword);
        sqlx::query_as::<_, SysDeptModel>(
            r#"
            SELECT id, parent_id, dept_name, leader, phone, status
            FROM sys_dept
            WHERE is_deleted = 0
              AND (? = '' OR dept_name LIKE ? OR leader LIKE ?)
            ORDER BY parent_id ASC, order_num ASC, id ASC
            "#,
        )
        .bind(&kw)
        .bind(&like)
        .bind(&like)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询部门失败: {err}")))
    }

    pub(crate) async fn get_by_id(&self, id: u64) -> Result<Option<SysDeptModel>, AppError> {
        sqlx::query_as::<_, SysDeptModel>(
            r#"
            SELECT id, parent_id, dept_name, leader, phone, status
            FROM sys_dept
            WHERE id = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询部门失败: {err}")))
    }

    pub(crate) async fn insert(&self, model: &SysDeptModel) -> Result<u64, AppError> {
        let ancestors = self.resolve_ancestors(model.parent_id).await?;
        let result = sqlx::query(
            r#"
            INSERT INTO sys_dept (
                parent_id, ancestors, dept_name, order_num, leader, phone, status,
                created_by, updated_by, is_deleted
            ) VALUES (?, ?, ?, 0, ?, ?, ?, 1, 1, 0)
            "#,
        )
        .bind(model.parent_id)
        .bind(ancestors)
        .bind(&model.dept_name)
        .bind(model.leader.as_deref())
        .bind(model.phone.as_deref())
        .bind(model.status)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增部门失败: {err}")))?;

        Ok(result.last_insert_id())
    }

    pub(crate) async fn update_by_id(
        &self,
        id: u64,
        model: &SysDeptModel,
    ) -> Result<bool, AppError> {
        let ancestors = self.resolve_ancestors(model.parent_id).await?;
        let result = sqlx::query(
            r#"
            UPDATE sys_dept
            SET parent_id = ?, ancestors = ?, dept_name = ?, leader = ?, phone = ?, status = ?, updated_by = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(model.parent_id)
        .bind(ancestors)
        .bind(&model.dept_name)
        .bind(model.leader.as_deref())
        .bind(model.phone.as_deref())
        .bind(model.status)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新部门失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    pub(crate) async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_dept
            SET is_deleted = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("删除部门失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn resolve_ancestors(&self, parent_id: u64) -> Result<String, AppError> {
        if parent_id == 0 {
            return Ok("0".to_string());
        }

        let parent = sqlx::query(
            r#"
            SELECT ancestors
            FROM sys_dept
            WHERE id = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(parent_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询上级部门失败: {err}")))?;

        let Some(parent) = parent else {
            return Err(AppError::bad_request("上级部门不存在"));
        };

        let parent_ancestors = parent
            .get::<Option<String>, _>("ancestors")
            .unwrap_or_else(|| "0".to_string());
        let normalized = if parent_ancestors.trim().is_empty() {
            "0".to_string()
        } else {
            parent_ancestors
        };

        Ok(format!("{normalized},{parent_id}"))
    }
}

#[async_trait]
impl ISysDeptRepository for SysDeptRepository {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysDeptModel>, AppError> {
        self.list(keyword).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysDeptModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysDeptModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, model: &SysDeptModel) -> Result<bool, AppError> {
        self.update_by_id(id, model).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}
