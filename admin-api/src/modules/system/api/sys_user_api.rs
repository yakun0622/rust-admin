use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::sys_user_dto::{SysUserCreateReqDto, SysUserListQueryDto, SysUserUpdateReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::sys_user_vo::{SysUserDeleteVo, SysUserListVo, SysUserRecordVo},
    },
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
    Query(query): Query<SysUserListQueryDto>,
) -> Result<Json<ApiResponse<SysUserListVo>>, AppError> {
    let service = state.sys_user_service.clone();
    let items = service.list(query.keyword.as_deref()).await?;
    let total = items.len();
    Ok(Json(ApiResponse::success(SysUserListVo { total, items })))
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<SysUserCreateReqDto>,
) -> Result<Json<ApiResponse<SysUserRecordVo>>, AppError> {
    let service = state.sys_user_service.clone();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SysUserRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<SysUserUpdateReqDto>,
) -> Result<Json<ApiResponse<SysUserRecordVo>>, AppError> {
    let service = state.sys_user_service.clone();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SysUserRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysUserDeleteVo>>, AppError> {
    let service = state.sys_user_service.clone();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SysUserDeleteVo { id, deleted })))
}
