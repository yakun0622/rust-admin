use crate::core::dbal::query::fragments;
use crate::core::dto::sys_dept_dto::{SysDeptListQueryDto, SysDeptUpdateReqDto};
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{MySql, MySqlPool, QueryBuilder, Row};

use crate::core::{errors::AppError, model::sys_dept::SysDeptModel};

#[async_trait]
pub trait ISysDeptRepository: Interface {
    async fn list(&self, query: SysDeptListQueryDto) -> Result<Vec<SysDeptModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysDeptModel>, AppError>;
    async fn insert(&self, model: &SysDeptModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysDeptUpdateReqDto) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysDeptRepository)]
pub(crate) struct SysDeptRepository {
    pool: MySqlPool,
}

impl SysDeptRepository {
    pub(crate) async fn list(
        &self,
        query: SysDeptListQueryDto,
    ) -> Result<Vec<SysDeptModel>, AppError> {
        let (name_kw, name_like) = fragments::keyword_args(query.name.as_deref());
        let status = query.status.filter(|s| !s.trim().is_empty());

        sqlx::query_as::<_, SysDeptModel>(
            r#"
            SELECT id, parent_id, dept_name, leader, phone, status
            FROM sys_dept
            WHERE is_deleted = 0
              AND (? = '' OR dept_name LIKE ?)
              AND (? IS NULL OR status = ?)
            ORDER BY parent_id ASC, order_num ASC, id ASC
            "#,
        )
        .bind(&name_kw)
        .bind(&name_like)
        .bind(&status)
        .bind(&status)
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
        dto: SysDeptUpdateReqDto,
    ) -> Result<bool, AppError> {
        let mut builder = QueryBuilder::<MySql>::new("UPDATE sys_dept SET ");
        let mut separated = builder.separated(", ");
        let mut has_update = false;

        if let Some(parent_id) = dto.parent_id {
            let ancestors = self.resolve_ancestors(parent_id).await?;
            separated.push("parent_id = ").push_bind(parent_id);
            separated.push("ancestors = ").push_bind(ancestors);
            has_update = true;
        }
        if let Some(name) = dto.name {
            separated.push("dept_name = ").push_bind(name);
            has_update = true;
        }
        if let Some(leader) = dto.leader {
            separated.push("leader = ").push_bind(leader);
            has_update = true;
        }
        if let Some(phone) = dto.phone {
            separated.push("phone = ").push_bind(phone);
            has_update = true;
        }
        if let Some(status) = dto.status {
            let status_value = if matches!(status.as_str(), "disabled" | "0") {
                0_i16
            } else {
                1_i16
            };
            separated.push("status = ").push_bind(status_value);
            has_update = true;
        }

        if !has_update {
            return Err(AppError::bad_request("没有可更新的字段"));
        }

        separated.push("updated_by = 1");
        builder
            .push(" WHERE id = ")
            .push_bind(id)
            .push(" AND is_deleted = 0");

        let result = builder
            .build()
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
    async fn list(&self, query: SysDeptListQueryDto) -> Result<Vec<SysDeptModel>, AppError> {
        self.list(query).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysDeptModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysDeptModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, dto: SysDeptUpdateReqDto) -> Result<bool, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}
