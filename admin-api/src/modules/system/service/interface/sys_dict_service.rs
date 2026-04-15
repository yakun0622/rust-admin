use async_trait::async_trait;
use shaku::Interface;

use crate::core::{
    dto::sys_dict_dto::{SysDictCreateReqDto, SysDictUpdateReqDto},
    errors::AppError,
    vo::sys_dict_vo::{SysDictListVo, SysDictVo},
};

#[async_trait]
pub trait ISysDictService: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<SysDictListVo, AppError>;
    async fn create(&self, dto: SysDictCreateReqDto) -> Result<SysDictVo, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysDictUpdateReqDto) -> Result<SysDictVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}
