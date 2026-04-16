use axum::{
    extract::State,
    http::HeaderMap,
    routing::{get, post},
    Json, Router,
};

use crate::{
    app::state::AppState,
    core::{
        common::CurrentUser,
        dto::auth_dto::LoginReqDto,
        errors::AppError,
        response::ApiResponse,
        vo::auth_vo::{AuthProfileVo, LoginVo},
    },
};

pub struct SysAuthRouter;

impl SysAuthRouter {
    pub fn public_routes() -> Router<AppState> {
        Router::new().route("/auth/login", post(login))
    }

    pub fn protected_routes() -> Router<AppState> {
        Router::new().route("/auth/profile", get(profile))
    }
}

async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginReqDto>,
) -> Result<Json<ApiResponse<LoginVo>>, AppError> {
    let service = state.auth_service();

    let login_vo = service.login(payload, extract_client_ip(&headers)).await?;
    Ok(Json(ApiResponse::success(login_vo)))
}

fn extract_client_ip(headers: &HeaderMap) -> Option<String> {
    if let Some(value) = headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
    {
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

async fn profile(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<ApiResponse<AuthProfileVo>>, AppError> {
    let data = state.auth_service().profile(current_user.user_id()).await?;
    Ok(Json(ApiResponse::success(data)))
}
