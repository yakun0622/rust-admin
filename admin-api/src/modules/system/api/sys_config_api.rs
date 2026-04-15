use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::sys_config_dto::{
            SysConfigCreateReqDto, SysConfigListQueryDto, SysConfigUpdateReqDto,
        },
        errors::AppError,
        response::ApiResponse,
        vo::sys_config_vo::{SysConfigDeleteVo, SysConfigListVo, SysConfigRecordVo},
    },
};

pub struct SysConfigRouter;

impl SysConfigRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/config", get(list).post(create))
            .route("/config/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    Query(query): Query<SysConfigListQueryDto>,
) -> Result<Json<ApiResponse<SysConfigListVo>>, AppError> {
    let service = state.config_service();
    let data = service.list(query.keyword.as_deref()).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<SysConfigCreateReqDto>,
) -> Result<Json<ApiResponse<SysConfigRecordVo>>, AppError> {
    let service = state.config_service();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SysConfigRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<SysConfigUpdateReqDto>,
) -> Result<Json<ApiResponse<SysConfigRecordVo>>, AppError> {
    let service = state.config_service();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SysConfigRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysConfigDeleteVo>>, AppError> {
    let service = state.config_service();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SysConfigDeleteVo {
        id,
        deleted,
    })))
}
