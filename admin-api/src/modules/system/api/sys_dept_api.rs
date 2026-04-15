use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::sys_dept_dto::{SysDeptCreateReqDto, SysDeptListQueryDto, SysDeptUpdateReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::sys_dept_vo::{SysDeptDeleteVo, SysDeptListVo, SysDeptRecordVo},
    },
};

pub struct SysDeptRouter;

impl SysDeptRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/dept", get(list).post(create))
            .route("/dept/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    Query(query): Query<SysDeptListQueryDto>,
) -> Result<Json<ApiResponse<SysDeptListVo>>, AppError> {
    let service = state.dept_service();
    let data = service.list(query.keyword.as_deref()).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<SysDeptCreateReqDto>,
) -> Result<Json<ApiResponse<SysDeptRecordVo>>, AppError> {
    let service = state.dept_service();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SysDeptRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<SysDeptUpdateReqDto>,
) -> Result<Json<ApiResponse<SysDeptRecordVo>>, AppError> {
    let service = state.dept_service();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SysDeptRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysDeptDeleteVo>>, AppError> {
    let service = state.dept_service();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SysDeptDeleteVo { id, deleted })))
}
