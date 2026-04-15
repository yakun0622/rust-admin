use crate::core::{model::ai::AiMessagePo, utils::now_timestamp_millis};

use super::InMemoryAiRepository;

impl InMemoryAiRepository {
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
