pub mod integration;

use std::sync::Arc;

use crate::{
    core::{
        dto::ai::{CreateAiSessionReqDto, SendAiMessageReqDto},
        errors::AppError,
        vo::ai::{AiMessageItemVo, AiMessageListVo, AiSendMessageVo, AiSessionItemVo, AiSessionListVo},
    },
    modules::ai::repository::InMemoryAiRepository,
};

#[derive(Clone)]
pub struct AiService {
    repo: Arc<InMemoryAiRepository>,
}

impl AiService {
    pub fn new(repo: Arc<InMemoryAiRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_sessions(&self) -> AiSessionListVo {
        let mut sessions = self
            .repo
            .list_sessions()
            .await
            .into_iter()
            .map(|session| AiSessionItemVo {
                id: session.id,
                title: session.title,
                status: session.status,
                last_active_at: session.last_active_at,
            })
            .collect::<Vec<_>>();
        sessions.sort_by(|a, b| b.last_active_at.cmp(&a.last_active_at));

        AiSessionListVo {
            total: sessions.len(),
            items: sessions,
        }
    }

    pub async fn create_session(&self, payload: CreateAiSessionReqDto) -> Result<AiSessionItemVo, AppError> {
        let title = payload
            .title
            .unwrap_or_else(|| "新会话".to_string())
            .trim()
            .to_string();
        if title.is_empty() {
            return Err(AppError::bad_request("会话标题不能为空"));
        }

        let created = self.repo.create_session(title).await;
        Ok(AiSessionItemVo {
            id: created.id,
            title: created.title,
            status: created.status,
            last_active_at: created.last_active_at,
        })
    }

    pub async fn list_messages(&self, session_id: u64) -> Result<AiMessageListVo, AppError> {
        let messages = self
            .repo
            .list_messages(session_id)
            .await
            .ok_or_else(|| AppError::not_found(format!("未找到会话: {session_id}")))?;

        let items = messages
            .into_iter()
            .map(|item| AiMessageItemVo {
                id: item.id,
                session_id: item.session_id,
                role: item.role,
                content: item.content,
                created_at: item.created_at,
            })
            .collect::<Vec<_>>();
        Ok(AiMessageListVo {
            session_id,
            total: items.len(),
            items,
        })
    }

    pub async fn send_message(
        &self,
        session_id: u64,
        payload: SendAiMessageReqDto,
    ) -> Result<AiSendMessageVo, AppError> {
        let content = payload.content.trim().to_string();
        if content.is_empty() {
            return Err(AppError::bad_request("消息内容不能为空"));
        }

        let user_message = self
            .repo
            .append_user_message(session_id, content.clone())
            .await
            .ok_or_else(|| AppError::not_found(format!("未找到会话: {session_id}")))?;

        let assistant_reply = format!(
            "这是 Mock 回复：已收到你的消息“{}”，当前版本未接入真实模型。",
            content
        );
        let assistant_message = self
            .repo
            .append_assistant_message(session_id, assistant_reply)
            .await
            .ok_or_else(|| AppError::not_found(format!("未找到会话: {session_id}")))?;

        Ok(AiSendMessageVo {
            session_id,
            user_message: AiMessageItemVo {
                id: user_message.id,
                session_id: user_message.session_id,
                role: user_message.role,
                content: user_message.content,
                created_at: user_message.created_at,
            },
            assistant_message: AiMessageItemVo {
                id: assistant_message.id,
                session_id: assistant_message.session_id,
                role: assistant_message.role,
                content: assistant_message.content,
                created_at: assistant_message.created_at,
            },
        })
    }
}
