use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SysDictModel {
    pub id: u64,
    pub dict_type: String,
    pub label: String,
    pub value: String,
    pub status: i16,
}

impl<'r> FromRow<'r, MySqlRow> for SysDictModel {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            dict_type: row.try_get("dict_type")?,
            label: row.try_get("label")?,
            value: row.try_get("value")?,
            status: row.try_get("status")?,
        })
    }
}
