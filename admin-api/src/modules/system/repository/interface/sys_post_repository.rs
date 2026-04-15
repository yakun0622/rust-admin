use async_trait::async_trait;
use shaku::Interface;

use crate::core::{errors::AppError, model::sys_post::SysPostModel};

#[async_trait]
pub trait ISysPostRepository: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysPostModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysPostModel>, AppError>;
    async fn insert(&self, model: &SysPostModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, model: &SysPostModel) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
