use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SysMenuVo {
    pub id: u64,
    pub parent_id: u64,
    pub menu_type: i16,
    pub name: String,
    pub route_name: String,
    pub path: String,
    pub component: String,
    pub permission: String,
    pub icon: String,
    pub order_num: i32,
    pub status: String,
    pub visible: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<SysMenuVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysMenuListVo {
    pub total: usize,
    pub items: Vec<SysMenuVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysMenuRecordVo {
    pub item: SysMenuVo,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysMenuDeleteVo {
    pub id: u64,
    pub deleted: bool,
}
