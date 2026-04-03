use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AiSessionItemVo {
    pub id: u64,
    pub title: String,
    pub status: String,
    pub last_active_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AiSessionListVo {
    pub total: usize,
    pub items: Vec<AiSessionItemVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AiMessageItemVo {
    pub id: u64,
    pub session_id: u64,
    pub role: String,
    pub content: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AiMessageListVo {
    pub session_id: u64,
    pub total: usize,
    pub items: Vec<AiMessageItemVo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AiSendMessageVo {
    pub session_id: u64,
    pub user_message: AiMessageItemVo,
    pub assistant_message: AiMessageItemVo,
}
