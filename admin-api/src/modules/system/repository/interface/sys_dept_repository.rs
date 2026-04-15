use async_trait::async_trait;
use shaku::Interface;

use crate::core::{errors::AppError, model::sys_dept::SysDeptModel};

#[async_trait]
pub trait ISysDeptRepository: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysDeptModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysDeptModel>, AppError>;
    async fn insert(&self, model: &SysDeptModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, model: &SysDeptModel) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
