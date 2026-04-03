use std::sync::Arc;

use crate::core::model::dashboard::DashboardOverviewPo;

pub trait DashboardRepository: Send + Sync {
    fn load_overview(&self) -> DashboardOverviewPo;
}

#[derive(Debug, Default)]
pub struct MockDashboardRepository;

impl MockDashboardRepository {
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl DashboardRepository for MockDashboardRepository {
    fn load_overview(&self) -> DashboardOverviewPo {
        DashboardOverviewPo {
            admin_total: 38,
            online_users: 7,
            role_total: 12,
            menu_total: 20,
            today_logins: 126,
            today_errors: 3,
            login_trend: vec![22, 18, 19, 15, 21, 14, 17],
            action_trend: vec![188, 201, 193, 175, 209, 184, 192],
        }
    }
}
