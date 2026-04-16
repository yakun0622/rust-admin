use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SysDeptListQueryDto {
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysDeptCreateReqDto {
    pub parent_id: u64,
    pub name: String,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysDeptUpdateReqDto {
    pub parent_id: Option<u64>,
    pub name: Option<String>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
}
