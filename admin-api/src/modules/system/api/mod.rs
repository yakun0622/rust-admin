mod sys_auth_api;
mod sys_config_api;
mod sys_dept_api;
mod sys_dict_api;
mod sys_log_api;
mod sys_menu_api;
mod sys_notice_api;
mod sys_post_api;
mod sys_role_api;
mod sys_user_api;

use axum::Router;

use crate::app::state::AppState;

pub fn public_router() -> Router<AppState> {
    Router::new().merge(sys_auth_api::SysAuthRouter::system_routes())
}

pub fn log_router() -> Router<AppState> {
    Router::new().merge(sys_log_api::SysLogRouter::routes())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(sys_user_api::SysUserRouter::routes())
        .merge(sys_role_api::SysRoleRouter::routes())
        .merge(sys_menu_api::SysMenuRouter::routes())
        .merge(sys_dept_api::SysDeptRouter::routes())
        .merge(sys_post_api::SysPostRouter::routes())
        .merge(sys_dict_api::SysDictRouter::routes())
        .merge(sys_config_api::SysConfigRouter::routes())
        .merge(sys_notice_api::SysNoticeRouter::routes())
}
