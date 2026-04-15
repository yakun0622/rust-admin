use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SysConfigVo {
    pub id: u64,
    pub name: String,
    pub value: String,
    pub remark: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysConfigListVo {
    pub total: usize,
    pub items: Vec<SysConfigVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysConfigRecordVo {
    pub item: SysConfigVo,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysConfigDeleteVo {
    pub id: u64,
    pub deleted: bool,
}
