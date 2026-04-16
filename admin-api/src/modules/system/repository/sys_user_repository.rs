use crate::core::dbal::query::fragments;
use crate::core::dto::sys_user_dto::{SysUserListQueryDto, SysUserUpdateReqDto};
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::core::{errors::AppError, model::sys_user::SysUserModel};

#[async_trait]
pub trait ISysUserRepository: Interface {
    async fn list(&self, query: SysUserListQueryDto) -> Result<Vec<SysUserModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysUserModel>, AppError>;
    async fn insert(&self, model: &SysUserModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysUserUpdateReqDto) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysUserRepository)]
pub(crate) struct SysUserRepository {
    pool: MySqlPool,
}

impl SysUserRepository {
    pub(crate) async fn list(
        &self,
        query: SysUserListQueryDto,
    ) -> Result<Vec<SysUserModel>, AppError> {
        let (username_kw, username_like) = fragments::keyword_args(query.username.as_deref());
        let (nickname_kw, nickname_like) = fragments::keyword_args(query.nickname.as_deref());
        let (phone_kw, phone_like) = fragments::keyword_args(query.phone.as_deref());
        let status = query.status.filter(|s| !s.trim().is_empty());

        sqlx::query_as::<_, SysUserModel>(
            r#"
            SELECT id, username, nickname, phone, status
            FROM sys_user
            WHERE is_deleted = 0
              AND (? = '' OR username LIKE ?)
              AND (? = '' OR nickname LIKE ?)
              AND (? = '' OR phone LIKE ?)
              AND (? IS NULL OR status = ?)
            ORDER BY id DESC
            "#,
        )
        .bind(&username_kw)
        .bind(&username_like)
        .bind(&nickname_kw)
        .bind(&nickname_like)
        .bind(&phone_kw)
        .bind(&phone_like)
        .bind(&status)
        .bind(&status)
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
        dto: SysUserUpdateReqDto,
    ) -> Result<bool, AppError> {
        let mut builder = QueryBuilder::<MySql>::new("UPDATE sys_user SET ");
        let mut separated = builder.separated(", ");
        let mut has_update = false;

        if let Some(username) = dto.username {
            separated.push("username = ").push_bind(username);
            has_update = true;
        }
        if let Some(nickname) = dto.nickname {
            separated.push("nickname = ").push_bind(nickname);
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
    async fn list(&self, query: SysUserListQueryDto) -> Result<Vec<SysUserModel>, AppError> {
        self.list(query).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysUserModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysUserModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, dto: SysUserUpdateReqDto) -> Result<bool, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}
