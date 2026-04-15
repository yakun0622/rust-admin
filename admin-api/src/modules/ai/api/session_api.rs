use axum::{extract::State, routing::get, Json, Router};

use crate::{
    app::state::AppState,
    core::{
        dto::ai_dto::CreateAiSessionReqDto,
        errors::AppError,
        response::ApiResponse,
        vo::ai_vo::{AiSessionItemVo, AiSessionListVo},
    },
};

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/sessions", get(list_sessions).post(create_session))
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
