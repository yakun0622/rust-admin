use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SysPostModel {
    pub id: u64,
    pub post_name: String,
    pub post_code: String,
    pub post_sort: i32,
    pub status: i16,
}

impl<'r> FromRow<'r, MySqlRow> for SysPostModel {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            post_name: row.try_get("post_name")?,
            post_code: row.try_get("post_code")?,
            post_sort: row.try_get("post_sort")?,
            status: row.try_get("status")?,
        })
    }
}
