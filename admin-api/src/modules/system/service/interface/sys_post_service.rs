use async_trait::async_trait;
use shaku::Interface;

use crate::core::{
    dto::sys_post_dto::{SysPostCreateReqDto, SysPostUpdateReqDto},
    errors::AppError,
    vo::sys_post_vo::{SysPostListVo, SysPostVo},
};

#[async_trait]
pub trait ISysPostService: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<SysPostListVo, AppError>;
    async fn create(&self, dto: SysPostCreateReqDto) -> Result<SysPostVo, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysPostUpdateReqDto) -> Result<SysPostVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
