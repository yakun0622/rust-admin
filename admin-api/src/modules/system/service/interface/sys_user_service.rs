use async_trait::async_trait;
use shaku::Interface;

use crate::core::{
    dto::sys_user_dto::{SysUserCreateReqDto, SysUserUpdateReqDto},
    errors::AppError,
    vo::sys_user_vo::SysUserVo,
};

#[async_trait]
pub trait ISysUserService: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysUserVo>, AppError>;
    async fn create(&self, dto: SysUserCreateReqDto) -> Result<SysUserVo, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysUserUpdateReqDto) -> Result<SysUserVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
