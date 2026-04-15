use crate::{core::converter::dashboard_converter::to_overview_vo, core::vo::dashboard_vo::DashboardOverviewVo};

use super::DashboardService;

impl DashboardService {
    pub fn overview(&self) -> DashboardOverviewVo {
        let data = self.repo.load_overview();
        to_overview_vo(data)
    }
}
