pub mod integration;

use std::sync::Arc;

use crate::{
    core::{converter::dashboard::to_overview_vo, vo::dashboard::DashboardOverviewVo},
    modules::dashboard::repository::DashboardRepository,
};

#[derive(Clone)]
pub struct DashboardService {
    repo: Arc<dyn DashboardRepository>,
}

impl DashboardService {
    pub fn new(repo: Arc<dyn DashboardRepository>) -> Self {
        Self { repo }
    }

    pub fn overview(&self) -> DashboardOverviewVo {
        let data = self.repo.load_overview();
        to_overview_vo(data)
    }
}
