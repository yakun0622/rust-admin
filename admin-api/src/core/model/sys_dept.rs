use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SysDeptModel {
    pub id: u64,
    pub parent_id: u64,
    pub dept_name: String,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub status: i16,
}

impl<'r> FromRow<'r, MySqlRow> for SysDeptModel {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            parent_id: row.try_get("parent_id")?,
            dept_name: row.try_get("dept_name")?,
            leader: row.try_get("leader")?,
            phone: row.try_get("phone")?,
            status: row.try_get("status")?,
        })
    }
}
