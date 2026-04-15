mod overview_api;

use axum::Router;

use crate::app::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().merge(overview_api::routes())
}
