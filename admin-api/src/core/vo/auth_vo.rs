use serde::Serialize;

use crate::core::vo::sys_menu_vo::SysMenuVo;

#[derive(Debug, Clone, Serialize)]
pub struct LoginVo {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: u64,
    pub username: String,
    pub nickname: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthProfileUserVo {
    pub user_id: u64,
    pub username: String,
    pub nickname: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthProfileVo {
    pub user: AuthProfileUserVo,
    pub permissions: Vec<String>,
    pub menus: Vec<SysMenuVo>,
}
