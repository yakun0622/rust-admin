use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperLogPo {
    pub id: u64,
    pub module: String,
    pub business_type: String,
    pub request_method: String,
    pub oper_name: String,
    pub ip: String,
    pub status: String,
    pub duration_ms: u32,
    pub oper_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginLogPo {
    pub id: u64,
    pub username: String,
    pub login_type: String,
    pub ip: String,
    pub status: String,
    pub message: String,
    pub login_at: i64,
}
