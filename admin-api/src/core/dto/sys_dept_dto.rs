use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SysDeptListQueryDto {
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysDeptCreateReqDto {
    pub parent_id: u64,
    pub name: String,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysDeptUpdateReqDto {
    pub parent_id: Option<u64>,
    pub name: String,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
}
