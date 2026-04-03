use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::ai::{CreateAiSessionReqDto, SendAiMessageReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::ai::{AiMessageListVo, AiSendMessageVo, AiSessionItemVo, AiSessionListVo},
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sessions", get(list_sessions).post(create_session))
        .route("/sessions/{session_id}/messages", get(list_messages).post(send_message))
}

async fn list_sessions(State(state): State<AppState>) -> Json<ApiResponse<AiSessionListVo>> {
    Json(ApiResponse::success(state.ai_service.list_sessions().await))
}

async fn create_session(
    State(state): State<AppState>,
    Json(payload): Json<CreateAiSessionReqDto>,
) -> Result<Json<ApiResponse<AiSessionItemVo>>, AppError> {
    let session = state.ai_service.create_session(payload).await?;
    Ok(Json(ApiResponse::success(session)))
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
