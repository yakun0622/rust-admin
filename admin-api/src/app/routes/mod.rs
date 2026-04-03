use axum::{extract::State, middleware, routing::get, Json, Router};
use serde_json::{json, Value};

use crate::{app::state::AppState, middleware::auth::require_auth, modules};

pub fn build_router(state: AppState) -> Router {
    let protected_api = modules::protected_router().route_layer(middleware::from_fn_with_state(
        state.clone(),
        require_auth,
    ));

    Router::new()
        .route("/health", get(health))
        .nest("/api", modules::public_router())
        .nest("/api", protected_api)
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Json<Value> {
    let mysql_status = match state.mysql_ping().await {
        Ok(_) => (true, String::from("ok")),
        Err(err) => (false, err.to_string()),
    };

    let redis_status = match state.redis_ping().await {
        Ok(_) => (true, String::from("ok")),
        Err(err) => (false, err.to_string()),
    };

    let overall = if mysql_status.0 && redis_status.0 {
        "ok"
    } else {
        "degraded"
    };

    Json(json!({
        "status": overall,
        "service": "admin-api",
        "deps": {
            "mysql": {
                "ok": mysql_status.0,
                "message": mysql_status.1
            },
            "redis": {
                "ok": redis_status.0,
                "message": redis_status.1
            }
        }
    }))
}
