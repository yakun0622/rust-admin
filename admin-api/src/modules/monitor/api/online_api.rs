use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        dto::monitor_dto::MonitorListQueryDto, response::ApiResponse, vo::monitor_vo::OnlineUserListVo,
    },
};

pub(super) fn routes() -> Router<AppState> {
    Router::new().route("/online", get(list_online))
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
