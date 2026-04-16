use std::sync::Arc;

use crate::core::vo::monitor_vo::{DatasourceOverviewVo, ServerOverviewVo};
use crate::core::{config::AppConfig, db::DbPool, redis::RedisClient, utils::now_timestamp_millis};

#[derive(Clone)]
pub struct MonitorOverviewService {
    db_pool: DbPool,
    redis_client: RedisClient,
    config: Arc<AppConfig>,
    started_at_millis: i64,
}

impl MonitorOverviewService {
    pub fn new(db_pool: DbPool, redis_client: RedisClient, config: Arc<AppConfig>) -> Self {
        Self {
            db_pool,
            redis_client,
            config,
            started_at_millis: now_timestamp_millis(),
        }
    }

    pub async fn datasource_overview(&self) -> DatasourceOverviewVo {
        let (ping_ok, ping_message) = match self.db_pool.ping().await {
            Ok(_) => (true, "ok".to_string()),
            Err(err) => (false, err.to_string()),
        };

        DatasourceOverviewVo {
            database: self.config.database.driver.display_name().to_string(),
            mysql_url: mask_database_url(&self.config.database.url),
            max_connections: self.config.database.max_connections,
            min_connections: self.config.database.min_connections,
            ping_ok,
            ping_message,
        }
    }

    pub async fn server_overview(&self) -> ServerOverviewVo {
        let mysql_ok = self.db_pool.ping().await.is_ok();

        let redis_ok = self.redis_client.ping().await.is_ok();

        let now = crate::core::utils::now_timestamp_millis();
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
}

fn mask_database_url(url: &str) -> String {
    if let Some((prefix, suffix)) = url.split_once('@') {
        if let Some((user, _)) = prefix.split_once("://") {
            return format!("{user}://***@{suffix}");
        }
    }
    url.to_string()
}
