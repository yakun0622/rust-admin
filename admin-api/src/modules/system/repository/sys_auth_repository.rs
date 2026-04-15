use async_trait::async_trait;
use shaku::Component;
use sqlx::{query_as, MySqlPool};

use crate::core::{errors::AppError, model::auth::UserCredentialPo};

use super::interface::ISysAuthRepository;

#[derive(Component, Clone)]
#[shaku(interface = ISysAuthRepository)]
pub struct SysAuthRepository {
    pool: MySqlPool,
}

impl SysAuthRepository {
    pub(crate) async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<UserCredentialPo>, AppError> {
        query_as::<_, UserCredentialPo>(
            r#"
            SELECT id, username, nickname, password_hash, status
            FROM sys_user
            WHERE username = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询用户失败: {err}")))
    }

    pub(crate) async fn append_login_log(
        &self,
        username: Option<&str>,
        login_type: i8,
        status: i8,
        message: &str,
        ip: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO sys_login_log (username, login_type, ip, status, message)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(username.unwrap_or_default())
        .bind(login_type)
        .bind(ip)
        .bind(status)
        .bind(message)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("写入登录日志失败: {err}")))?;
        Ok(())
    }
}

#[async_trait]
impl ISysAuthRepository for SysAuthRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<UserCredentialPo>, AppError> {
        self.find_by_username(username).await
    }

    async fn append_login_log(
        &self,
        username: Option<&str>,
        login_type: i8,
        status: i8,
        message: &str,
        ip: &str,
    ) -> Result<(), AppError> {
        self.append_login_log(username, login_type, status, message, ip)
            .await
    }
}
