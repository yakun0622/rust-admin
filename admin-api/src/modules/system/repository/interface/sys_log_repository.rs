use async_trait::async_trait;
use shaku::Interface;

use crate::core::{
    errors::AppError,
    model::log::{LoginLogPo, OperLogPo},
};

#[async_trait]
pub trait ISysLogRepository: Interface {
    async fn list_oper(&self, keyword: Option<&str>) -> Result<Vec<OperLogPo>, AppError>;
    async fn list_login(&self, keyword: Option<&str>) -> Result<Vec<LoginLogPo>, AppError>;
}
