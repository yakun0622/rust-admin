use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::log_dto::LogListQueryDto,
        errors::AppError,
        response::ApiResponse,
        vo::log_vo::{LoginLogListVo, OperLogListVo},
    },
};

pub struct SysLogRouter;

impl SysLogRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/oper", get(oper_logs))
            .route("/login", get(login_logs))
    }
}

async fn oper_logs(
    State(state): State<AppState>,
    Query(query): Query<LogListQueryDto>,
) -> Result<Json<ApiResponse<OperLogListVo>>, AppError> {
    Ok(Json(ApiResponse::success(
        state
            .log_service()
            .list_oper(query.keyword.as_deref())
            .await?,
    )))
}

async fn login_logs(
    State(state): State<AppState>,
    Query(query): Query<LogListQueryDto>,
) -> Result<Json<ApiResponse<LoginLogListVo>>, AppError> {
    Ok(Json(ApiResponse::success(
        state
            .log_service()
            .list_login(query.keyword.as_deref())
            .await?,
    )))
}
