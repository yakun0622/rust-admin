use async_trait::async_trait;
use shaku::Interface;

use crate::core::{errors::AppError, model::system::SysNoticePo};

#[async_trait]
pub trait ISysNoticeRepository: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysNoticePo>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysNoticePo>, AppError>;
    async fn insert(
        &self,
        title: &str,
        notice_type: i16,
        status: i16,
        publisher: Option<&str>,
    ) -> Result<u64, AppError>;
    async fn update_by_id(
        &self,
        id: u64,
        title: &str,
        notice_type: i16,
        status: i16,
        publisher: Option<&str>,
    ) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
