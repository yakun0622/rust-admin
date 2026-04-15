use crate::core::dbal::query::fragments;
use async_trait::async_trait;
use shaku::Component;
use sqlx::MySqlPool;

use crate::core::{errors::AppError, model::sys_role::SysRoleModel};

use super::interface::ISysRoleRepository;

#[derive(Component, Clone)]
#[shaku(interface = ISysRoleRepository)]
pub(crate) struct SysRoleRepository {
    pool: MySqlPool,
}

impl SysRoleRepository {
    pub(crate) async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysRoleModel>, AppError> {
        let (kw, like) = fragments::keyword_args(keyword);
        sqlx::query_as::<_, SysRoleModel>(
            r#"
            SELECT id, role_name, role_key, role_sort, status
            FROM sys_role
            WHERE is_deleted = 0
              AND (? = '' OR role_name LIKE ? OR role_key LIKE ?)
            ORDER BY id DESC
            "#,
        )
        .bind(&kw)
        .bind(&like)
        .bind(&like)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询角色失败: {err}")))
    }

    pub(crate) async fn get_by_id(&self, id: u64) -> Result<Option<SysRoleModel>, AppError> {
        sqlx::query_as::<_, SysRoleModel>(
            r#"
            SELECT id, role_name, role_key, role_sort, status
            FROM sys_role
            WHERE id = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询角色失败: {err}")))
    }

    pub(crate) async fn insert(&self, model: &SysRoleModel) -> Result<u64, AppError> {
        let result = sqlx::query(
            r#"
            INSERT INTO sys_role (
                role_name, role_key, role_sort, status, data_scope, created_by, updated_by, is_deleted
            ) VALUES (?, ?, ?, ?, 5, 1, 1, 0)
            "#,
        )
        .bind(&model.role_name)
        .bind(&model.role_key)
        .bind(model.role_sort)
        .bind(model.status)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增角色失败: {err}")))?;

        Ok(result.last_insert_id())
    }

    pub(crate) async fn update_by_id(
        &self,
        id: u64,
        model: &SysRoleModel,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_role
            SET role_name = ?, role_key = ?, role_sort = ?, status = ?, updated_by = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(&model.role_name)
        .bind(&model.role_key)
        .bind(model.role_sort)
        .bind(model.status)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新角色失败: {err}")))?;

        Ok(result.rows_affected() > 0)
    }

    pub(crate) async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_role
            SET is_deleted = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("删除角色失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }
}

#[async_trait]
impl ISysRoleRepository for SysRoleRepository {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysRoleModel>, AppError> {
        self.list(keyword).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysRoleModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysRoleModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, model: &SysRoleModel) -> Result<bool, AppError> {
        self.update_by_id(id, model).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}
