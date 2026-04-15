use async_trait::async_trait;
use shaku::Interface;

use crate::core::{errors::AppError, model::auth::UserCredentialPo};

#[async_trait]
pub trait ISysAuthRepository: Interface {
    async fn find_by_username(&self, username: &str) -> Result<Option<UserCredentialPo>, AppError>;

    async fn append_login_log(
        &self,
        username: Option<&str>,
        login_type: i8,
        status: i8,
        message: &str,
        ip: &str,
    ) -> Result<(), AppError>;
}
