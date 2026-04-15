pub mod integration;
pub mod interface;

mod sys_auth_service;
mod sys_config_service;
mod sys_dept_service;
mod sys_dict_service;
mod sys_log_service;
mod sys_menu_service;
mod sys_notice_service;
mod sys_post_service;
mod sys_role_service;
mod sys_user_service;

pub use {
    sys_auth_service::{JwtClaims, SysAuthService},
    sys_config_service::SysConfigService,
    sys_dept_service::SysDeptService,
    sys_dict_service::SysDictService,
    sys_log_service::SysLogService,
    sys_menu_service::SysMenuService,
    sys_notice_service::SysNoticeService,
    sys_post_service::SysPostService,
    sys_role_service::SysRoleService,
    sys_user_service::SysUserService,
};

pub(crate) use sys_auth_service::SysAuthServiceParameters;
