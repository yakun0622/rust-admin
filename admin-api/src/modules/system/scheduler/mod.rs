use std::{collections::HashMap, str::FromStr, sync::Arc, time::Instant};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cron::Schedule;
use tokio::sync::RwLock;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    core::{
        errors::AppError, model::monitor::JobPo, redis::RedisClient, utils::now_timestamp_millis,
    },
    modules::system::repository::{JobExecutionLogInput, SysJobRepository},
};

#[derive(Debug, Clone, Copy)]
pub enum TriggerType {
    Auto,
    Manual,
}

impl TriggerType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Manual => "manual",
        }
    }
}

#[async_trait]
pub trait IJobDispatcher: Send + Sync {
    async fn dispatch(&self, job: &JobPo, trigger_type: TriggerType) -> Result<String, AppError>;
}

#[derive(Clone)]
pub struct SchedulerManager {
    repo: Arc<SysJobRepository>,
    redis_client: RedisClient,
    dispatcher: Arc<dyn IJobDispatcher>,
    scheduler: Arc<RwLock<Option<Arc<JobScheduler>>>>,
    scheduler_job_map: Arc<RwLock<HashMap<u64, Uuid>>>,
}

impl SchedulerManager {
    pub fn new(
        repo: Arc<SysJobRepository>,
        redis_client: RedisClient,
        dispatcher: Arc<dyn IJobDispatcher>,
    ) -> Self {
        Self {
            repo,
            redis_client,
            dispatcher,
            scheduler: Arc::new(RwLock::new(None)),
            scheduler_job_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start(&self) -> Result<(), AppError> {
        let scheduler = Arc::new(
            JobScheduler::new()
                .await
                .map_err(|err| AppError::internal(format!("初始化 cron 调度器失败: {err}")))?,
        );

        scheduler
            .start()
            .await
            .map_err(|err| AppError::internal(format!("启动 cron 调度器失败: {err}")))?;

        {
            let mut scheduler_guard = self.scheduler.write().await;
            *scheduler_guard = Some(scheduler);
        }

        self.load_jobs_on_startup().await?;
        info!("system scheduler manager started");
        Ok(())
    }

    pub async fn load_jobs_on_startup(&self) -> Result<(), AppError> {
        let jobs = self.repo.list_jobs(None).await?;
        for job in jobs {
            self.sync_job(&job).await?;
        }
        Ok(())
    }

    pub async fn sync_job(&self, job: &JobPo) -> Result<(), AppError> {
        self.unschedule_job(job.id).await?;

        if job.status != "running" {
            self.repo.update_next_run_at(job.id, None).await?;
            return Ok(());
        }

        let next_run_at = next_run_at_millis(&job.cron_expression, now_timestamp_millis())?;
        self.repo.update_next_run_at(job.id, next_run_at).await?;

        let scheduler = self.scheduler_ref().await?;
        let scheduled_job = self.build_scheduler_job(job.id, &job.cron_expression)?;
        let scheduler_job_id = scheduled_job.guid();

        scheduler
            .add(scheduled_job)
            .await
            .map_err(|err| AppError::internal(format!("注册定时任务失败(id={}): {err}", job.id)))?;

        self.scheduler_job_map
            .write()
            .await
            .insert(job.id, scheduler_job_id);
        Ok(())
    }

    pub async fn remove_job(&self, job_id: u64) -> Result<(), AppError> {
        self.unschedule_job(job_id).await
    }

    pub async fn trigger_once(
        &self,
        job_id: u64,
        trigger_type: TriggerType,
    ) -> Result<JobPo, AppError> {
        self.execute_job_with_lock(job_id, trigger_type).await?;
        self.repo
            .get_job_by_id(job_id)
            .await?
            .ok_or_else(|| AppError::not_found(format!("未找到任务: {job_id}")))
    }

    pub fn validate_cron_expression(cron_expression: &str) -> Result<(), AppError> {
        Schedule::from_str(cron_expression)
            .map(|_| ())
            .map_err(|err| {
                AppError::bad_request(format!("Cron 表达式非法: {cron_expression}, error: {err}"))
            })
    }

    async fn scheduler_ref(&self) -> Result<Arc<JobScheduler>, AppError> {
        self.scheduler
            .read()
            .await
            .as_ref()
            .cloned()
            .ok_or_else(|| AppError::internal("cron 调度器尚未启动"))
    }

    async fn unschedule_job(&self, job_id: u64) -> Result<(), AppError> {
        let Some(scheduler_job_id) = self.scheduler_job_map.write().await.remove(&job_id) else {
            return Ok(());
        };

        let scheduler = self.scheduler_ref().await?;
        if let Err(err) = scheduler.remove(&scheduler_job_id).await {
            warn!(
                "remove scheduler job failed, job_id={}, scheduler_job_id={}, err={}",
                job_id, scheduler_job_id, err
            );
        }
        Ok(())
    }

    fn build_scheduler_job(&self, job_id: u64, cron_expression: &str) -> Result<Job, AppError> {
        let manager = self.clone();
        let expression = cron_expression.to_string();
        let job = Job::new_async(
            cron_expression,
            move |_scheduler_job_id, _scheduler_lock| {
                let manager = manager.clone();
                Box::pin(async move {
                    if let Err(err) = manager
                        .execute_job_with_lock(job_id, TriggerType::Auto)
                        .await
                    {
                        warn!(
                            "scheduled execute failed, job_id={}, err={}",
                            job_id, err.message
                        );
                    }
                })
            },
        )
        .map_err(|err| {
            AppError::bad_request(format!("Cron 表达式非法: {expression}, error: {err}"))
        })?;
        Ok(job)
    }

    async fn execute_job_with_lock(
        &self,
        job_id: u64,
        trigger_type: TriggerType,
    ) -> Result<(), AppError> {
        let Some(job_snapshot) = self.repo.get_job_by_id(job_id).await? else {
            if matches!(trigger_type, TriggerType::Auto) {
                return Ok(());
            }
            return Err(AppError::not_found(format!("未找到任务: {job_id}")));
        };

        if matches!(trigger_type, TriggerType::Auto) && job_snapshot.status != "running" {
            return Ok(());
        }

        let lock_key = format!("rust-admin:job:lock:{job_id}");
        let lock_token = match self.try_acquire_lock(&lock_key).await? {
            Some(token) => token,
            None => match trigger_type {
                TriggerType::Auto => return Ok(()),
                TriggerType::Manual => {
                    return Err(AppError::bad_request("任务正在执行中，请稍后重试"));
                }
            },
        };

        let execute_result = async {
            let started_at_millis = now_timestamp_millis();
            let started_at = Instant::now();
            let dispatch_result = self.dispatcher.dispatch(&job_snapshot, trigger_type).await;
            let finished_at_millis = now_timestamp_millis();
            let duration_ms = started_at.elapsed().as_millis().min(u32::MAX as u128) as u32;

            let latest_job = self
                .repo
                .get_job_by_id(job_id)
                .await?
                .unwrap_or_else(|| job_snapshot.clone());
            let next_run_at_millis = if latest_job.status == "running" {
                next_run_at_millis(&latest_job.cron_expression, finished_at_millis)?
            } else {
                None
            };

            let (status, message, exception_info, dispatch_error) = match dispatch_result {
                Ok(message) => (true, message, None, None),
                Err(err) => (
                    false,
                    format!("任务执行失败: {}", err.message),
                    Some(err.message.clone()),
                    Some(err),
                ),
            };

            self.repo
                .record_job_execution(JobExecutionLogInput {
                    job_id: latest_job.id,
                    job_name: latest_job.job_name.clone(),
                    job_group: latest_job.job_group.clone(),
                    invoke_target: latest_job.invoke_target.clone(),
                    cron_expression: latest_job.cron_expression.clone(),
                    status,
                    message,
                    exception_info,
                    trigger_type: trigger_type.as_str().to_string(),
                    started_at_millis,
                    finished_at_millis,
                    duration_ms,
                    next_run_at_millis,
                })
                .await?;

            if let Some(err) = dispatch_error {
                return Err(err);
            }

            Ok(())
        }
        .await;

        if let Err(err) = self.release_lock(&lock_key, &lock_token).await {
            warn!(
                "release redis lock failed, key={}, err={}",
                lock_key, err.message
            );
        }

        execute_result
    }

    async fn try_acquire_lock(&self, lock_key: &str) -> Result<Option<String>, AppError> {
        let lock_token = Uuid::new_v4().to_string();
        let locked = self
            .redis_client
            .try_lock(lock_key, &lock_token, 60)
            .await?;
        if locked {
            Ok(Some(lock_token))
        } else {
            Ok(None)
        }
    }

    async fn release_lock(&self, lock_key: &str, lock_token: &str) -> Result<(), AppError> {
        self.redis_client
            .release_lock_if_owner(lock_key, lock_token)
            .await
    }
}

fn next_run_at_millis(cron_expression: &str, from_millis: i64) -> Result<Option<i64>, AppError> {
    let schedule = Schedule::from_str(cron_expression).map_err(|err| {
        AppError::bad_request(format!("Cron 表达式非法: {cron_expression}, error: {err}"))
    })?;
    let from_time = DateTime::<Utc>::from_timestamp_millis(from_millis).ok_or_else(|| {
        AppError::internal(format!("无效时间戳: {from_millis}，无法计算下次执行时间"))
    })?;
    Ok(schedule
        .after(&from_time)
        .next()
        .map(|next| next.timestamp_millis()))
}
