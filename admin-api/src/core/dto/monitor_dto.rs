use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MonitorListQueryDto {
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct JobLogQueryDto {
    pub keyword: Option<String>,
    pub job_id: Option<u64>,
    pub status: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CacheSearchQueryDto {
    pub keyword: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobUpsertReqDto {
    pub job_name: String,
    pub job_group: String,
    pub invoke_target: String,
    pub cron_expression: String,
    pub concurrent: Option<bool>,
    pub status: Option<String>,
    pub remark: Option<String>,
}
