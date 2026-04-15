use std::{sync::Arc, time::Duration};

use sqlx::mysql::MySqlPoolOptions;

#[allow(unused_imports)]
use crate::wire;

use crate::core::{config::AppConfig, db::DbPool, errors::AppError};
use crate::modules::{
    ai::{repository::InMemoryAiRepository, service::AiService},
    dashboard::{repository::MockDashboardRepository, service::DashboardService},
    monitor::{repository::InMemoryMonitorRepository, service::MonitorService},
    system::repository::{
        SysAuthRepository, SysConfigRepository, SysDeptRepository, SysDictRepository,
        SysLogRepository, SysMenuRepository, SysNoticeRepository, SysPostRepository,
        SysRoleRepository, SysUserRepository,
    },
    system::service::{
        SysAuthService, SysConfigService, SysDeptService, SysDictService, SysLogService,
        SysMenuService, SysNoticeService, SysPostService, SysRoleService, SysUserService,
    },
};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: DbPool,
    pub redis_client: redis::Client,
    pub sys_auth_service: SysAuthService,
    pub dashboard_service: DashboardService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_menu_service: SysMenuService,
    pub sys_dept_service: SysDeptService,
    pub sys_post_service: SysPostService,
    pub sys_dict_service: SysDictService,
    pub sys_config_service: SysConfigService,
    pub sys_notice_service: SysNoticeService,
    pub sys_log_service: SysLogService,
    pub monitor_service: MonitorService,
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

        let redis_client = redis::Client::open(config.redis.url.as_str())
            .map_err(|e| -> BoxError { format!("failed to create redis client: {e}").into() })?;

        {
            let mut conn = tokio::time::timeout(
                Duration::from_secs(config.redis.connection_timeout_secs),
                redis_client.get_multiplexed_async_connection(),
            )
            .await
            .map_err(|_| -> BoxError { "redis connect timeout".into() })?
            .map_err(|e| -> BoxError { format!("failed to connect redis: {e}").into() })?;

            let pong: String = redis::cmd("PING")
                .query_async(&mut conn)
                .await
                .map_err(|e| -> BoxError { format!("failed to ping redis: {e}").into() })?;
            if pong != "PONG" {
                return Err(format!("redis ping returned unexpected result: {pong}").into());
            }
        }

        // 标准装配：Service[Repo] 全部由 pool 驱动
        wire!(pool;
            sys_user_service:   SysUserService[SysUserRepository],
            sys_role_service:   SysRoleService[SysRoleRepository],
            sys_menu_service:   SysMenuService[SysMenuRepository],
            sys_dept_service:   SysDeptService[SysDeptRepository],
            sys_post_service:   SysPostService[SysPostRepository],
            sys_dict_service:   SysDictService[SysDictRepository],
            sys_config_service: SysConfigService[SysConfigRepository],
            sys_notice_service: SysNoticeService[SysNoticeRepository],
            sys_log_service:    SysLogService[SysLogRepository],
        );

        // 需要额外参数的单独处理
        let sys_auth_service = SysAuthService::new(
            SysAuthRepository::new(pool.clone()),
            config.security.jwt_secret.clone(),
            config.security.jwt_expires_secs,
        );

        let dashboard_repo = MockDashboardRepository::new_arc();
        let monitor_repo = InMemoryMonitorRepository::seeded();
        let ai_repo = InMemoryAiRepository::seeded();

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
            sys_auth_service,
            dashboard_service: DashboardService::new(dashboard_repo),
            sys_user_service,
            sys_role_service,
            sys_menu_service,
            sys_dept_service,
            sys_post_service,
            sys_dict_service,
            sys_config_service,
            sys_notice_service,
            sys_log_service,
            monitor_service,
            ai_service: AiService::new(ai_repo),
        })
    }

    pub async fn db_ping(&self) -> Result<(), AppError> {
        self.db_pool.ping().await
    }

    pub async fn redis_ping(&self) -> Result<(), AppError> {
        let mut conn = tokio::time::timeout(
            Duration::from_secs(self.config.redis.connection_timeout_secs),
            self.redis_client.get_multiplexed_async_connection(),
        )
        .await
        .map_err(|_| AppError::internal("redis connect timeout"))?
        .map_err(|e| AppError::internal(format!("failed to connect redis: {e}")))?;

        let pong: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::internal(format!("failed to ping redis: {e}")))?;
        if pong != "PONG" {
            return Err(AppError::internal(format!(
                "redis ping returned unexpected result: {pong}"
            )));
        }
        Ok(())
    }
}
