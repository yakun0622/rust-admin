use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::monitor_dto::{JobLogQueryDto, JobUpsertReqDto, MonitorListQueryDto},
        errors::AppError,
        response::ApiResponse,
        vo::monitor_vo::{JobActionVo, JobItemVo, JobListVo, JobLogListVo},
    },
    middleware::auth::ensure_permission,
};

pub struct SysJobRouter;

impl SysJobRouter {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/job", get(list_jobs).post(create_job))
            .route("/job/log", get(list_job_logs))
            .route("/job/{id}", put(update_job).delete(delete_job))
            .route("/job/{id}/run", post(run_job))
            .route("/job/{id}/pause", post(pause_job))
            .route("/job/{id}/resume", post(resume_job))
    }
}

async fn list_jobs(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<MonitorListQueryDto>,
) -> Result<Json<ApiResponse<JobListVo>>, AppError> {
    ensure_permission(&state, &current_user, "monitor:job:view").await?;
    let data = state
        .sys_job_service
        .list_jobs(query.keyword.as_deref())
        .await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn list_job_logs(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<JobLogQueryDto>,
) -> Result<Json<ApiResponse<JobLogListVo>>, AppError> {
    ensure_permission(&state, &current_user, "monitor:job:view").await?;
    let data = state.sys_job_service.list_job_logs(query).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn create_job(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<JobUpsertReqDto>,
) -> Result<Json<ApiResponse<JobItemVo>>, AppError> {
    ensure_permission(&state, &current_user, "monitor:job:create").await?;
    let data = state.sys_job_service.create_job(payload).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn update_job(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
    Json(payload): Json<JobUpsertReqDto>,
) -> Result<Json<ApiResponse<JobItemVo>>, AppError> {
    ensure_permission(&state, &current_user, "monitor:job:update").await?;
    let data = state.sys_job_service.update_job(id, payload).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn delete_job(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<JobActionVo>>, AppError> {
    ensure_permission(&state, &current_user, "monitor:job:delete").await?;
    let data = state.sys_job_service.delete_job(id).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn run_job(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<JobActionVo>>, AppError> {
    ensure_permission(&state, &current_user, "monitor:job:run").await?;
    let data = state.sys_job_service.run_job_once(id).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn pause_job(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<JobActionVo>>, AppError> {
    ensure_permission(&state, &current_user, "monitor:job:pause").await?;
    let data = state.sys_job_service.pause_job(id).await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn resume_job(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<u64>,
) -> Result<Json<ApiResponse<JobActionVo>>, AppError> {
    ensure_permission(&state, &current_user, "monitor:job:resume").await?;
    let data = state.sys_job_service.resume_job(id).await?;
    Ok(Json(ApiResponse::success(data)))
}
