use std::{sync::Arc, time::Duration};

use anyhow::Context;
use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions};

mod repository_factory;

use crate::core::{
    config::{AppConfig, DatabaseDriver},
    db::DbPool,
};
use crate::modules::{
    ai::{repository::InMemoryAiRepository, service::AiService},
    auth::service::AuthService,
    dashboard::{repository::MockDashboardRepository, service::DashboardService},
    log::service::LogService,
    monitor::{repository::InMemoryMonitorRepository, service::MonitorService},
    system::service::SystemService,
};

use self::repository_factory::build_repository_bundle;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: DbPool,
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
        let db_pool = match config.database.driver {
            DatabaseDriver::MySql => {
                let pool = MySqlPoolOptions::new()
                    .max_connections(config.database.max_connections)
                    .min_connections(config.database.min_connections)
                    .acquire_timeout(Duration::from_secs(config.database.acquire_timeout_secs))
                    .connect(&config.database.url)
                    .await
                    .context("failed to connect mysql")?;
                DbPool::MySql(pool)
            }
            DatabaseDriver::Postgres => {
                let pool = PgPoolOptions::new()
                    .max_connections(config.database.max_connections)
                    .min_connections(config.database.min_connections)
                    .acquire_timeout(Duration::from_secs(config.database.acquire_timeout_secs))
                    .connect(&config.database.url)
                    .await
                    .context("failed to connect postgres")?;
                DbPool::Postgres(pool)
            }
        };

        let redis_client = redis::Client::open(config.redis.url.as_str())
            .context("failed to create redis client")?;

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
                return Err(anyhow::anyhow!(
                    "redis ping returned unexpected result: {pong}"
                ));
            }
        }

        let repositories = build_repository_bundle(config.database.driver, &db_pool)?;
        let dashboard_repo = MockDashboardRepository::new_arc();
        let monitor_repo = InMemoryMonitorRepository::seeded();
        let ai_repo = InMemoryAiRepository::seeded();
        let jwt_secret = config.security.jwt_secret.clone();
        let jwt_expires_secs = config.security.jwt_expires_secs;

        let monitor_service = MonitorService::new(
            monitor_repo,
            db_pool.clone(),
            redis_client.clone(),
            Arc::new(config.clone()),
        );
        monitor_service.start_builtin_scheduler();

        Ok(Self {
            config: Arc::new(config),
            db_pool,
            redis_client,
            auth_service: AuthService::new(repositories.auth_repo, jwt_secret, jwt_expires_secs),
            dashboard_service: DashboardService::new(dashboard_repo),
            system_service: SystemService::new(repositories.system_repo),
            log_service: LogService::new(repositories.log_repo),
            monitor_service,
            ai_service: AiService::new(ai_repo),
        })
    }

    pub async fn db_ping(&self) -> anyhow::Result<()> {
        self.db_pool.ping().await
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
            return Err(anyhow::anyhow!(
                "redis ping returned unexpected result: {pong}"
            ));
        }
        Ok(())
    }
}
