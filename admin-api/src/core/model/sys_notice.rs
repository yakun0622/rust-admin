use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SysNoticeModel {
    pub id: u64,
    pub title: String,
    pub notice_type: i16,
    pub status: i16,
    pub publisher: String,
}

impl<'r> FromRow<'r, MySqlRow> for SysNoticeModel {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            notice_type: row.try_get("notice_type")?,
            status: row.try_get("status")?,
            publisher: row.try_get("publisher")?,
        })
    }
}
