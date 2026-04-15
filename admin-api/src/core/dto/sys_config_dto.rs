use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SysConfigListQueryDto {
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysConfigCreateReqDto {
    pub name: String,
    pub value: String,
    pub remark: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysConfigUpdateReqDto {
    pub name: String,
    pub value: String,
    pub remark: Option<String>,
    pub status: Option<String>,
}
