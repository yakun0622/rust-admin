use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::monitor_dto::{JobUpsertReqDto, MonitorListQueryDto},
        errors::AppError,
        response::ApiResponse,
        vo::monitor_vo::{JobActionVo, JobItemVo, JobListVo},
    },
};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/job", get(list_jobs).post(create_job))
        .route("/job/{id}", put(update_job).delete(delete_job))
        .route("/job/{id}/run", post(run_job))
        .route("/job/{id}/pause", post(pause_job))
        .route("/job/{id}/resume", post(resume_job))
}

async fn list_jobs(
    State(state): State<AppState>,
    Query(query): Query<MonitorListQueryDto>,
) -> Json<ApiResponse<JobListVo>> {
    Json(ApiResponse::success(
        state
            .monitor_service
            .list_jobs(query.keyword.as_deref())
            .await,
    ))
}

async fn create_job(
    State(state): State<AppState>,
    Json(payload): Json<JobUpsertReqDto>,
) -> Result<Json<ApiResponse<JobItemVo>>, AppError> {
    let data = state.monitor_service.create_job(payload).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn update_job(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<JobUpsertReqDto>,
) -> Result<Json<ApiResponse<JobItemVo>>, AppError> {
    let data = state.monitor_service.update_job(id, payload).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn delete_job(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<JobActionVo>>, AppError> {
    let data = state.monitor_service.delete_job(id).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn run_job(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<JobActionVo>>, AppError> {
    let data = state.monitor_service.run_job_once(id).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn pause_job(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<JobActionVo>>, AppError> {
    let data = state.monitor_service.pause_job(id).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn resume_job(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<JobActionVo>>, AppError> {
    let data = state.monitor_service.resume_job(id).await?;
    Ok(Json(ApiResponse::success(data)))
}
