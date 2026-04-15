use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::monitor_dto::CacheSearchQueryDto,
        errors::AppError,
        response::ApiResponse,
        vo::monitor_vo::{CacheNamespaceListVo, CacheSearchVo},
    },
};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/cache", get(cache_search))
        .route("/cache-list", get(cache_list))
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
