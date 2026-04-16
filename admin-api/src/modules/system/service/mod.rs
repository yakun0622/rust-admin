pub mod integration;

mod sys_auth_service;
mod sys_config_service;
mod sys_dept_service;
mod sys_dict_service;
mod sys_job_service;
mod sys_log_service;
mod sys_menu_service;
mod sys_notice_service;
mod sys_post_service;
mod sys_role_service;
mod sys_user_service;

pub use {
    sys_auth_service::{ISysAuthService, SysAuthService},
    sys_config_service::{ISysConfigService, SysConfigService},
    sys_dept_service::{ISysDeptService, SysDeptService},
    sys_dict_service::{ISysDictService, SysDictService},
    sys_job_service::SysJobService,
    sys_log_service::{ISysLogService, SysLogService},
    sys_menu_service::{ISysMenuService, SysMenuService},
    sys_notice_service::{ISysNoticeService, SysNoticeService},
    sys_post_service::{ISysPostService, SysPostService},
    sys_role_service::{ISysRoleService, SysRoleService},
    sys_user_service::{ISysUserService, SysUserService},
};

pub(crate) use sys_auth_service::SysAuthServiceParameters;
