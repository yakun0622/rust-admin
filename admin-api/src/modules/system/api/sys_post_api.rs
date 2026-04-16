use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::sys_post_dto::{SysPostCreateReqDto, SysPostListQueryDto, SysPostUpdateReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::sys_post_vo::{SysPostDeleteVo, SysPostListVo, SysPostRecordVo},
    },
    middleware::auth::ensure_permission,
};

pub struct SysPostRouter;

impl SysPostRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/post", get(list).post(create))
            .route("/post/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SysPostListQueryDto>,
) -> Result<Json<ApiResponse<SysPostListVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:post:view").await?;
    let service = state.post_service();
    let data = service.list(query.keyword.as_deref()).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysPostCreateReqDto>,
) -> Result<Json<ApiResponse<SysPostRecordVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:post:create").await?;
    let service = state.post_service();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SysPostRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysPostUpdateReqDto>,
) -> Result<Json<ApiResponse<SysPostRecordVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:post:update").await?;
    let service = state.post_service();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SysPostRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysPostDeleteVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:post:delete").await?;
    let service = state.post_service();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SysPostDeleteVo { id, deleted })))
}
