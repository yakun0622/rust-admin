use crate::core::{
    dto::ai_dto::SendAiMessageReqDto,
    errors::AppError,
    vo::ai_vo::{AiMessageItemVo, AiMessageListVo, AiSendMessageVo},
};

use super::AiService;

impl AiService {
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
