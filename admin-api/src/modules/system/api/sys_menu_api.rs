use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
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
            .route("/menu/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    Query(query): Query<SysMenuListQueryDto>,
) -> Result<Json<ApiResponse<SysMenuListVo>>, AppError> {
    let service = state.sys_menu_service.clone();
    let data = service.list(query.keyword.as_deref()).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<SysMenuCreateReqDto>,
) -> Result<Json<ApiResponse<SysMenuRecordVo>>, AppError> {
    let service = state.sys_menu_service.clone();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SysMenuRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<SysMenuUpdateReqDto>,
) -> Result<Json<ApiResponse<SysMenuRecordVo>>, AppError> {
    let service = state.sys_menu_service.clone();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SysMenuRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysMenuDeleteVo>>, AppError> {
    let service = state.sys_menu_service.clone();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SysMenuDeleteVo { id, deleted })))
}
