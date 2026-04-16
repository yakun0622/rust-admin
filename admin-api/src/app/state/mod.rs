use std::{sync::Arc, time::Duration};

use shaku::HasComponent;
use sqlx::mysql::MySqlPoolOptions;

use crate::modules::{
    ai::{repository::InMemoryAiRepository, service::AiService},
    dashboard::{repository::MockDashboardRepository, service::DashboardService},
    monitor::{
        repository::MonitorOnlineRepository,
        service::{MonitorCacheService, MonitorOnlineService, MonitorOverviewService},
    },
    system::{
        repository::SysJobRepository,
        scheduler::SchedulerManager,
        service::{
            integration::SysJobDispatcherService, ISysAuthService, ISysConfigService,
            ISysDeptService, ISysDictService, ISysLogService, ISysMenuService, ISysNoticeService,
            ISysPostService, ISysRoleService, ISysUserService, SysJobService,
        },
    },
};
use crate::{
    app::container::{build_app_module, AppModule},
    core::{config::AppConfig, db::DbPool, errors::AppError, redis::RedisClient},
};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: DbPool,
    pub redis_client: RedisClient,
    pub app_module: Arc<AppModule>,
    pub dashboard_service: DashboardService,
    pub monitor_online_service: MonitorOnlineService,
    pub monitor_cache_service: MonitorCacheService,
    pub monitor_overview_service: MonitorOverviewService,
    pub sys_job_service: SysJobService,
    pub scheduler_manager: Arc<SchedulerManager>,
    pub ai_service: AiService,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, BoxError> {
        let pool = MySqlPoolOptions::new()
            .max_connections(config.database.max_connections)
            .min_connections(config.database.min_connections)
            .acquire_timeout(Duration::from_secs(config.database.acquire_timeout_secs))
            .connect(&config.database.url)
            .await
            .map_err(|e| -> BoxError { format!("failed to connect mysql: {e}").into() })?;
        let db_pool = DbPool::MySql(pool.clone());

        let raw_redis_client = redis::Client::open(config.redis.url.as_str())
            .map_err(|e| -> BoxError { format!("failed to create redis client: {e}").into() })?;
        let redis_client = RedisClient::new(raw_redis_client.clone());

        {
            tokio::time::timeout(
                Duration::from_secs(config.redis.connection_timeout_secs),
                redis_client.ping(),
            )
            .await
            .map_err(|_| -> BoxError { "redis connect timeout".into() })?
            .map_err(|e| -> BoxError { format!("failed to ping redis: {}", e.message).into() })?;
        }

        let app_module = build_app_module(pool.clone(), &config);

        let dashboard_repo = MockDashboardRepository::new_arc();
        let monitor_online_repo = MonitorOnlineRepository::seeded();
        let sys_job_repo = Arc::new(SysJobRepository::new(pool.clone()));
        let sys_job_dispatcher = Arc::new(SysJobDispatcherService::new(
            db_pool.clone(),
            redis_client.clone(),
        ));
        let ai_repo = InMemoryAiRepository::seeded();
        let scheduler_manager = Arc::new(SchedulerManager::new(
            sys_job_repo.clone(),
            redis_client.clone(),
            sys_job_dispatcher,
        ));

        let monitor_online_service = MonitorOnlineService::new(monitor_online_repo);
        let monitor_cache_service = MonitorCacheService::new(redis_client.clone());
        let monitor_overview_service = MonitorOverviewService::new(
            db_pool.clone(),
            redis_client.clone(),
            Arc::new(config.clone()),
        );
        let sys_job_service = SysJobService::new(sys_job_repo, scheduler_manager.clone());
        scheduler_manager.start().await.map_err(|e| -> BoxError {
            format!("failed to start system job scheduler: {}", e.message).into()
        })?;

        Ok(Self {
            config: Arc::new(config),
            db_pool,
            redis_client,
            app_module,
            dashboard_service: DashboardService::new(dashboard_repo),
            monitor_online_service,
            monitor_cache_service,
            monitor_overview_service,
            sys_job_service,
            scheduler_manager,
            ai_service: AiService::new(ai_repo),
        })
    }

    pub fn auth_service(&self) -> Arc<dyn ISysAuthService> {
        self.app_module.resolve()
    }

    pub fn user_service(&self) -> Arc<dyn ISysUserService> {
        self.app_module.resolve()
    }

    pub fn role_service(&self) -> Arc<dyn ISysRoleService> {
        self.app_module.resolve()
    }

    pub fn menu_service(&self) -> Arc<dyn ISysMenuService> {
        self.app_module.resolve()
    }

    pub fn dept_service(&self) -> Arc<dyn ISysDeptService> {
        self.app_module.resolve()
    }

    pub fn post_service(&self) -> Arc<dyn ISysPostService> {
        self.app_module.resolve()
    }

    pub fn dict_service(&self) -> Arc<dyn ISysDictService> {
        self.app_module.resolve()
    }

    pub fn config_service(&self) -> Arc<dyn ISysConfigService> {
        self.app_module.resolve()
    }

    pub fn notice_service(&self) -> Arc<dyn ISysNoticeService> {
        self.app_module.resolve()
    }

    pub fn log_service(&self) -> Arc<dyn ISysLogService> {
        self.app_module.resolve()
    }

    pub async fn db_ping(&self) -> Result<(), AppError> {
        self.db_pool.ping().await
    }

    pub async fn redis_ping(&self) -> Result<(), AppError> {
        tokio::time::timeout(
            Duration::from_secs(self.config.redis.connection_timeout_secs),
            self.redis_client.ping(),
        )
        .await
        .map_err(|_| AppError::internal("redis connect timeout"))?
    }
}
