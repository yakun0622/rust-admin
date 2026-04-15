use axum::Router;

use crate::app::state::AppState;

pub mod ai;
pub mod dashboard;
pub mod monitor;
pub mod system;

pub fn public_router() -> Router<AppState> {
    Router::new().nest("/system", system::api::public_router())
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .nest("/dashboard", dashboard::api::router())
        .nest("/system", system::api::router())
        .nest("/log", system::api::log_router())
        .nest("/monitor", monitor::api::router())
        .nest("/ai", ai::api::router())
}
