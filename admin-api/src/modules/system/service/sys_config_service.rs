use std::sync::Arc;

use async_trait::async_trait;
use shaku::Component;

use crate::core::{
    converter::sys_config_converter::{from_create_dto, from_update_dto, to_sys_config_vo},
    dto::sys_config_dto::{SysConfigCreateReqDto, SysConfigUpdateReqDto},
    errors::AppError,
    vo::sys_config_vo::{SysConfigListVo, SysConfigVo},
};
use crate::modules::system::repository::interface::ISysConfigRepository;

use super::interface::ISysConfigService;

#[derive(Component, Clone)]
#[shaku(interface = ISysConfigService)]
pub struct SysConfigService {
    #[shaku(inject)]
    repo: Arc<dyn ISysConfigRepository>,
}

impl SysConfigService {
    pub async fn list(&self, keyword: Option<&str>) -> Result<SysConfigListVo, AppError> {
        let items = self
            .repo
            .list(keyword.and_then(normalize_optional_str))
            .await?
            .into_iter()
            .map(to_sys_config_vo)
            .collect::<Vec<_>>();

        Ok(SysConfigListVo {
            total: items.len(),
            items,
        })
    }

    pub async fn create(&self, dto: SysConfigCreateReqDto) -> Result<SysConfigVo, AppError> {
        let model = from_create_dto(dto)?;
        let id = self.repo.insert(&model).await?;
        let created = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("创建成功但读取配置失败: id={id}")))?;
        Ok(to_sys_config_vo(created))
    }

    pub async fn update_by_id(
        &self,
        id: u64,
        dto: SysConfigUpdateReqDto,
    ) -> Result<SysConfigVo, AppError> {
        let model = from_update_dto(id, dto)?;
        let affected = self.repo.update_by_id(id, &model).await?;
        if !affected {
            return Err(AppError::not_found(format!(
                "config 资源中不存在 id={id} 的记录"
            )));
        }
        let updated = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("更新成功但读取配置失败: id={id}")))?;
        Ok(to_sys_config_vo(updated))
    }

    pub async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let deleted = self.repo.delete_by_id(id).await?;
        if !deleted {
            return Err(AppError::not_found(format!(
                "config 资源中不存在 id={id} 的记录"
            )));
        }
        Ok(true)
    }
}

#[async_trait]
impl ISysConfigService for SysConfigService {
    async fn list(&self, keyword: Option<&str>) -> Result<SysConfigListVo, AppError> {
        self.list(keyword).await
    }

    async fn create(&self, dto: SysConfigCreateReqDto) -> Result<SysConfigVo, AppError> {
        self.create(dto).await
    }

    async fn update_by_id(
        &self,
        id: u64,
        dto: SysConfigUpdateReqDto,
    ) -> Result<SysConfigVo, AppError> {
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
