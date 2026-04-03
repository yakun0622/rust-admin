pub mod integration;

use std::{collections::BTreeMap, sync::Arc, time::Duration};

use redis::aio::MultiplexedConnection;
use sqlx::MySqlPool;

use crate::{
    core::{
        config::AppConfig,
        dto::monitor::JobUpsertReqDto,
        errors::AppError,
        utils::now_timestamp_millis,
        vo::monitor::{
            CacheKeyItemVo, CacheNamespaceItemVo, CacheNamespaceListVo, CacheSearchVo, DatasourceOverviewVo,
            JobActionVo, JobItemVo, JobListVo, OnlineUserItemVo, OnlineUserListVo, ServerOverviewVo,
        },
    },
    modules::monitor::repository::InMemoryMonitorRepository,
};

#[derive(Clone)]
pub struct MonitorService {
    repo: Arc<InMemoryMonitorRepository>,
    mysql_pool: MySqlPool,
    redis_client: redis::Client,
    config: Arc<AppConfig>,
    started_at_millis: i64,
}

impl MonitorService {
    pub fn new(
        repo: Arc<InMemoryMonitorRepository>,
        mysql_pool: MySqlPool,
        redis_client: redis::Client,
        config: Arc<AppConfig>,
    ) -> Self {
        Self {
            repo,
            mysql_pool,
            redis_client,
            config,
            started_at_millis: now_timestamp_millis(),
        }
    }

