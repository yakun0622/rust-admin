use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SysMenuModel {
    pub id: u64,
    pub parent_id: u64,
    pub menu_name: String,
    pub route_path: Option<String>,
    pub component_path: Option<String>,
    pub is_visible: i16,
}

impl<'r> FromRow<'r, MySqlRow> for SysMenuModel {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            parent_id: row.try_get("parent_id")?,
            menu_name: row.try_get("menu_name")?,
            route_path: row.try_get("route_path")?,
            component_path: row.try_get("component_path")?,
            is_visible: row.try_get("is_visible")?,
        })
    }
}
