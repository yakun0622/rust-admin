use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{MySqlPool, PgPool, Row};

use crate::core::{errors::AppError, model::auth::UserCredentialPo};

#[derive(Debug, Clone)]
pub struct LoginAuditRecord {
    pub username: Option<String>,
    pub login_type: i8,
    pub status: i8,
    pub message: String,
    pub ip: String,
}

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_by_username(&self, username: &str) -> Result<Option<UserCredentialPo>, AppError>;
    async fn append_login_log(&self, record: LoginAuditRecord) -> Result<(), AppError>;
}

#[derive(Debug, Clone)]
pub struct MySqlAuthRepository {
    pool: MySqlPool,
}

impl MySqlAuthRepository {
    pub fn new(pool: MySqlPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }
}

#[async_trait]
impl AuthRepository for MySqlAuthRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<UserCredentialPo>, AppError> {
        sqlx::query_as::<_, UserCredentialPo>(
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

    async fn append_login_log(&self, record: LoginAuditRecord) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO sys_login_log (username, login_type, ip, status, message)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(record.username.unwrap_or_default())
        .bind(record.login_type)
        .bind(record.ip)
        .bind(record.status)
        .bind(record.message)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("写入登录日志失败: {err}")))?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PostgresAuthRepository {
    pool: PgPool,
}

impl PostgresAuthRepository {
    pub fn new(pool: PgPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }
}

#[async_trait]
impl AuthRepository for PostgresAuthRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<UserCredentialPo>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, nickname, password_hash, status
            FROM sys_user
            WHERE username = $1 AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询用户失败: {err}")))?;

        Ok(row.map(|row| UserCredentialPo {
            id: row.get::<i64, _>("id") as u64,
            username: row.get::<String, _>("username"),
            nickname: row.get::<String, _>("nickname"),
            password_hash: row.get::<String, _>("password_hash"),
            status: row.get::<i16, _>("status") as i8,
        }))
    }

    async fn append_login_log(&self, record: LoginAuditRecord) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO sys_login_log (username, login_type, ip, status, message)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(record.username.unwrap_or_default())
        .bind(record.login_type as i16)
        .bind(record.ip)
        .bind(record.status as i16)
        .bind(record.message)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("写入登录日志失败: {err}")))?;
        Ok(())
    }
}
