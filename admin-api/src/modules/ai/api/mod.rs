mod message_api;
mod session_api;

use axum::Router;

use crate::app::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(session_api::routes())
        .merge(message_api::routes())
}
