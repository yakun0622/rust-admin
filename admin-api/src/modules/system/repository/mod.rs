mod sys_auth_repository;
mod sys_config_repository;
mod sys_dept_repository;
mod sys_dict_repository;
mod sys_job_repository;
mod sys_log_repository;
mod sys_menu_repository;
mod sys_notice_repository;
mod sys_post_repository;
mod sys_role_repository;
mod sys_user_repository;

pub(crate) const DEFAULT_PASSWORD_HASH: &str =
    "$2b$10$jh6uvsoSAuxAfUYOc5ckkecacY3x2zPL0GuvlX38JCpRHM2OtoByi";

pub use {
    sys_auth_repository::ISysAuthRepository, sys_config_repository::ISysConfigRepository,
    sys_dept_repository::ISysDeptRepository, sys_dict_repository::ISysDictRepository,
    sys_job_repository::SysJobRepository, sys_log_repository::ISysLogRepository,
    sys_menu_repository::ISysMenuRepository, sys_notice_repository::ISysNoticeRepository,
    sys_post_repository::ISysPostRepository, sys_role_repository::ISysRoleRepository,
    sys_user_repository::ISysUserRepository,
};

pub(crate) use {
    sys_auth_repository::{SysAuthRepository, SysAuthRepositoryParameters},
    sys_config_repository::{SysConfigRepository, SysConfigRepositoryParameters},
    sys_dept_repository::{SysDeptRepository, SysDeptRepositoryParameters},
    sys_dict_repository::{SysDictRepository, SysDictRepositoryParameters},
    sys_job_repository::JobExecutionLogInput,
    sys_log_repository::{SysLogRepository, SysLogRepositoryParameters},
    sys_menu_repository::{SysMenuRepository, SysMenuRepositoryParameters},
    sys_notice_repository::{SysNoticeRepository, SysNoticeRepositoryParameters},
    sys_post_repository::{SysPostRepository, SysPostRepositoryParameters},
    sys_role_repository::{SysRoleRepository, SysRoleRepositoryParameters},
    sys_user_repository::{SysUserRepository, SysUserRepositoryParameters},
};
