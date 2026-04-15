mod message_repository;
mod session_repository;

use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::core::{
    model::ai::{AiMessagePo, AiSessionPo},
    utils::now_timestamp_millis,
};

#[derive(Debug, Default)]
pub struct InMemoryAiRepository {
    sessions: RwLock<Vec<AiSessionPo>>,
    messages: RwLock<HashMap<u64, Vec<AiMessagePo>>>,
}

impl InMemoryAiRepository {
    pub fn seeded() -> Arc<Self> {
        let now = now_timestamp_millis();
        let sessions = vec![AiSessionPo {
            id: 1,
            title: "默认会话".to_string(),
            status: "active".to_string(),
            last_active_at: now,
        }];

        let mut messages_map = HashMap::new();
        messages_map.insert(
            1,
            vec![AiMessagePo {
                id: 1,
                session_id: 1,
                role: "assistant".to_string(),
                content: "你好，我是 AI 助手（Mock）。".to_string(),
                created_at: now,
            }],
        );

        Arc::new(Self {
            sessions: RwLock::new(sessions),
            messages: RwLock::new(messages_map),
        })
    }
}
