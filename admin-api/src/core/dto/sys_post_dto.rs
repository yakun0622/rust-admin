use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SysPostListQueryDto {
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysPostCreateReqDto {
    pub name: String,
    pub code: String,
    pub sort: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysPostUpdateReqDto {
    pub name: String,
    pub code: String,
    pub sort: Option<i32>,
    pub status: Option<String>,
}
