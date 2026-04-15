use async_trait::async_trait;
use shaku::Interface;

use crate::core::{errors::AppError, model::sys_config::SysConfigModel};

#[async_trait]
pub trait ISysConfigRepository: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysConfigModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysConfigModel>, AppError>;
    async fn insert(&self, model: &SysConfigModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, model: &SysConfigModel) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
