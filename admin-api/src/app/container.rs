use std::sync::Arc;

use shaku::module;
use sqlx::MySqlPool;

use crate::{
    core::config::AppConfig,
    modules::system::{
        repository::{
            SysAuthRepository, SysAuthRepositoryParameters, SysConfigRepository,
            SysConfigRepositoryParameters, SysDeptRepository, SysDeptRepositoryParameters,
            SysDictRepository, SysDictRepositoryParameters, SysLogRepository,
            SysLogRepositoryParameters, SysMenuRepository, SysMenuRepositoryParameters,
            SysNoticeRepository, SysNoticeRepositoryParameters, SysPostRepository,
            SysPostRepositoryParameters, SysRoleRepository, SysRoleRepositoryParameters,
            SysUserRepository, SysUserRepositoryParameters,
        },
        service::{
            SysAuthService, SysAuthServiceParameters, SysConfigService, SysDeptService,
            SysDictService, SysLogService, SysMenuService, SysNoticeService, SysPostService,
            SysRoleService, SysUserService,
        },
    },
};

module! {
    pub AppModule {
        components = [
            SysAuthRepository,
            SysUserRepository,
            SysRoleRepository,
            SysMenuRepository,
            SysDeptRepository,
            SysPostRepository,
            SysDictRepository,
            SysConfigRepository,
            SysNoticeRepository,
            SysLogRepository,
            SysAuthService,
            SysUserService,
            SysRoleService,
            SysMenuService,
            SysDeptService,
            SysPostService,
            SysDictService,
            SysConfigService,
            SysNoticeService,
            SysLogService,
        ],
        providers = []
    }
}

pub fn build_app_module(pool: MySqlPool, config: &AppConfig) -> Arc<AppModule> {
    Arc::new(
        AppModule::builder()
            .with_component_parameters::<SysAuthRepository>(SysAuthRepositoryParameters {
                pool: pool.clone(),
            })
            .with_component_parameters::<SysUserRepository>(SysUserRepositoryParameters {
                pool: pool.clone(),
            })
            .with_component_parameters::<SysRoleRepository>(SysRoleRepositoryParameters {
                pool: pool.clone(),
            })
            .with_component_parameters::<SysMenuRepository>(SysMenuRepositoryParameters {
                pool: pool.clone(),
            })
            .with_component_parameters::<SysDeptRepository>(SysDeptRepositoryParameters {
                pool: pool.clone(),
            })
            .with_component_parameters::<SysPostRepository>(SysPostRepositoryParameters {
                pool: pool.clone(),
            })
            .with_component_parameters::<SysDictRepository>(SysDictRepositoryParameters {
                pool: pool.clone(),
            })
            .with_component_parameters::<SysConfigRepository>(SysConfigRepositoryParameters {
                pool: pool.clone(),
            })
            .with_component_parameters::<SysNoticeRepository>(SysNoticeRepositoryParameters {
                pool: pool.clone(),
            })
            .with_component_parameters::<SysLogRepository>(SysLogRepositoryParameters { pool })
            .with_component_parameters::<SysAuthService>(SysAuthServiceParameters {
                jwt_secret: Arc::new(config.security.jwt_secret.clone()),
                jwt_expires_secs: config.security.jwt_expires_secs,
            })
            .build(),
    )
}
