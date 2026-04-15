use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::sys_role_dto::{SysRoleCreateReqDto, SysRoleListQueryDto, SysRoleUpdateReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::sys_role_vo::{SysRoleDeleteVo, SysRoleListVo, SysRoleRecordVo},
    },
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
    Query(query): Query<SysRoleListQueryDto>,
) -> Result<Json<ApiResponse<SysRoleListVo>>, AppError> {
    let service = state.role_service();
    let data = service.list(query.keyword.as_deref()).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<SysRoleCreateReqDto>,
) -> Result<Json<ApiResponse<SysRoleRecordVo>>, AppError> {
    let service = state.role_service();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SysRoleRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<SysRoleUpdateReqDto>,
) -> Result<Json<ApiResponse<SysRoleRecordVo>>, AppError> {
    let service = state.role_service();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SysRoleRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysRoleDeleteVo>>, AppError> {
    let service = state.role_service();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SysRoleDeleteVo { id, deleted })))
}
