use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SysMenuListQueryDto {
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysMenuCreateReqDto {
    pub parent_id: u64,
    pub name: String,
    pub path: String,
    pub component: String,
    pub visible: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysMenuUpdateReqDto {
    pub parent_id: Option<u64>,
    pub name: String,
    pub path: String,
    pub component: String,
    pub visible: Option<String>,
}
