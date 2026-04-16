use std::sync::Arc;

use crate::core::{
    dto::monitor_dto::{JobLogQueryDto, JobUpsertReqDto},
    errors::AppError,
    model::monitor::{JobLogPo, JobPo},
    vo::monitor_vo::{JobActionVo, JobItemVo, JobListVo, JobLogItemVo, JobLogListVo},
};
use crate::modules::system::{
    repository::SysJobRepository,
    scheduler::{SchedulerManager, TriggerType},
};

#[derive(Clone)]
pub struct SysJobService {
    repo: Arc<SysJobRepository>,
    scheduler_manager: Arc<SchedulerManager>,
}

impl SysJobService {
    pub fn new(repo: Arc<SysJobRepository>, scheduler_manager: Arc<SchedulerManager>) -> Self {
        Self {
            repo,
            scheduler_manager,
        }
    }

    pub async fn list_jobs(&self, keyword: Option<&str>) -> Result<JobListVo, AppError> {
        let items = self
            .repo
            .list_jobs(keyword)
            .await?
            .into_iter()
            .map(to_job_item_vo)
            .collect::<Vec<_>>();
        Ok(JobListVo {
            total: items.len(),
            items,
        })
    }

    pub async fn create_job(&self, payload: JobUpsertReqDto) -> Result<JobItemVo, AppError> {
        validate_job_payload(&payload)?;
        SchedulerManager::validate_cron_expression(&payload.cron_expression)?;
        let created = self.repo.create_job(payload).await?;
        self.scheduler_manager.sync_job(&created).await?;
        Ok(to_job_item_vo(created))
    }

    pub async fn list_job_logs(&self, query: JobLogQueryDto) -> Result<JobLogListVo, AppError> {
        let items = self
            .repo
            .list_job_logs(&query)
            .await?
            .into_iter()
            .map(to_job_log_item_vo)
            .collect::<Vec<_>>();
        Ok(JobLogListVo {
            total: items.len(),
            items,
        })
    }

    pub async fn update_job(
        &self,
        id: u64,
        payload: JobUpsertReqDto,
    ) -> Result<JobItemVo, AppError> {
        validate_job_payload(&payload)?;
        SchedulerManager::validate_cron_expression(&payload.cron_expression)?;
        let updated = self
            .repo
            .update_job(id, payload)
            .await?
            .ok_or_else(|| AppError::not_found(format!("未找到任务: {id}")))?;
        self.scheduler_manager.sync_job(&updated).await?;
        Ok(to_job_item_vo(updated))
    }

    pub async fn delete_job(&self, id: u64) -> Result<JobActionVo, AppError> {
        let deleted = self.repo.delete_job(id).await?;
        if !deleted {
            return Err(AppError::not_found(format!("未找到任务: {id}")));
        }
        self.scheduler_manager.remove_job(id).await?;
        Ok(JobActionVo {
            id,
            status: "deleted".to_string(),
            last_run_at: None,
            next_run_at: None,
            message: "删除成功".to_string(),
        })
    }

    pub async fn run_job_once(&self, id: u64) -> Result<JobActionVo, AppError> {
        let job = self
            .scheduler_manager
            .trigger_once(id, TriggerType::Manual)
            .await?;
        Ok(JobActionVo {
            id: job.id,
            status: job.status,
            last_run_at: job.last_run_at,
            next_run_at: job.next_run_at,
            message: "任务执行成功（tokio-cron-scheduler）".to_string(),
        })
    }

    pub async fn pause_job(&self, id: u64) -> Result<JobActionVo, AppError> {
        let job = self
            .repo
            .set_job_status(id, "paused")
            .await?
            .ok_or_else(|| AppError::not_found(format!("未找到任务: {id}")))?;
        self.scheduler_manager.sync_job(&job).await?;
        Ok(JobActionVo {
            id: job.id,
            status: job.status,
            last_run_at: job.last_run_at,
            next_run_at: job.next_run_at,
            message: "任务已暂停".to_string(),
        })
    }

    pub async fn resume_job(&self, id: u64) -> Result<JobActionVo, AppError> {
        let job = self
            .repo
            .set_job_status(id, "running")
            .await?
            .ok_or_else(|| AppError::not_found(format!("未找到任务: {id}")))?;
        self.scheduler_manager.sync_job(&job).await?;
        Ok(JobActionVo {
            id: job.id,
            status: job.status,
            last_run_at: job.last_run_at,
            next_run_at: job.next_run_at,
            message: "任务已恢复".to_string(),
        })
    }
}

fn validate_job_payload(payload: &JobUpsertReqDto) -> Result<(), AppError> {
    if payload.job_name.trim().is_empty() {
        return Err(AppError::bad_request("任务名称不能为空"));
    }
    if payload.job_group.trim().is_empty() {
        return Err(AppError::bad_request("任务组不能为空"));
    }
    if payload.invoke_target.trim().is_empty() {
        return Err(AppError::bad_request("调用目标不能为空"));
    }
    if payload.cron_expression.trim().is_empty() {
        return Err(AppError::bad_request("Cron 表达式不能为空"));
    }
    Ok(())
}

fn to_job_item_vo(item: JobPo) -> JobItemVo {
    JobItemVo {
        id: item.id,
        job_name: item.job_name,
        job_group: item.job_group,
        invoke_target: item.invoke_target,
        cron_expression: item.cron_expression,
        concurrent: item.concurrent,
        status: item.status,
        remark: item.remark,
        last_run_at: item.last_run_at,
        next_run_at: item.next_run_at,
    }
}

fn to_job_log_item_vo(item: JobLogPo) -> JobLogItemVo {
    JobLogItemVo {
        id: item.id,
        job_id: item.job_id,
        job_name: item.job_name,
        job_group: item.job_group,
        invoke_target: item.invoke_target,
        cron_expression: item.cron_expression,
        status: item.status,
        message: item.message,
        exception_info: item.exception_info,
        trigger_type: item.trigger_type,
        started_at: item.started_at,
        finished_at: item.finished_at,
        duration_ms: item.duration_ms,
    }
}
