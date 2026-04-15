use crate::core::{model::dashboard::DashboardOverviewPo, vo::dashboard_vo::DashboardOverviewVo};

pub fn to_overview_vo(data: DashboardOverviewPo) -> DashboardOverviewVo {
    DashboardOverviewVo {
        admin_total: data.admin_total,
        online_users: data.online_users,
        role_total: data.role_total,
        menu_total: data.menu_total,
        today_logins: data.today_logins,
        today_errors: data.today_errors,
        login_trend: data.login_trend,
        action_trend: data.action_trend,
    }
}
