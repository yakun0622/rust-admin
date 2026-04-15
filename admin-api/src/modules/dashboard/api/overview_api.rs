use axum::{extract::State, routing::get, Json, Router};

use crate::{app::state::AppState, core::response::ApiResponse};

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/overview", get(overview))
}

async fn overview(
    State(state): State<AppState>,
) -> Json<ApiResponse<crate::core::vo::dashboard_vo::DashboardOverviewVo>> {
    Json(ApiResponse::success(state.dashboard_service.overview()))
}
