use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SysUserVo {
    pub id: u64,
    pub username: String,
    pub nickname: String,
    pub phone: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysUserListVo {
    pub total: usize,
    pub items: Vec<SysUserVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysUserRecordVo {
    pub item: SysUserVo,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysUserDeleteVo {
    pub id: u64,
    pub deleted: bool,
}
