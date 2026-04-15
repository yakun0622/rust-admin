use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SysRoleVo {
    pub id: u64,
    pub name: String,
    pub key: String,
    pub sort: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysRoleListVo {
    pub total: usize,
    pub items: Vec<SysRoleVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysRoleRecordVo {
    pub item: SysRoleVo,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysRoleDeleteVo {
    pub id: u64,
    pub deleted: bool,
}
