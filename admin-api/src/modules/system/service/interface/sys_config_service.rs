use async_trait::async_trait;
use shaku::Interface;

use crate::core::{
    dto::sys_config_dto::{SysConfigCreateReqDto, SysConfigUpdateReqDto},
    errors::AppError,
    vo::sys_config_vo::{SysConfigListVo, SysConfigVo},
};

#[async_trait]
pub trait ISysConfigService: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<SysConfigListVo, AppError>;
    async fn create(&self, dto: SysConfigCreateReqDto) -> Result<SysConfigVo, AppError>;
    async fn update_by_id(
        &self,
        id: u64,
        dto: SysConfigUpdateReqDto,
    ) -> Result<SysConfigVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
