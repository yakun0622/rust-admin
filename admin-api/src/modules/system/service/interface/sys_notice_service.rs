use async_trait::async_trait;
use serde_json::Value;
use shaku::Interface;

use crate::core::{
    dto::system_dto::{SysNoticeCreateReqDto, SysNoticeUpdateReqDto},
    errors::AppError,
    vo::system_vo::SystemCrudListVo,
};

#[async_trait]
pub trait ISysNoticeService: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<SystemCrudListVo, AppError>;
    async fn create(&self, dto: SysNoticeCreateReqDto) -> Result<Value, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysNoticeUpdateReqDto) -> Result<Value, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
