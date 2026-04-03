use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{core::{errors::AppError, model::auth::UserCredentialPo}};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_by_username(&self, username: &str) -> Result<Option<UserCredentialPo>, AppError>;
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
}