    pub fn start_builtin_scheduler(&self) {
        let repo = self.repo.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(15));
            loop {
                ticker.tick().await;
                repo.tick_scheduler().await;
            }
        });
    }

    pub async fn list_online_users(&self, keyword: Option<&str>) -> OnlineUserListVo {
        let items = self
            .repo
            .list_online_users(keyword)
            .await
            .into_iter()
            .map(|item| OnlineUserItemVo {
                id: item.id,
                username: item.username,
                ip: item.ip,
                browser: item.browser,
                os: item.os,
                login_at: item.login_at,
                last_active_at: item.last_active_at,
                status: item.status,
            })
            .collect::<Vec<_>>();
        OnlineUserListVo {
            total: items.len(),
            items,
        }
    }

    pub async fn list_jobs(&self, keyword: Option<&str>) -> JobListVo {
        let items = self
            .repo
            .list_jobs(keyword)
            .await
            .into_iter()
            .map(|item| JobItemVo {
                id: item.id,
                job_name: item.job_name,
                job_group: item.job_group,
                invoke_target: item.invoke_target,
                cron_expression: item.cron_expression,
                status: item.status,
                remark: item.remark,
                last_run_at: item.last_run_at,
                next_run_at: item.next_run_at,
            })
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

    pub async fn update_job(&self, id: u64, payload: JobUpsertReqDto) -> Result<JobItemVo, AppError> {
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

    pub async fn datasource_overview(&self) -> DatasourceOverviewVo {
        let ping = sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&self.mysql_pool)
            .await;
        let (ping_ok, ping_message) = match ping {
            Ok(_) => (true, "ok".to_string()),
            Err(err) => (false, err.to_string()),
        };

        DatasourceOverviewVo {
            database: "MySQL".to_string(),
            mysql_url: mask_mysql_url(&self.config.mysql.url),
            max_connections: self.config.mysql.max_connections,
            min_connections: self.config.mysql.min_connections,
            ping_ok,
            ping_message,
        }
    }

    pub async fn server_overview(&self) -> ServerOverviewVo {
        let mysql_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&self.mysql_pool)
            .await
            .is_ok();

        let redis_ok = match self.redis_client.get_multiplexed_async_connection().await {
            Ok(mut conn) => redis::cmd("PING")
                .query_async::<String>(&mut conn)
                .await
                .map(|pong| pong == "PONG")
                .unwrap_or(false),
            Err(_) => false,
        };

        let now = now_timestamp_millis();
        let uptime_secs = now.saturating_sub(self.started_at_millis) as u64 / 1000;
        ServerOverviewVo {
            app_name: self.config.app.name.clone(),
            env: self.config.app.env.clone(),
            uptime_secs,
            mysql_ok,
            redis_ok,
            now_millis: now,
        }
    }

    pub async fn search_cache(
        &self,
        keyword: Option<&str>,
        limit: usize,
    ) -> Result<CacheSearchVo, AppError> {
        let safe_limit = limit.clamp(1, 200);
        let pattern = build_redis_pattern(keyword);
        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|err| AppError::internal(format!("获取 Redis 连接失败: {err}")))?;

        let keys = scan_keys(&mut conn, &pattern, safe_limit)
            .await
            .map_err(|err| AppError::internal(format!("扫描 Redis Key 失败: {err}")))?;
        let mut items = Vec::with_capacity(keys.len());

        for key in keys {
            let data_type = redis::cmd("TYPE")
                .arg(&key)
                .query_async::<String>(&mut conn)
                .await
                .unwrap_or_else(|_| "unknown".to_string());
            let ttl_secs = redis::cmd("TTL")
                .arg(&key)
                .query_async::<i64>(&mut conn)
                .await
                .unwrap_or(-2);
            let sample = if data_type == "string" {
                redis::cmd("GET")
                    .arg(&key)
                    .query_async::<Option<String>>(&mut conn)
                    .await
                    .ok()
                    .flatten()
                    .map(|value| truncate_sample(&value))
                    .unwrap_or_else(|| "-".to_string())
            } else {
                format!("<{data_type}>")
            };

            items.push(CacheKeyItemVo {
                key,
                data_type,
                ttl_secs,
                sample,
            });
        }

        Ok(CacheSearchVo {
            pattern,
            total: items.len(),
            items,
        })
    }

    pub async fn cache_namespace_list(&self) -> Result<CacheNamespaceListVo, AppError> {
        let mut conn = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|err| AppError::internal(format!("获取 Redis 连接失败: {err}")))?;

        let keys = scan_keys(&mut conn, "*", 500)
            .await
            .map_err(|err| AppError::internal(format!("扫描 Redis Key 失败: {err}")))?;

        let mut namespace_map: BTreeMap<String, (u64, String)> = BTreeMap::new();
        for key in keys {
            let namespace = key
                .split(':')
                .next()
                .filter(|segment| !segment.trim().is_empty())
                .unwrap_or("(root)")
                .to_string();
            let entry = namespace_map
                .entry(namespace)
                .or_insert((0_u64, key.clone()));
            entry.0 += 1;
            if entry.1.is_empty() {
                entry.1 = key;
            }
        }

        let items = namespace_map
            .into_iter()
            .map(|(namespace, (key_count, example_key))| CacheNamespaceItemVo {
                namespace,
                key_count,
                example_key,
            })
            .collect::<Vec<_>>();

        Ok(CacheNamespaceListVo {
            total: items.len(),
            items,
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

fn to_job_item_vo(item: crate::core::model::monitor::JobPo) -> JobItemVo {
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

fn build_redis_pattern(keyword: Option<&str>) -> String {
    let kw = keyword.map(str::trim).unwrap_or_default();
    if kw.is_empty() {
        "*".to_string()
    } else {
        format!("*{kw}*")
    }
}

async fn scan_keys(
    conn: &mut MultiplexedConnection,
    pattern: &str,
    limit: usize,
) -> redis::RedisResult<Vec<String>> {
    let mut cursor = 0_u64;
    let mut keys = Vec::new();
    while keys.len() < limit {
        let (next_cursor, batch): (u64, Vec<String>) = redis::cmd("SCAN")
            .arg(cursor)
            .arg("MATCH")
            .arg(pattern)
            .arg("COUNT")
            .arg(100)
            .query_async(conn)
            .await?;

        for key in batch {
            keys.push(key);
            if keys.len() >= limit {
                break;
            }
        }
        if next_cursor == 0 {
            break;
        }
        cursor = next_cursor;
    }

    Ok(keys)
}

fn truncate_sample(value: &str) -> String {
    const MAX_LEN: usize = 120;
    if value.len() <= MAX_LEN {
        value.to_string()
    } else {
        format!("{}...", &value[..MAX_LEN])
    }
}

fn mask_mysql_url(url: &str) -> String {
    if let Some((prefix, suffix)) = url.split_once('@') {
        if let Some((user, _)) = prefix.split_once("://") {
            return format!("{user}://***@{suffix}");
        }
    }
    url.to_string()
}
