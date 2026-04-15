#![allow(dead_code)]

pub mod sys_auth_repository;
pub mod sys_config_repository;
pub mod sys_dept_repository;
pub mod sys_dict_repository;
pub mod sys_log_repository;
pub mod sys_menu_repository;
pub mod sys_notice_repository;
pub mod sys_post_repository;
pub mod sys_role_repository;
pub mod sys_user_repository;

pub use {
    sys_auth_repository::ISysAuthRepository, sys_config_repository::ISysConfigRepository,
    sys_dept_repository::ISysDeptRepository, sys_dict_repository::ISysDictRepository,
    sys_log_repository::ISysLogRepository, sys_menu_repository::ISysMenuRepository,
    sys_notice_repository::ISysNoticeRepository, sys_post_repository::ISysPostRepository,
    sys_role_repository::ISysRoleRepository, sys_user_repository::ISysUserRepository,
};
