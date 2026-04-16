use axum::{extract::State, routing::get, Json, Router};

use crate::{
    app::state::AppState,
    core::{
        response::ApiResponse,
        vo::monitor_vo::{DatasourceOverviewVo, ServerOverviewVo},
    },
};

pub(super) fn routes() -> Router<AppState> {
    Router::new()
        .route("/datasource", get(datasource))
        .route("/server", get(server))
}

async fn datasource(State(state): State<AppState>) -> Json<ApiResponse<DatasourceOverviewVo>> {
    Json(ApiResponse::success(
        state.monitor_overview_service.datasource_overview().await,
    ))
}

async fn server(State(state): State<AppState>) -> Json<ApiResponse<ServerOverviewVo>> {
    Json(ApiResponse::success(
        state.monitor_overview_service.server_overview().await,
    ))
}
