use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SysPostVo {
    pub id: u64,
    pub name: String,
    pub code: String,
    pub sort: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysPostListVo {
    pub total: usize,
    pub items: Vec<SysPostVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysPostRecordVo {
    pub item: SysPostVo,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysPostDeleteVo {
    pub id: u64,
    pub deleted: bool,
}
