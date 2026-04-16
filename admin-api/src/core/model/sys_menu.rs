use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SysMenuModel {
    pub id: u64,
    pub parent_id: u64,
    pub menu_type: i16,
    pub menu_name: String,
    pub route_name: Option<String>,
    pub route_path: Option<String>,
    pub component_path: Option<String>,
    pub perms: Option<String>,
    pub permission: Option<String>,
    pub icon: Option<String>,
    pub order_num: i32,
    pub is_visible: i16,
    pub status: i16,
}

impl<'r> FromRow<'r, MySqlRow> for SysMenuModel {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            parent_id: row.try_get("parent_id")?,
            menu_type: row.try_get("menu_type")?,
            menu_name: row.try_get("menu_name")?,
            route_name: row.try_get("route_name")?,
            route_path: row.try_get("route_path")?,
            component_path: row.try_get("component_path")?,
            perms: row.try_get("perms")?,
            permission: row.try_get("permission")?,
            icon: row.try_get("icon")?,
            order_num: row.try_get("order_num")?,
            is_visible: row.try_get("is_visible")?,
            status: row.try_get("status")?,
        })
    }
}
