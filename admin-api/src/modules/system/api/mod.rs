use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};
use serde_json::Value;

use crate::{
    app::state::AppState,
    core::{
        dto::system::SystemListQueryDto,
        errors::AppError,
        response::ApiResponse,
        vo::system::{SystemCrudDeleteVo, SystemCrudListVo, SystemCrudRecordVo},
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{resource}", get(list).post(create))
        .route("/{resource}/{id}", put(update).delete(remove))
}

async fn list(
    State(state): State<AppState>,
    Path(resource): Path<String>,
    Query(query): Query<SystemListQueryDto>,
) -> Result<Json<ApiResponse<SystemCrudListVo>>, AppError> {
    let data = state
        .system_service
        .list(&resource, query.keyword.as_deref())
        .await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create(
    State(state): State<AppState>,
    Path(resource): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ApiResponse<SystemCrudRecordVo>>, AppError> {
    let item = state.system_service.create(&resource, payload).await?;
    Ok(Json(ApiResponse::success(SystemCrudRecordVo { item })))
}

async fn update(
    State(state): State<AppState>,
    Path((resource, id)): Path<(String, u64)>,
    Json(payload): Json<Value>,
) -> Result<Json<ApiResponse<SystemCrudRecordVo>>, AppError> {
    let item = state.system_service.update(&resource, id, payload).await?;
    Ok(Json(ApiResponse::success(SystemCrudRecordVo { item })))
}

async fn remove(
    State(state): State<AppState>,
    Path((resource, id)): Path<(String, u64)>,
) -> Result<Json<ApiResponse<SystemCrudDeleteVo>>, AppError> {
    let deleted = state.system_service.delete(&resource, id).await?;
    Ok(Json(ApiResponse::success(SystemCrudDeleteVo { id, deleted })))
}
