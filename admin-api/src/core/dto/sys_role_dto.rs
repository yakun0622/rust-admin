use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SysRoleListQueryDto {
    pub name: Option<String>,
    pub key: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysRoleCreateReqDto {
    pub name: String,
    pub key: String,
    pub sort: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysRoleUpdateReqDto {
    pub name: Option<String>,
    pub key: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<String>,
}
