use async_trait::async_trait;
use shaku::Interface;

use crate::core::{errors::AppError, model::sys_role::SysRoleModel};

#[async_trait]
pub trait ISysRoleRepository: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysRoleModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysRoleModel>, AppError>;
    async fn insert(&self, model: &SysRoleModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, model: &SysRoleModel) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
