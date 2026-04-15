use crate::core::vo::monitor_vo::{OnlineUserItemVo, OnlineUserListVo};

use super::MonitorService;

impl MonitorService {
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
