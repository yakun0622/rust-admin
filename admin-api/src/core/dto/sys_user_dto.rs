use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SysUserListQueryDto {
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysUserCreateReqDto {
    pub username: String,
    pub nickname: String,
    pub phone: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysUserUpdateReqDto {
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
}
