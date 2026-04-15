use crate::core::{
    dto::ai_dto::CreateAiSessionReqDto,
    errors::AppError,
    vo::ai_vo::{AiSessionItemVo, AiSessionListVo},
};

use super::AiService;

impl AiService {
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

    pub async fn create_session(
        &self,
        payload: CreateAiSessionReqDto,
    ) -> Result<AiSessionItemVo, AppError> {
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
}
