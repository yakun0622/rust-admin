use async_trait::async_trait;
use shaku::Interface;

use crate::{
    core::{dto::auth_dto::LoginReqDto, errors::AppError, vo::auth_vo::LoginVo},
    modules::system::service::JwtClaims,
};

#[async_trait]
pub trait ISysAuthService: Interface {
    async fn login(
        &self,
        payload: LoginReqDto,
        client_ip: Option<String>,
    ) -> Result<LoginVo, AppError>;

    fn verify_token(&self, token: &str) -> Result<JwtClaims, AppError>;
}
