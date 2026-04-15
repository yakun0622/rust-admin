mod sys_auth_repository;
mod sys_config_repository;
mod sys_dept_repository;
mod sys_dict_repository;
mod sys_log_repository;
mod sys_menu_repository;
mod sys_notice_repository;
mod sys_post_repository;
mod sys_role_repository;
mod sys_user_repository;

pub(crate) const DEFAULT_PASSWORD_HASH: &str =
    "$2b$10$jh6uvsoSAuxAfUYOc5ckkecacY3x2zPL0GuvlX38JCpRHM2OtoByi";

pub(crate) use {
    sys_auth_repository::SysAuthRepository, sys_config_repository::SysConfigRepository,
    sys_dept_repository::SysDeptRepository, sys_dict_repository::SysDictRepository,
    sys_log_repository::SysLogRepository, sys_menu_repository::SysMenuRepository,
    sys_notice_repository::SysNoticeRepository, sys_post_repository::SysPostRepository,
    sys_role_repository::SysRoleRepository, sys_user_repository::SysUserRepository,
};
