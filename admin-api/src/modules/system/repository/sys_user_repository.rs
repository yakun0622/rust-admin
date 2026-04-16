use crate::core::dbal::query::fragments;
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::MySqlPool;

use crate::core::{errors::AppError, model::sys_user::SysUserModel};

#[async_trait]
pub trait ISysUserRepository: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysUserModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysUserModel>, AppError>;
    async fn insert(&self, model: &SysUserModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, model: &SysUserModel) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysUserRepository)]
pub(crate) struct SysUserRepository {
    pool: MySqlPool,
}

impl SysUserRepository {
    pub(crate) async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysUserModel>, AppError> {
        let (kw, like) = fragments::keyword_args(keyword);
        sqlx::query_as::<_, SysUserModel>(
            r#"
            SELECT id, username, nickname, phone, status
            FROM sys_user
            WHERE is_deleted = 0
              AND (? = '' OR username LIKE ? OR nickname LIKE ? OR phone LIKE ?)
            ORDER BY id DESC
            "#,
        )
        .bind(&kw)
        .bind(&like)
        .bind(&like)
        .bind(&like)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询用户失败: {err}")))
    }

    pub(crate) async fn get_by_id(&self, id: u64) -> Result<Option<SysUserModel>, AppError> {
        sqlx::query_as::<_, SysUserModel>(
            r#"
            SELECT id, username, nickname, phone, status
            FROM sys_user
            WHERE id = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询用户失败: {err}")))
    }

    pub(crate) async fn insert(&self, model: &SysUserModel) -> Result<u64, AppError> {
        let result = sqlx::query(
            r#"
            INSERT INTO sys_user (
                username, nickname, phone, status, password_hash, created_by, updated_by, is_deleted
            ) VALUES (?, ?, ?, ?, ?, ?, ?, 0)
            "#,
        )
        .bind(&model.username)
        .bind(&model.nickname)
        .bind(model.phone.as_deref())
        .bind(model.status)
        .bind(&model.password_hash)
        .bind(model.created_by)
        .bind(model.updated_by)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增用户失败: {err}")))?;

        Ok(result.last_insert_id())
    }

    pub(crate) async fn update_by_id(
        &self,
        id: u64,
        model: &SysUserModel,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_user
            SET username = ?, nickname = ?, phone = ?, status = ?, updated_by = ?
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(&model.username)
        .bind(&model.nickname)
        .bind(model.phone.as_deref())
        .bind(model.status)
        .bind(model.updated_by)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新用户失败: {err}")))?;

        Ok(result.rows_affected() > 0)
    }

    pub(crate) async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_user
            SET is_deleted = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("删除用户失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }
}

#[async_trait]
impl ISysUserRepository for SysUserRepository {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysUserModel>, AppError> {
        self.list(keyword).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysUserModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysUserModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, model: &SysUserModel) -> Result<bool, AppError> {
        self.update_by_id(id, model).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}
