use async_trait::async_trait;
use shaku::Interface;

use crate::core::{
    dto::sys_dept_dto::{SysDeptCreateReqDto, SysDeptUpdateReqDto},
    errors::AppError,
    vo::sys_dept_vo::{SysDeptListVo, SysDeptVo},
};

#[async_trait]
pub trait ISysDeptService: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<SysDeptListVo, AppError>;
    async fn create(&self, dto: SysDeptCreateReqDto) -> Result<SysDeptVo, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysDeptUpdateReqDto) -> Result<SysDeptVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
