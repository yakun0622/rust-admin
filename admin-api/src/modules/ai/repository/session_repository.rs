use crate::core::{model::ai::AiSessionPo, utils::now_timestamp_millis};

use super::InMemoryAiRepository;

impl InMemoryAiRepository {
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
}
