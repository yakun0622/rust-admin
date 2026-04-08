use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::monitor::{CacheSearchQueryDto, JobUpsertReqDto, MonitorListQueryDto},
        errors::AppError,
        response::ApiResponse,
        vo::monitor::{
            CacheNamespaceListVo, CacheSearchVo, DatasourceOverviewVo, JobActionVo, JobItemVo,
            JobListVo, OnlineUserListVo, ServerOverviewVo,
        },
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/online", get(list_online))
        .route("/job", get(list_jobs).post(create_job))
        .route("/job/{id}", put(update_job).delete(delete_job))
        .route("/job/{id}/run", post(run_job))
        .route("/job/{id}/pause", post(pause_job))
        .route("/job/{id}/resume", post(resume_job))
        .route("/datasource", get(datasource))
        .route("/server", get(server))
        .route("/cache", get(cache_search))
        .route("/cache-list", get(cache_list))
}

async fn list_online(
    State(state): State<AppState>,
    Query(query): Query<MonitorListQueryDto>,
) -> Json<ApiResponse<OnlineUserListVo>> {
    Json(ApiResponse::success(
        state
            .monitor_service
            .list_online_users(query.keyword.as_deref())
            .await,
    ))
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

async fn datasource(State(state): State<AppState>) -> Json<ApiResponse<DatasourceOverviewVo>> {
    Json(ApiResponse::success(
        state.monitor_service.datasource_overview().await,
    ))
}

async fn server(State(state): State<AppState>) -> Json<ApiResponse<ServerOverviewVo>> {
    Json(ApiResponse::success(
        state.monitor_service.server_overview().await,
    ))
}

async fn cache_search(
    State(state): State<AppState>,
    Query(query): Query<CacheSearchQueryDto>,
) -> Result<Json<ApiResponse<CacheSearchVo>>, AppError> {
    let limit = query.limit.unwrap_or(50);
    let data = state
        .monitor_service
        .search_cache(query.keyword.as_deref(), limit)
        .await?;
    Ok(Json(ApiResponse::success(data)))
}

async fn cache_list(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<CacheNamespaceListVo>>, AppError> {
    let data = state.monitor_service.cache_namespace_list().await?;
    Ok(Json(ApiResponse::success(data)))
}
