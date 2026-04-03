use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    core::{dto::monitor::JobUpsertReqDto, model::monitor::{JobPo, OnlineUserPo}, utils::now_timestamp_millis},
};

const JOB_INTERVAL_MS: i64 = 60_000;

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

    pub async fn list_online_users(&self, keyword: Option<&str>) -> Vec<OnlineUserPo> {
        let users = self.online_users.read().await;
        filter_with_keyword(keyword, users.as_slice(), |item| {
            format!("{} {} {} {} {}", item.username, item.ip, item.browser, item.os, item.status)
        })
    }

    pub async fn list_jobs(&self, keyword: Option<&str>) -> Vec<JobPo> {
        let jobs = self.jobs.read().await;
        filter_with_keyword(keyword, jobs.as_slice(), |item| {
            format!(
                "{} {} {} {} {} {}",
                item.job_name, item.job_group, item.invoke_target, item.cron_expression, item.status, item.remark
            )
        })
    }

    pub async fn create_job(&self, payload: JobUpsertReqDto) -> JobPo {
        let mut jobs = self.jobs.write().await;
        let id = jobs.iter().map(|job| job.id).max().unwrap_or(0) + 1;
        let now = now_timestamp_millis();
        let status = normalize_job_status(payload.status.as_deref());

        let created = JobPo {
            id,
            job_name: payload.job_name,
            job_group: payload.job_group,
            invoke_target: payload.invoke_target,
            cron_expression: payload.cron_expression,
            status: status.to_string(),
            remark: payload.remark.unwrap_or_default(),
            last_run_at: None,
            next_run_at: if status == "running" {
                Some(now + JOB_INTERVAL_MS)
            } else {
                None
            },
        };
        jobs.push(created.clone());
        created
    }

    pub async fn update_job(&self, id: u64, payload: JobUpsertReqDto) -> Option<JobPo> {
        let mut jobs = self.jobs.write().await;
        let now = now_timestamp_millis();
        let target = jobs.iter_mut().find(|job| job.id == id)?;
        let status = normalize_job_status(payload.status.as_deref());

        target.job_name = payload.job_name;
        target.job_group = payload.job_group;
        target.invoke_target = payload.invoke_target;
        target.cron_expression = payload.cron_expression;
        target.status = status.to_string();
        target.remark = payload.remark.unwrap_or_default();
        target.next_run_at = if status == "running" {
            Some(target.next_run_at.unwrap_or(now + JOB_INTERVAL_MS))
        } else {
            None
        };
        Some(target.clone())
    }

    pub async fn delete_job(&self, id: u64) -> bool {
        let mut jobs = self.jobs.write().await;
        let before = jobs.len();
        jobs.retain(|job| job.id != id);
        before != jobs.len()
    }

    pub async fn run_job_once(&self, id: u64) -> Option<JobPo> {
        let mut jobs = self.jobs.write().await;
        let now = now_timestamp_millis();
        let target = jobs.iter_mut().find(|job| job.id == id)?;
        target.last_run_at = Some(now);
        target.next_run_at = if target.status == "running" {
            Some(now + JOB_INTERVAL_MS)
        } else {
            target.next_run_at
        };
        Some(target.clone())
    }

    pub async fn set_job_status(&self, id: u64, status: &str) -> Option<JobPo> {
        let mut jobs = self.jobs.write().await;
        let now = now_timestamp_millis();
        let normalized = normalize_job_status(Some(status)).to_string();
        let target = jobs.iter_mut().find(|job| job.id == id)?;
        target.status = normalized.clone();
        target.next_run_at = if normalized == "running" {
            Some(now + JOB_INTERVAL_MS)
        } else {
            None
        };
        Some(target.clone())
    }

    pub async fn tick_scheduler(&self) {
        let now = now_timestamp_millis();
        let mut jobs = self.jobs.write().await;
        for job in jobs.iter_mut() {
            if job.status != "running" {
                continue;
            }

            if job.next_run_at.is_some_and(|next| next > now) {
                continue;
            }

            job.last_run_at = Some(now);
            job.next_run_at = Some(now + JOB_INTERVAL_MS);
        }
    }
}

fn normalize_job_status(status: Option<&str>) -> &'static str {
    match status.map(|value| value.trim().to_lowercase()) {
        Some(value) if value == "paused" => "paused",
        _ => "running",
    }
}

fn filter_with_keyword<T: Clone>(
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
