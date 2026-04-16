use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};

use crate::core::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: u64,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Clone)]
pub struct CurrentUser(pub JwtClaims);

impl CurrentUser {
    pub fn user_id(&self) -> u64 {
        self.0.sub
    }

    pub fn username(&self) -> &str {
        &self.0.username
    }

    pub fn claims(&self) -> &JwtClaims {
        &self.0
    }
}

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let claims = parts
            .extensions
            .get::<JwtClaims>()
            .cloned();

        async move {
            claims
                .map(CurrentUser)
                .ok_or_else(|| AppError::unauthorized("未登录或登录态已失效"))
        }
    }
}
