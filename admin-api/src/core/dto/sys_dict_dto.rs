use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SysDictListQueryDto {
    pub dict_type: Option<String>,
    pub dict_label: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysDictCreateReqDto {
    #[serde(rename = "type")]
    pub dict_type: String,
    pub label: String,
    pub value: String,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysDictUpdateReqDto {
    #[serde(rename = "type")]
    pub dict_type: Option<String>,
    pub label: Option<String>,
    pub value: Option<String>,
    pub status: Option<String>,
}
