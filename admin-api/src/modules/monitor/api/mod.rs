mod cache_api;
mod online_api;
mod overview_api;

use axum::Router;

use crate::app::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(online_api::routes())
        .merge(overview_api::routes())
        .merge(cache_api::routes())
        .merge(crate::modules::system::api::job_router())
}
