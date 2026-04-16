use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperLogPo {
    pub id: u64,
    pub module: String,
    pub business_type: String,
    pub request_method: String,
    pub oper_name: String,
    pub ip: String,
    pub location: String,
    pub status: String,
    pub duration_ms: u32,
    pub oper_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperLogCreatePo {
    pub module: String,
    pub business_type: i8,
    pub method: Option<String>,
    pub request_method: Option<String>,
    pub operator_type: i8,
    pub oper_name: Option<String>,
    pub dept_name: Option<String>,
    pub url: Option<String>,
    pub ip: Option<String>,
    pub location: Option<String>,
    pub request_params: Option<String>,
    pub response_data: Option<String>,
    pub status: i8,
    pub error_msg: Option<String>,
    pub user_agent: Option<String>,
    pub os: Option<String>,
    pub duration_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginLogPo {
    pub id: u64,
    pub username: String,
    pub login_type: String,
    pub ip: String,
    pub location: String,
    pub status: String,
    pub message: String,
    pub login_at: i64,
}
