mod job_repository;
mod online_repository;

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::core::{
    model::monitor::{JobPo, OnlineUserPo},
    utils::now_timestamp_millis,
};

pub(super) const JOB_INTERVAL_MS: i64 = 60_000;

#[derive(Debug, Default)]
pub struct InMemoryMonitorRepository {
    online_users: RwLock<Vec<OnlineUserPo>>,
    jobs: RwLock<Vec<JobPo>>,
}

impl InMemoryMonitorRepository {
    pub fn seeded() -> Arc<Self> {
        let now = now_timestamp_millis();
        Arc::new(Self {
            online_users: RwLock::new(vec![
                OnlineUserPo {
                    id: 1,
                    username: "admin".to_string(),
                    ip: "127.0.0.1".to_string(),
                    browser: "Chrome 123".to_string(),
                    os: "macOS".to_string(),
                    login_at: now - 30 * 60 * 1000,
                    last_active_at: now - 30 * 1000,
                    status: "online".to_string(),
                },
                OnlineUserPo {
                    id: 2,
                    username: "ops".to_string(),
                    ip: "10.0.0.12".to_string(),
                    browser: "Edge 122".to_string(),
                    os: "Windows".to_string(),
                    login_at: now - 120 * 60 * 1000,
                    last_active_at: now - 8 * 60 * 1000,
                    status: "online".to_string(),
                },
            ]),
            jobs: RwLock::new(vec![
                JobPo {
                    id: 1,
                    job_name: "清理登录日志".to_string(),
                    job_group: "system".to_string(),
                    invoke_target: "log.cleanup_login".to_string(),
                    cron_expression: "0 */5 * * * *".to_string(),
                    status: "running".to_string(),
                    remark: "每5分钟执行一次".to_string(),
                    last_run_at: Some(now - 2 * 60 * 1000),
                    next_run_at: Some(now + 3 * 60 * 1000),
                },
                JobPo {
                    id: 2,
                    job_name: "同步缓存统计".to_string(),
                    job_group: "monitor".to_string(),
                    invoke_target: "monitor.cache.sync".to_string(),
                    cron_expression: "0 */10 * * * *".to_string(),
                    status: "paused".to_string(),
                    remark: "手动暂停".to_string(),
                    last_run_at: Some(now - 30 * 60 * 1000),
                    next_run_at: None,
                },
            ]),
        })
    }
}

pub(super) fn normalize_job_status(status: Option<&str>) -> &'static str {
    match status.map(|value| value.trim().to_lowercase()) {
        Some(value) if value == "paused" => "paused",
        _ => "running",
    }
}

pub(super) fn filter_with_keyword<T: Clone>(
    keyword: Option<&str>,
    data: &[T],
    to_searchable: impl Fn(&T) -> String,
) -> Vec<T> {
    let normalized = keyword
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_lowercase);

    data.iter()
        .filter(|item| {
            if let Some(ref kw) = normalized {
                to_searchable(item).to_lowercase().contains(kw)
            } else {
                true
            }
        })
        .cloned()
        .collect()
}
