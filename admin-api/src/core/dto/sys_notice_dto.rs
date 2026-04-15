use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SysNoticeListQueryDto {
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysNoticeCreateReqDto {
    pub title: String,
    #[serde(rename = "type")]
    pub notice_type: Option<String>,
    pub status: Option<String>,
    pub publisher: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysNoticeUpdateReqDto {
    pub title: String,
    #[serde(rename = "type")]
    pub notice_type: Option<String>,
    pub status: Option<String>,
    pub publisher: Option<String>,
}
