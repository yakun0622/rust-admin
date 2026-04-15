use async_trait::async_trait;
use shaku::Interface;

use crate::core::{
    dto::sys_role_dto::{SysRoleCreateReqDto, SysRoleUpdateReqDto},
    errors::AppError,
    vo::sys_role_vo::{SysRoleListVo, SysRoleVo},
};

#[async_trait]
pub trait ISysRoleService: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<SysRoleListVo, AppError>;
    async fn create(&self, dto: SysRoleCreateReqDto) -> Result<SysRoleVo, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysRoleUpdateReqDto) -> Result<SysRoleVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
