use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct SystemCrudListVo {
    pub total: usize,
    pub items: Vec<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemCrudRecordVo {
    pub item: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemCrudDeleteVo {
    pub id: u64,
    pub deleted: bool,
}
