use async_trait::async_trait;
use shaku::Interface;

use crate::core::{errors::AppError, model::sys_dict::SysDictModel};

#[async_trait]
pub trait ISysDictRepository: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysDictModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysDictModel>, AppError>;
    async fn insert(&self, model: &SysDictModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, model: &SysDictModel) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
