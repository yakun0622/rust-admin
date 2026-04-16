use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::core::{
    converter::sys_config_converter::{from_create_dto, to_sys_config_vo},
    dto::sys_config_dto::{SysConfigCreateReqDto, SysConfigListQueryDto, SysConfigUpdateReqDto},
    errors::AppError,
    vo::sys_config_vo::{SysConfigListVo, SysConfigVo},
};
use crate::modules::system::repository::ISysConfigRepository;

#[async_trait]
pub trait ISysConfigService: Interface {
    async fn list(&self, query: SysConfigListQueryDto) -> Result<SysConfigListVo, AppError>;
    async fn create(&self, dto: SysConfigCreateReqDto) -> Result<SysConfigVo, AppError>;
    async fn update_by_id(
        &self,
        id: u64,
        dto: SysConfigUpdateReqDto,
    ) -> Result<SysConfigVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysConfigService)]
pub struct SysConfigService {
    #[shaku(inject)]
    repo: Arc<dyn ISysConfigRepository>,
}

impl SysConfigService {
    pub async fn list(&self, query: SysConfigListQueryDto) -> Result<SysConfigListVo, AppError> {
        let items = self
            .repo
            .list(query)
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
        let normalized = normalize_update_dto(dto)?;
        let affected = self.repo.update_by_id(id, normalized).await?;
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
    async fn list(&self, query: SysConfigListQueryDto) -> Result<SysConfigListVo, AppError> {
        self.list(query).await
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

fn normalize_update_dto(mut dto: SysConfigUpdateReqDto) -> Result<SysConfigUpdateReqDto, AppError> {
    dto.name = match dto.name {
        Some(name) => {
            let normalized = name.trim().to_string();
            if normalized.is_empty() {
                return Err(AppError::bad_request("参数名称不能为空"));
            }
            Some(normalized)
        }
        None => None,
    };

    dto.value = match dto.value {
        Some(value) => {
            let normalized = value.trim().to_string();
            if normalized.is_empty() {
                return Err(AppError::bad_request("参数值不能为空"));
            }
            Some(normalized)
        }
        None => None,
    };

    dto.remark = dto.remark.and_then(|remark| {
        let normalized = remark.trim().to_string();
        if normalized.is_empty() {
            None
        } else {
            Some(normalized)
        }
    });

    dto.status = match dto.status {
        Some(status) => {
            let normalized = status.trim().to_ascii_lowercase();
            match normalized.as_str() {
                "enabled" | "1" => Some("enabled".to_string()),
                "disabled" | "0" => Some("disabled".to_string()),
                _ => {
                    return Err(AppError::bad_request(
                        "状态值非法，仅支持 enabled/disabled/1/0",
                    ));
                }
            }
        }
        None => None,
    };

    Ok(dto)
}
