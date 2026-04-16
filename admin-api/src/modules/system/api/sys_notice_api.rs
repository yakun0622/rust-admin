use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::system_dto::{SysNoticeCreateReqDto, SysNoticeUpdateReqDto, SystemListQueryDto},
        errors::AppError,
        response::ApiResponse,
        vo::system_vo::{SystemCrudDeleteVo, SystemCrudListVo, SystemCrudRecordVo},
    },
    middleware::auth::ensure_permission,
};

pub struct SysNoticeRouter;

impl SysNoticeRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/notice", get(list).post(create))
            .route("/notice/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SystemListQueryDto>,
) -> Result<Json<ApiResponse<SystemCrudListVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:notice:view").await?;
    let service = state.notice_service();
    let data = service.list(query.keyword.as_deref()).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysNoticeCreateReqDto>,
) -> Result<Json<ApiResponse<SystemCrudRecordVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:notice:create").await?;
    let service = state.notice_service();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SystemCrudRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysNoticeUpdateReqDto>,
) -> Result<Json<ApiResponse<SystemCrudRecordVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:notice:update").await?;
    let service = state.notice_service();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SystemCrudRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SystemCrudDeleteVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:notice:delete").await?;
    let service = state.notice_service();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SystemCrudDeleteVo {
        id,
        deleted,
    })))
}
