mod overview_repository;

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
