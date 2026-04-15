use crate::core::model::monitor::OnlineUserPo;

use super::{filter_with_keyword, InMemoryMonitorRepository};

impl InMemoryMonitorRepository {
    pub async fn list_online_users(&self, keyword: Option<&str>) -> Vec<OnlineUserPo> {
        let users = self.online_users.read().await;
        filter_with_keyword(keyword, users.as_slice(), |item| {
            format!(
                "{} {} {} {} {}",
                item.username, item.ip, item.browser, item.os, item.status
            )
        })
    }
}
