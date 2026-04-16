use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::sys_menu_dto::{SysMenuCreateReqDto, SysMenuListQueryDto, SysMenuUpdateReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::sys_menu_vo::{SysMenuDeleteVo, SysMenuListVo, SysMenuRecordVo},
    },
    middleware::auth::ensure_permission,
};

pub struct SysMenuRouter;

impl SysMenuRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/menu", get(list).post(create))
            .route("/menu/tree", get(list_tree))
            .route("/menu/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SysMenuListQueryDto>,
) -> Result<Json<ApiResponse<SysMenuListVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:menu:view").await?;
    let service = state.menu_service();
    let data = service.list(query.keyword.as_deref()).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysMenuCreateReqDto>,
) -> Result<Json<ApiResponse<SysMenuRecordVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:menu:create").await?;
    let service = state.menu_service();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SysMenuRecordVo { item })))
}

async fn list_tree(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SysMenuListQueryDto>,
) -> Result<Json<ApiResponse<SysMenuListVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:menu:view").await?;
    let service = state.menu_service();
    let data = service.list_tree(query.keyword.as_deref()).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysMenuUpdateReqDto>,
) -> Result<Json<ApiResponse<SysMenuRecordVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:menu:update").await?;
    let service = state.menu_service();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SysMenuRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysMenuDeleteVo>>, AppError> {
    ensure_permission(&state, &current_user, "system:menu:delete").await?;
    let service = state.menu_service();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SysMenuDeleteVo { id, deleted })))
}
