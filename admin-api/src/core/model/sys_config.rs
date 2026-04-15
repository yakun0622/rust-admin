use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SysConfigModel {
    pub id: u64,
    pub config_name: String,
    pub config_key: String,
    pub config_value: String,
    pub remark: Option<String>,
    pub status: i16,
}

impl<'r> FromRow<'r, MySqlRow> for SysConfigModel {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            config_name: row.try_get("config_name")?,
            config_key: row.try_get("config_key")?,
            config_value: row.try_get("config_value")?,
            remark: row.try_get("remark")?,
            status: row.try_get("status")?,
        })
    }
}
