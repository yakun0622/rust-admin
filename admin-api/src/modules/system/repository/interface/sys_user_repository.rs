use async_trait::async_trait;
use shaku::Interface;

use crate::core::{errors::AppError, model::sys_user::SysUserModel};

#[async_trait]
pub trait ISysUserRepository: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysUserModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysUserModel>, AppError>;
    async fn insert(&self, model: &SysUserModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, model: &SysUserModel) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
