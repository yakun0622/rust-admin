use async_trait::async_trait;
use shaku::Interface;

use crate::core::{
    dto::sys_menu_dto::{SysMenuCreateReqDto, SysMenuUpdateReqDto},
    errors::AppError,
    vo::sys_menu_vo::{SysMenuListVo, SysMenuVo},
};

#[async_trait]
pub trait ISysMenuService: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<SysMenuListVo, AppError>;
    async fn create(&self, dto: SysMenuCreateReqDto) -> Result<SysMenuVo, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysMenuUpdateReqDto) -> Result<SysMenuVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
