use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineUserPo {
    pub id: u64,
    pub username: String,
    pub ip: String,
    pub browser: String,
    pub os: String,
    pub login_at: i64,
    pub last_active_at: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPo {
    pub id: u64,
    pub job_name: String,
    pub job_group: String,
    pub invoke_target: String,
    pub cron_expression: String,
    pub concurrent: bool,
    pub status: String,
    pub remark: String,
    pub last_run_at: Option<i64>,
    pub next_run_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobLogPo {
    pub id: u64,
    pub job_id: u64,
    pub job_name: String,
    pub job_group: String,
    pub invoke_target: String,
    pub cron_expression: String,
    pub status: String,
    pub message: String,
    pub exception_info: Option<String>,
    pub trigger_type: String,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub duration_ms: u32,
}
