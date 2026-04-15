use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

use crate::{app::state::AppState, core::errors::AppError, modules::system::service::JwtClaims};

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::unauthorized("缺少 Authorization 头"))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::unauthorized("Authorization 格式错误"))?;

    let claims = state.auth_service().verify_token(token)?;
    request.extensions_mut().insert::<JwtClaims>(claims);

    Ok(next.run(request).await)
}
