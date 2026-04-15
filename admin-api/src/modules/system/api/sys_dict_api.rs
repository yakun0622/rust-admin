use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::sys_dict_dto::{SysDictCreateReqDto, SysDictListQueryDto, SysDictUpdateReqDto},
        errors::AppError,
        response::ApiResponse,
        vo::sys_dict_vo::{SysDictDeleteVo, SysDictListVo, SysDictRecordVo},
    },
};

pub struct SysDictRouter;

impl SysDictRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/dict", get(list).post(create))
            .route("/dict/{id}", put(update).delete(remove))
    }
}

async fn list(
    State(state): State<AppState>,
    Query(query): Query<SysDictListQueryDto>,
) -> Result<Json<ApiResponse<SysDictListVo>>, AppError> {
    let service = state.dict_service();
    let data = service.list(query.keyword.as_deref()).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<SysDictCreateReqDto>,
) -> Result<Json<ApiResponse<SysDictRecordVo>>, AppError> {
    let service = state.dict_service();
    let item = service.create(payload).await?;
    Ok(Json(ApiResponse::success(SysDictRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<SysDictUpdateReqDto>,
) -> Result<Json<ApiResponse<SysDictRecordVo>>, AppError> {
    let service = state.dict_service();
    let item = service.update_by_id(id, payload).await?;
    Ok(Json(ApiResponse::success(SysDictRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<SysDictDeleteVo>>, AppError> {
    let service = state.dict_service();
    let deleted = service.delete_by_id(id).await?;
    Ok(Json(ApiResponse::success(SysDictDeleteVo { id, deleted })))
}
