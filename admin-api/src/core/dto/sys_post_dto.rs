use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SysPostListQueryDto {
    pub name: Option<String>,
    pub code: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysPostCreateReqDto {
    pub name: String,
    pub code: String,
    pub sort: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysPostUpdateReqDto {
    pub name: Option<String>,
    pub code: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<String>,
}
