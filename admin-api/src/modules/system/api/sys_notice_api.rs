use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::system_dto::{SysNoticeCreateReqDto, SysNoticeUpdateReqDto, SystemListQueryDto},
        errors::AppError,
        response::ApiResponse,
        vo::system_vo::{SystemCrudDeleteVo, SystemCrudListVo, SystemCrudRecordVo},
    },
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
    Query(query): Query<SystemListQueryDto>,
) -> Result<Json<ApiResponse<SystemCrudListVo>>, AppError> {
    let service = state.notice_service();
    let data = service.list(query.keyword.as_deref()).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<SysNoticeCreateReqDto>,
) -> Result<Json<ApiResponse<SystemCrudRecordVo>>, AppError> {
    let service = state.notice_service();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SystemCrudRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<SysNoticeUpdateReqDto>,
) -> Result<Json<ApiResponse<SystemCrudRecordVo>>, AppError> {
    let service = state.notice_service();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SystemCrudRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SystemCrudDeleteVo>>, AppError> {
    let service = state.notice_service();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SystemCrudDeleteVo {
        id,
        deleted,
    })))
}
