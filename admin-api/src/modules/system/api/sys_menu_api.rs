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
    crate::permission!(state, current_user, "system:menu:view");
    let service = state.menu_service();
    let data = service.list(query).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<SysMenuCreateReqDto>,
) -> Result<Json<ApiResponse<SysMenuRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:menu:create");
    let service = state.menu_service();
    let item = crate::admin_log!(state, current_user, "创建菜单", 1_i8, async move {
        service.create(payload).await
    })?;
    Ok(Json(ApiResponse::success(SysMenuRecordVo { item })))
}

async fn list_tree(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<SysMenuListQueryDto>,
) -> Result<Json<ApiResponse<SysMenuListVo>>, AppError> {
    crate::permission!(state, current_user, "system:menu:view");
    let service = state.menu_service();
    let data = service.list_tree(query).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn update(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<SysMenuUpdateReqDto>,
) -> Result<Json<ApiResponse<SysMenuRecordVo>>, AppError> {
    crate::permission!(state, current_user, "system:menu:update");
    let service = state.menu_service();
    let item = crate::admin_log!(state, current_user, "修改菜单", 2_i8, async move {
        service.update_by_id(id, payload).await
    })?;
    Ok(Json(ApiResponse::success(SysMenuRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysMenuDeleteVo>>, AppError> {
    crate::permission!(state, current_user, "system:menu:delete");
    let service = state.menu_service();
    let deleted = crate::admin_log!(state, current_user, "删除菜单", 3_i8, async move {
        service.delete_by_id(id).await
    })?;
    Ok(Json(ApiResponse::success(SysMenuDeleteVo { id, deleted })))
}
