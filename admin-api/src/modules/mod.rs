use axum::Router;

use crate::app::state::AppState;

pub mod ai;
pub mod auth;
pub mod dashboard;
pub mod log;
pub mod monitor;
pub mod system;

pub fn public_router() -> Router<AppState> {
    Router::new().nest("/auth", auth::api::router())
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .nest("/dashboard", dashboard::api::router())
        .nest("/system", system::api::router())
        .nest("/log", log::api::router())
        .nest("/monitor", monitor::api::router())
        .nest("/ai", ai::api::router())
}
