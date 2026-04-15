pub mod integration;

mod overview_service;

use std::sync::Arc;

use crate::modules::dashboard::repository::DashboardRepository;

#[derive(Clone)]
pub struct DashboardService {
    repo: Arc<dyn DashboardRepository>,
}

impl DashboardService {
    pub fn new(repo: Arc<dyn DashboardRepository>) -> Self {
        Self { repo }
    }
}
