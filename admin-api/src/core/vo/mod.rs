use serde::Serialize;

pub mod ai_vo;
pub mod auth_vo;
pub mod dashboard_vo;
pub mod log_vo;
pub mod monitor_vo;
pub mod sys_config_vo;
pub mod sys_dept_vo;
pub mod sys_dict_vo;
pub mod sys_menu_vo;
pub mod sys_post_vo;
pub mod sys_role_vo;
pub mod sys_user_vo;
pub mod system_vo;

#[derive(Debug, Clone, Serialize)]
pub struct SimpleMessageVo {
    pub module: &'static str,
    pub message: &'static str,
}
