use std::{sync::Arc, time::Duration};

use anyhow::Context;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

use crate::core::config::AppConfig;
use crate::modules::{
    ai::{repository::InMemoryAiRepository, service::AiService},
    auth::{repository::MySqlAuthRepository, service::AuthService},
    dashboard::{repository::MockDashboardRepository, service::DashboardService},
    log::{repository::MySqlLogRepository, service::LogService},
    monitor::{repository::InMemoryMonitorRepository, service::MonitorService},
    system::{repository::MySqlSystemRepository, service::SystemService},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub mysql_pool: MySqlPool,
    pub redis_client: redis::Client,
    pub auth_service: AuthService,
    pub dashboard_service: DashboardService,
    pub system_service: SystemService,
    pub log_service: LogService,
    pub monitor_service: MonitorService,
    pub ai_service: AiService,
}

impl AppState {
    pub async fn new(config: AppConfig) -> anyhow::Result<Self> {
        let mysql_pool = MySqlPoolOptions::new()
            .max_connections(config.mysql.max_connections)
            .min_connections(config.mysql.min_connections)
            .acquire_timeout(Duration::from_secs(config.mysql.acquire_timeout_secs))
            .connect(&config.mysql.url)
            .await
            .context("failed to connect mysql")?;

        let redis_client =
            redis::Client::open(config.redis.url.as_str()).context("failed to create redis client")?;

        {
            let mut conn = tokio::time::timeout(
                Duration::from_secs(config.redis.connection_timeout_secs),
                redis_client.get_multiplexed_async_connection(),
            )
            .await
            .context("redis connect timeout")?
            .context("failed to connect redis")?;

            let pong: String = redis::cmd("PING")
                .query_async(&mut conn)
                .await
                .context("failed to ping redis")?;
            if pong != "PONG" {
                return Err(anyhow::anyhow!("redis ping returned unexpected result: {pong}"));
            }
        }

        let auth_repo = MySqlAuthRepository::new(mysql_pool.clone());
        let dashboard_repo = MockDashboardRepository::new_arc();
        let system_repo = MySqlSystemRepository::new(mysql_pool.clone());
        let log_repo = MySqlLogRepository::new(mysql_pool.clone());
        let monitor_repo = InMemoryMonitorRepository::seeded();
        let ai_repo = InMemoryAiRepository::seeded();
        let jwt_secret = config.security.jwt_secret.clone();
        let jwt_expires_secs = config.security.jwt_expires_secs;
        let mysql_pool_for_auth = mysql_pool.clone();

        let monitor_service = MonitorService::new(
            monitor_repo,
            mysql_pool.clone(),
            redis_client.clone(),
            Arc::new(config.clone()),
        );
        monitor_service.start_builtin_scheduler();

        Ok(Self {
            config: Arc::new(config),
            mysql_pool,
            redis_client,
            auth_service: AuthService::new(
                auth_repo,
                jwt_secret,
                jwt_expires_secs,
                mysql_pool_for_auth,
            ),
            dashboard_service: DashboardService::new(dashboard_repo),
            system_service: SystemService::new(system_repo),
            log_service: LogService::new(log_repo),
            monitor_service,
            ai_service: AiService::new(ai_repo),
        })
    }

    pub async fn mysql_ping(&self) -> anyhow::Result<()> {
        let _: i32 = sqlx::query_scalar("SELECT 1")
            .fetch_one(&self.mysql_pool)
            .await
            .context("mysql ping query failed")?;
        Ok(())
    }

    pub async fn redis_ping(&self) -> anyhow::Result<()> {
        let mut conn = tokio::time::timeout(
            Duration::from_secs(self.config.redis.connection_timeout_secs),
            self.redis_client.get_multiplexed_async_connection(),
        )
        .await
        .context("redis connect timeout")?
        .context("failed to connect redis")?;

        let pong: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .context("failed to ping redis")?;
        if pong != "PONG" {
            return Err(anyhow::anyhow!("redis ping returned unexpected result: {pong}"));
        }
        Ok(())
    }
}
