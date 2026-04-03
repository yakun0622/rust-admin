use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserCredentialPo {
    pub id: u64,
    pub username: String,
    pub nickname: String,
    pub password_hash: String,
    pub status: i8,
}
