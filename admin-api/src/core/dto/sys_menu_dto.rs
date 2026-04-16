use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SysMenuListQueryDto {
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysMenuCreateReqDto {
    pub parent_id: u64,
    pub menu_type: Option<i16>,
    pub name: String,
    pub route_name: Option<String>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub permission: Option<String>,
    pub icon: Option<String>,
    pub order_num: Option<i32>,
    pub status: Option<String>,
    pub visible: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysMenuUpdateReqDto {
    pub parent_id: Option<u64>,
    pub menu_type: Option<i16>,
    pub name: Option<String>,
    pub route_name: Option<String>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub permission: Option<String>,
    pub icon: Option<String>,
    pub order_num: Option<i32>,
    pub status: Option<String>,
    pub visible: Option<String>,
}
