pub mod integration;

mod cache_service;
mod job_service;
mod online_service;
mod overview_service;

use std::{sync::Arc, time::Duration};

use crate::{
    core::{config::AppConfig, db::DbPool, utils::now_timestamp_millis},
    modules::monitor::repository::InMemoryMonitorRepository,
};

#[derive(Clone)]
pub struct MonitorService {
    repo: Arc<InMemoryMonitorRepository>,
    db_pool: DbPool,
    redis_client: redis::Client,
    config: Arc<AppConfig>,
    started_at_millis: i64,
}

impl MonitorService {
    pub fn new(
        repo: Arc<InMemoryMonitorRepository>,
        db_pool: DbPool,
        redis_client: redis::Client,
        config: Arc<AppConfig>,
    ) -> Self {
        Self {
            repo,
            db_pool,
            redis_client,
            config,
            started_at_millis: now_timestamp_millis(),
        }
    }

    pub fn start_builtin_scheduler(&self) {
        let repo = self.repo.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(15));
            loop {
                ticker.tick().await;
                repo.tick_scheduler().await;
            }
        });
    }
}
