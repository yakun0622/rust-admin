use std::sync::Arc;

use crate::core::vo::monitor_vo::{OnlineUserItemVo, OnlineUserListVo};
use crate::modules::monitor::repository::MonitorOnlineRepository;

#[derive(Clone)]
pub struct MonitorOnlineService {
    repo: Arc<MonitorOnlineRepository>,
}

impl MonitorOnlineService {
    pub fn new(repo: Arc<MonitorOnlineRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_online_users(&self, keyword: Option<&str>) -> OnlineUserListVo {
        let items = self
            .repo
            .list_online_users(keyword)
            .await
            .into_iter()
            .map(|item| OnlineUserItemVo {
                id: item.id,
                username: item.username,
                ip: item.ip,
                browser: item.browser,
                os: item.os,
                login_at: item.login_at,
                last_active_at: item.last_active_at,
                status: item.status,
            })
            .collect::<Vec<_>>();
        OnlineUserListVo {
            total: items.len(),
            items,
        }
    }
}
