use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::core::{
    converter::sys_dict_converter::{from_create_dto, from_update_dto, to_sys_dict_vo},
    dto::sys_dict_dto::{SysDictCreateReqDto, SysDictUpdateReqDto},
    errors::AppError,
    vo::sys_dict_vo::{SysDictListVo, SysDictVo},
};
use crate::modules::system::repository::ISysDictRepository;

#[async_trait]
pub trait ISysDictService: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<SysDictListVo, AppError>;
    async fn create(&self, dto: SysDictCreateReqDto) -> Result<SysDictVo, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysDictUpdateReqDto) -> Result<SysDictVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysDictService)]
pub struct SysDictService {
    #[shaku(inject)]
    repo: Arc<dyn ISysDictRepository>,
}

impl SysDictService {
    pub async fn list(&self, keyword: Option<&str>) -> Result<SysDictListVo, AppError> {
        let items = self
            .repo
            .list(keyword.and_then(normalize_optional_str))
            .await?
            .into_iter()
            .map(to_sys_dict_vo)
            .collect::<Vec<_>>();

        Ok(SysDictListVo {
            total: items.len(),
            items,
        })
    }

    pub async fn create(&self, dto: SysDictCreateReqDto) -> Result<SysDictVo, AppError> {
        let model = from_create_dto(dto)?;
        let id = self.repo.insert(&model).await?;
        let created = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("创建成功但读取字典失败: id={id}")))?;
        Ok(to_sys_dict_vo(created))
    }

    pub async fn update_by_id(
        &self,
        id: u64,
        dto: SysDictUpdateReqDto,
    ) -> Result<SysDictVo, AppError> {
        let model = from_update_dto(id, dto)?;
        let affected = self.repo.update_by_id(id, &model).await?;
        if !affected {
            return Err(AppError::not_found(format!(
                "dict 资源中不存在 id={id} 的记录"
            )));
        }
        let updated = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("更新成功但读取字典失败: id={id}")))?;
        Ok(to_sys_dict_vo(updated))
    }

    pub async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let deleted = self.repo.delete_by_id(id).await?;
        if !deleted {
            return Err(AppError::not_found(format!(
                "dict 资源中不存在 id={id} 的记录"
            )));
        }
        Ok(true)
    }
}

#[async_trait]
impl ISysDictService for SysDictService {
    async fn list(&self, keyword: Option<&str>) -> Result<SysDictListVo, AppError> {
        self.list(keyword).await
    }

    async fn create(&self, dto: SysDictCreateReqDto) -> Result<SysDictVo, AppError> {
        self.create(dto).await
    }

    async fn update_by_id(&self, id: u64, dto: SysDictUpdateReqDto) -> Result<SysDictVo, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}

fn normalize_optional_str(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
