use crate::core::{
    dto::monitor_dto::JobUpsertReqDto,
    errors::AppError,
    model::monitor::JobPo,
    vo::monitor_vo::{JobActionVo, JobItemVo, JobListVo},
};

use super::MonitorService;

impl MonitorService {
    pub async fn list_jobs(&self, keyword: Option<&str>) -> JobListVo {
        let items = self
            .repo
            .list_jobs(keyword)
            .await
            .into_iter()
            .map(to_job_item_vo)
            .collect::<Vec<_>>();
        JobListVo {
            total: items.len(),
            items,
        }
    }

    pub async fn create_job(&self, payload: JobUpsertReqDto) -> Result<JobItemVo, AppError> {
        validate_job_payload(&payload)?;
        let created = self.repo.create_job(payload).await;
        Ok(to_job_item_vo(created))
    }

    pub async fn update_job(
        &self,
        id: u64,
        payload: JobUpsertReqDto,
    ) -> Result<JobItemVo, AppError> {
        validate_job_payload(&payload)?;
        let updated = self
            .repo
            .update_job(id, payload)
            .await
            .ok_or_else(|| AppError::not_found(format!("未找到任务: {id}")))?;
        Ok(to_job_item_vo(updated))
    }

    pub async fn delete_job(&self, id: u64) -> Result<JobActionVo, AppError> {
        let deleted = self.repo.delete_job(id).await;
        if !deleted {
            return Err(AppError::not_found(format!("未找到任务: {id}")));
        }
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
            .repo
            .run_job_once(id)
            .await
            .ok_or_else(|| AppError::not_found(format!("未找到任务: {id}")))?;
        Ok(JobActionVo {
            id: job.id,
            status: job.status,
            last_run_at: job.last_run_at,
            next_run_at: job.next_run_at,
            message: "任务执行成功（内置调度器）".to_string(),
        })
    }

    pub async fn pause_job(&self, id: u64) -> Result<JobActionVo, AppError> {
        let job = self
            .repo
            .set_job_status(id, "paused")
            .await
            .ok_or_else(|| AppError::not_found(format!("未找到任务: {id}")))?;
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
            .await
            .ok_or_else(|| AppError::not_found(format!("未找到任务: {id}")))?;
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
        status: item.status,
        remark: item.remark,
        last_run_at: item.last_run_at,
        next_run_at: item.next_run_at,
    }
}
