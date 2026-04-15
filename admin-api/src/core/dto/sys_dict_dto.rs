use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SysDictListQueryDto {
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysDictCreateReqDto {
    #[serde(rename = "type")]
    pub dict_type: String,
    pub label: String,
    pub value: String,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysDictUpdateReqDto {
    #[serde(rename = "type")]
    pub dict_type: String,
    pub label: String,
    pub value: String,
    pub status: Option<String>,
}
