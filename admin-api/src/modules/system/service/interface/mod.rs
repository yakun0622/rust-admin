#![allow(dead_code)]

pub mod sys_auth_service;
pub mod sys_config_service;
pub mod sys_dept_service;
pub mod sys_dict_service;
pub mod sys_log_service;
pub mod sys_menu_service;
pub mod sys_notice_service;
pub mod sys_post_service;
pub mod sys_role_service;
pub mod sys_user_service;

pub use {
    sys_auth_service::ISysAuthService, sys_config_service::ISysConfigService,
    sys_dept_service::ISysDeptService, sys_dict_service::ISysDictService,
    sys_log_service::ISysLogService, sys_menu_service::ISysMenuService,
    sys_notice_service::ISysNoticeService, sys_post_service::ISysPostService,
    sys_role_service::ISysRoleService, sys_user_service::ISysUserService,
};
