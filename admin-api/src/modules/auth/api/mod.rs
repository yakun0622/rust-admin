use axum::{extract::State, http::HeaderMap, routing::post, Json, Router};

use crate::{
    app::state::AppState,
    core::{dto::auth::LoginReqDto, errors::AppError, response::ApiResponse, vo::auth::LoginVo},
};

pub fn router() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginReqDto>,
) -> Result<Json<ApiResponse<LoginVo>>, AppError> {
    let login_vo = state
        .auth_service
        .login(payload, extract_client_ip(&headers))
        .await?;
    Ok(Json(ApiResponse::success(login_vo)))
}

fn extract_client_ip(headers: &HeaderMap) -> Option<String> {
    if let Some(value) = headers.get("x-forwarded-for").and_then(|value| value.to_str().ok()) {
        let first = value.split(',').next().unwrap_or("").trim();
        if !first.is_empty() {
            return Some(first.to_string());
        }
    }

    headers
        .get("x-real-ip")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}
