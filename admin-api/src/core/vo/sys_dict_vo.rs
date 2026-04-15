use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SysDictVo {
    pub id: u64,
    #[serde(rename = "type")]
    pub dict_type: String,
    pub label: String,
    pub value: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysDictListVo {
    pub total: usize,
    pub items: Vec<SysDictVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysDictRecordVo {
    pub item: SysDictVo,
}

#[derive(Debug, Clone, Serialize)]
pub struct SysDictDeleteVo {
    pub id: u64,
    pub deleted: bool,
}
