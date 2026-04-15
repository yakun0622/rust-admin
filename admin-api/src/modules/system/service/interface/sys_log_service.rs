use async_trait::async_trait;
use shaku::Interface;

use crate::core::{
    errors::AppError,
    vo::log_vo::{LoginLogListVo, OperLogListVo},
};

#[async_trait]
pub trait ISysLogService: Interface {
    async fn list_oper(&self, keyword: Option<&str>) -> Result<OperLogListVo, AppError>;
    async fn list_login(&self, keyword: Option<&str>) -> Result<LoginLogListVo, AppError>;
}
