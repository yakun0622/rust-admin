use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::sys_user_dto::{SysUserCreateReqDto, SysUserListQueryDto, SysUserUpdateReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::sys_user_vo::{SysUserDeleteVo, SysUserListVo, SysUserRecordVo},
    },
    middleware::auth::ensure_permission,
};

pub struct SysUserRouter;

impl SysUserRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/user", get(list).post(create))
            .route("/user/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SysUserListQueryDto>,
) -> Result<Json<ApiResponse<SysUserListVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:user:view").await?;
    let service = state.user_service();
    let items = service.list(query.keyword.as_deref()).await?;
    let total = items.len();
    Ok(Json(ApiResponse::success(SysUserListVo { total, items })))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysUserCreateReqDto>,
) -> Result<Json<ApiResponse<SysUserRecordVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:user:create").await?;
    let service = state.user_service();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SysUserRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysUserUpdateReqDto>,
) -> Result<Json<ApiResponse<SysUserRecordVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:user:update").await?;
    let service = state.user_service();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SysUserRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysUserDeleteVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:user:delete").await?;
    let service = state.user_service();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SysUserDeleteVo { id, deleted })))
}
