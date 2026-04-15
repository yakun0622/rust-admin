use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SysRoleListQueryDto {
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysRoleCreateReqDto {
    pub name: String,
    pub key: String,
    pub sort: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysRoleUpdateReqDto {
    pub name: String,
    pub key: String,
    pub sort: Option<i32>,
    pub status: Option<String>,
}
