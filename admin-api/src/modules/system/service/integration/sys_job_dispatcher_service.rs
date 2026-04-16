use async_trait::async_trait;
use tracing::info;

use crate::{
    core::{
        db::DbPool, errors::AppError, model::monitor::JobPo, redis::RedisClient,
        utils::now_timestamp_millis,
    },
    modules::system::scheduler::{IJobDispatcher, TriggerType},
};

#[derive(Clone)]
pub struct SysJobDispatcherService {
    db_pool: DbPool,
    redis_client: RedisClient,
}

impl SysJobDispatcherService {
    pub fn new(db_pool: DbPool, redis_client: RedisClient) -> Self {
        Self {
            db_pool,
            redis_client,
        }
    }

    async fn cleanup_login_logs(&self, keep_days: u16) -> Result<String, AppError> {
        let pool = self
            .db_pool
            .as_mysql()
            .ok_or_else(|| AppError::internal("当前数据库驱动不支持清理登录日志任务"))?;

        let result = sqlx::query(
            r#"
            DELETE FROM sys_login_log
            WHERE login_at < DATE_SUB(NOW(3), INTERVAL ? DAY)
            "#,
        )
        .bind(keep_days)
        .execute(&pool)
        .await
        .map_err(|err| AppError::internal(format!("清理登录日志失败: {err}")))?;

        Ok(format!(
            "清理登录日志完成，保留近 {keep_days} 天，删除 {} 条",
            result.rows_affected()
        ))
    }

    async fn sync_cache_stats(&self) -> Result<String, AppError> {
        info!("job monitor.cache.sync started");
        let now = now_timestamp_millis();
        self.redis_client
            .set_string("monitor:cache:last_sync_millis", now.to_string())
            .await?;

        info!("job monitor.cache.sync finished, last_sync_millis={}", now);
        Ok(format!("缓存统计同步完成，timestamp={now}"))
    }
}

#[async_trait]
impl IJobDispatcher for SysJobDispatcherService {
    async fn dispatch(&self, job: &JobPo, trigger_type: TriggerType) -> Result<String, AppError> {
        let prefix = format!(
            "[trigger={}][target={}]",
            trigger_type.as_str(),
            job.invoke_target
        );
        let message = match job.invoke_target.as_str() {
            "log.cleanup_login" => self.cleanup_login_logs(30).await?,
            "monitor.cache.sync" => self.sync_cache_stats().await?,
            _ => {
                return Err(AppError::bad_request(format!(
                    "未注册的任务目标: {}",
                    job.invoke_target
                )));
            }
        };
        Ok(format!("{prefix} {message}"))
    }
}
