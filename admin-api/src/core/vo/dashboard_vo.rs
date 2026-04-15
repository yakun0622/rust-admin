use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DashboardOverviewVo {
    pub admin_total: u64,
    pub online_users: u64,
    pub role_total: u64,
    pub menu_total: u64,
    pub today_logins: u64,
    pub today_errors: u64,
    pub login_trend: Vec<u64>,
    pub action_trend: Vec<u64>,
}
