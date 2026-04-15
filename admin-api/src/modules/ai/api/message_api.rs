use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::ai_dto::SendAiMessageReqDto,
        errors::AppError,
        response::ApiResponse,
        vo::ai_vo::{AiMessageListVo, AiSendMessageVo},
    },
};

pub(super) fn routes() -> Router<AppState> {
    Router::new().route(
        "/sessions/{session_id}/messages",
        get(list_messages).post(send_message),
    )
}

async fn list_messages(
    State(state): State<AppState>,
    Path(session_id): Path<u64>,
) -> Result<Json<ApiResponse<AiMessageListVo>>, AppError> {
    let data = state.ai_service.list_messages(session_id).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn send_message(
    State(state): State<AppState>,
    Path(session_id): Path<u64>,
    Json(payload): Json<SendAiMessageReqDto>,
) -> Result<Json<ApiResponse<AiSendMessageVo>>, AppError> {
    let data = state.ai_service.send_message(session_id, payload).await?;
    Ok(Json(ApiResponse::success(data)))
}
