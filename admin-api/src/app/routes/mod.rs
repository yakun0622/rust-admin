use axum::{extract::State, middleware, routing::get, Json, Router};
use serde_json::{json, Value};

use crate::{
    app::state::AppState,
    middleware::{auth::require_auth, request_log::log_api_request},
    modules,
};

pub fn build_router(state: AppState) -> Router {
    let protected_api = modules::protected_router()
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    Router::new()
        .route("/health", get(health))
        .nest("/api", modules::public_router())
        .nest("/api", protected_api)
        .layer(middleware::from_fn(log_api_request))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Json<Value> {
    let database_status = match state.db_ping().await {
        Ok(_) => (true, String::from("ok")),
        Err(err) => (false, err.to_string()),
    };

    let redis_status = match state.redis_ping().await {
        Ok(_) => (true, String::from("ok")),
        Err(err) => (false, err.to_string()),
    };

    let overall = if database_status.0 && redis_status.0 {
        "ok"
    } else {
        "degraded"
    };

    Json(json!({
        "status": overall,
        "service": "admin-api",
        "deps": {
            "database": {
                "driver": state.db_pool.driver_name(),
                "ok": database_status.0,
                "message": database_status.1
            },
            "redis": {
                "ok": redis_status.0,
                "message": redis_status.1
            }
        }
    }))
}
