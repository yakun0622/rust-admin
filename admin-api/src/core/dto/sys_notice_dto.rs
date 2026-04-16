use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SysNoticeListQueryDto {
    pub title: Option<String>,
    pub notice_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysNoticeCreateReqDto {
    pub title: String,
    #[serde(rename = "type")]
    pub notice_type: Option<String>,
    pub status: Option<String>,
    pub publisher: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysNoticeUpdateReqDto {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub notice_type: Option<String>,
    pub status: Option<String>,
    pub publisher: Option<String>,
}
