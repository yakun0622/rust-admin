use crate::core::{
    dto::monitor_dto::JobUpsertReqDto, model::monitor::JobPo, utils::now_timestamp_millis,
};

use super::{
    filter_with_keyword, normalize_job_status, InMemoryMonitorRepository, JOB_INTERVAL_MS,
};

impl InMemoryMonitorRepository {
    pub async fn list_jobs(&self, keyword: Option<&str>) -> Vec<JobPo> {
        let jobs = self.jobs.read().await;
        filter_with_keyword(keyword, jobs.as_slice(), |item| {
            format!(
                "{} {} {} {} {} {}",
                item.job_name,
                item.job_group,
                item.invoke_target,
                item.cron_expression,
                item.status,
                item.remark
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
