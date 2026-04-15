use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SysDeptVo {
    pub id: u64,
    pub parent_id: u64,
    pub name: String,
    pub leader: String,
    pub phone: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysDeptListVo {
    pub total: usize,
    pub items: Vec<SysDeptVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysDeptRecordVo {
    pub item: SysDeptVo,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysDeptDeleteVo {
    pub id: u64,
    pub deleted: bool,
}
