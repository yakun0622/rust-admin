use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SysUserModel {
    pub id: u64,
    pub username: String,
    pub nickname: String,
    pub phone: Option<String>,
    pub status: i16,
    pub password_hash: String,
    pub created_by: i64,
    pub updated_by: i64,
    pub is_deleted: i16,
}

impl<'r> FromRow<'r, MySqlRow> for SysUserModel {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            nickname: row.try_get("nickname")?,
            phone: row.try_get("phone")?,
            status: row.try_get("status")?,
            password_hash: String::new(),
            created_by: 0,
            updated_by: 0,
            is_deleted: 0,
        })
    }
}
