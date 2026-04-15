use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SysUserListQueryDto {
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysUserCreateReqDto {
    pub username: String,
    pub nickname: String,
    pub phone: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysUserUpdateReqDto {
    pub username: String,
    pub nickname: String,
    pub phone: Option<String>,
    pub status: Option<String>,
}
