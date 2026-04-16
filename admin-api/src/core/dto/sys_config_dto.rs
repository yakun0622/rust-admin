use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SysConfigListQueryDto {
    pub name: Option<String>,
    pub key: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysConfigCreateReqDto {
    pub name: String,
    pub value: String,
    pub remark: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysConfigUpdateReqDto {
    pub name: Option<String>,
    pub value: Option<String>,
    pub remark: Option<String>,
    pub status: Option<String>,
}
