use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct OnlineUserItemVo {
    pub id: u64,
    pub username: String,
    pub ip: String,
    pub browser: String,
    pub os: String,
    pub login_at: i64,
    pub last_active_at: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OnlineUserListVo {
    pub total: usize,
    pub items: Vec<OnlineUserItemVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JobItemVo {
    pub id: u64,
    pub job_name: String,
    pub job_group: String,
    pub invoke_target: String,
    pub cron_expression: String,
    pub status: String,
    pub remark: String,
    pub last_run_at: Option<i64>,
    pub next_run_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JobListVo {
    pub total: usize,
    pub items: Vec<JobItemVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JobActionVo {
    pub id: u64,
    pub status: String,
    pub last_run_at: Option<i64>,
    pub next_run_at: Option<i64>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DatasourceOverviewVo {
    pub database: String,
    pub mysql_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub ping_ok: bool,
    pub ping_message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ServerOverviewVo {
    pub app_name: String,
    pub env: String,
    pub uptime_secs: u64,
    pub mysql_ok: bool,
    pub redis_ok: bool,
    pub now_millis: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheKeyItemVo {
    pub key: String,
    pub data_type: String,
    pub ttl_secs: i64,
    pub sample: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheSearchVo {
    pub pattern: String,
    pub total: usize,
    pub items: Vec<CacheKeyItemVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheNamespaceItemVo {
    pub namespace: String,
    pub key_count: u64,
    pub example_key: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheNamespaceListVo {
    pub total: usize,
    pub items: Vec<CacheNamespaceItemVo>,
}
