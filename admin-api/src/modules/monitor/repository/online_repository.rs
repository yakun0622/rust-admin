use std::sync::Arc;

use tokio::sync::RwLock;

use crate::core::{model::monitor::OnlineUserPo, utils::now_timestamp_millis};

#[derive(Debug, Default)]
pub struct MonitorOnlineRepository {
    online_users: RwLock<Vec<OnlineUserPo>>,
}

impl MonitorOnlineRepository {
    pub fn seeded() -> Arc<Self> {
        let now = now_timestamp_millis();
        Arc::new(Self {
            online_users: RwLock::new(vec![
                OnlineUserPo {
                    id: 1,
                    username: "admin".to_string(),
                    ip: "127.0.0.1".to_string(),
                    browser: "Chrome 123".to_string(),
                    os: "macOS".to_string(),
                    login_at: now - 30 * 60 * 1000,
                    last_active_at: now - 30 * 1000,
                    status: "online".to_string(),
                },
                OnlineUserPo {
                    id: 2,
                    username: "ops".to_string(),
                    ip: "10.0.0.12".to_string(),
                    browser: "Edge 122".to_string(),
                    os: "Windows".to_string(),
                    login_at: now - 120 * 60 * 1000,
                    last_active_at: now - 8 * 60 * 1000,
                    status: "online".to_string(),
                },
            ]),
        })
    }

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

fn filter_with_keyword<T: Clone>(
    keyword: Option<&str>,
    data: &[T],
    to_searchable: impl Fn(&T) -> String,
) -> Vec<T> {
    let normalized = keyword
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_lowercase);

    data.iter()
        .filter(|item| {
            if let Some(ref kw) = normalized {
                to_searchable(item).to_lowercase().contains(kw)
            } else {
                true
            }
        })
        .cloned()
        .collect()
}
