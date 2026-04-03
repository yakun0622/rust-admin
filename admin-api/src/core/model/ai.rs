use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSessionPo {
    pub id: u64,
    pub title: String,
    pub status: String,
    pub last_active_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessagePo {
    pub id: u64,
    pub session_id: u64,
    pub role: String,
    pub content: String,
    pub created_at: i64,
}
