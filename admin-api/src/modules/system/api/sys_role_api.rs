use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::sys_role_dto::{SysRoleCreateReqDto, SysRoleListQueryDto, SysRoleUpdateReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::sys_role_vo::{SysRoleDeleteVo, SysRoleListVo, SysRoleRecordVo},
    },
    middleware::auth::ensure_permission,
};

pub struct SysRoleRouter;

impl SysRoleRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/role", get(list).post(create))
            .route("/role/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SysRoleListQueryDto>,
) -> Result<Json<ApiResponse<SysRoleListVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:role:view").await?;
    let service = state.role_service();
    let data = service.list(query.keyword.as_deref()).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysRoleCreateReqDto>,
) -> Result<Json<ApiResponse<SysRoleRecordVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:role:create").await?;
    let service = state.role_service();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SysRoleRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysRoleUpdateReqDto>,
) -> Result<Json<ApiResponse<SysRoleRecordVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:role:update").await?;
    let service = state.role_service();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SysRoleRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysRoleDeleteVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:role:delete").await?;
    let service = state.role_service();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SysRoleDeleteVo { id, deleted })))
}
