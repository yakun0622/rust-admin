use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct OperLogItemVo {
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

#[derive(Debug, Clone, Serialize)]
pub struct OperLogListVo {
    pub total: usize,
    pub items: Vec<OperLogItemVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginLogItemVo {
    pub id: u64,
    pub username: String,
    pub login_type: String,
    pub ip: String,
    pub location: String,
    pub status: String,
    pub message: String,
    pub login_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginLogListVo {
    pub total: usize,
    pub items: Vec<LoginLogItemVo>,
}
