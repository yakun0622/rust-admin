use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SysMenuVo {
    pub id: u64,
    pub parent_id: u64,
    pub name: String,
    pub path: String,
    pub component: String,
    pub visible: String,
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
