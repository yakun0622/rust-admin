use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SysRoleModel {
    pub id: u64,
    pub role_name: String,
    pub role_key: String,
    pub role_sort: i32,
    pub status: i16,
}

impl<'r> FromRow<'r, MySqlRow> for SysRoleModel {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            role_name: row.try_get("role_name")?,
            role_key: row.try_get("role_key")?,
            role_sort: row.try_get("role_sort")?,
            status: row.try_get("status")?,
        })
    }
}
