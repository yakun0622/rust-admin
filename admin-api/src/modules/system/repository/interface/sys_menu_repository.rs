use async_trait::async_trait;
use shaku::Interface;

use crate::core::{errors::AppError, model::sys_menu::SysMenuModel};

#[async_trait]
pub trait ISysMenuRepository: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysMenuModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysMenuModel>, AppError>;
    async fn insert(&self, model: &SysMenuModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, model: &SysMenuModel) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
