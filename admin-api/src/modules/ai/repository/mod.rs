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

    pub async fn list_sessions(&self) -> Vec<AiSessionPo> {
        self.sessions.read().await.clone()
    }

    pub async fn create_session(&self, title: String) -> AiSessionPo {
        let mut sessions = self.sessions.write().await;
        let id = sessions.iter().map(|session| session.id).max().unwrap_or(0) + 1;
        let now = now_timestamp_millis();
        let created = AiSessionPo {
            id,
            title,
            status: "active".to_string(),
            last_active_at: now,
        };
        sessions.push(created.clone());

        self.messages.write().await.entry(id).or_default();
        created
    }

    pub async fn list_messages(&self, session_id: u64) -> Option<Vec<AiMessagePo>> {
        let messages = self.messages.read().await;
        messages.get(&session_id).cloned()
    }

    pub async fn append_user_message(
        &self,
        session_id: u64,
        content: String,
    ) -> Option<AiMessagePo> {
        self.append_message(session_id, "user".to_string(), content)
            .await
    }

    pub async fn append_assistant_message(
        &self,
        session_id: u64,
        content: String,
    ) -> Option<AiMessagePo> {
        self.append_message(session_id, "assistant".to_string(), content)
            .await
    }

    async fn append_message(
        &self,
        session_id: u64,
        role: String,
        content: String,
    ) -> Option<AiMessagePo> {
        let mut messages = self.messages.write().await;
        let session_messages = messages.get_mut(&session_id)?;
        let id = session_messages
            .iter()
            .map(|message| message.id)
            .max()
            .unwrap_or(0)
            + 1;
        let now = now_timestamp_millis();
        let message = AiMessagePo {
            id,
            session_id,
            role,
            content,
            created_at: now,
        };
        session_messages.push(message.clone());

        drop(messages);

        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.iter_mut().find(|session| session.id == session_id) {
            session.last_active_at = now;
        }

        Some(message)
    }
}
