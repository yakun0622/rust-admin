use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::core::{
    converter::sys_dict_converter::{from_create_dto, to_sys_dict_vo},
    dto::sys_dict_dto::{SysDictCreateReqDto, SysDictListQueryDto, SysDictUpdateReqDto},
    errors::AppError,
    vo::sys_dict_vo::{SysDictListVo, SysDictVo},
};
use crate::modules::system::repository::ISysDictRepository;

#[async_trait]
pub trait ISysDictService: Interface {
    async fn list(&self, query: SysDictListQueryDto) -> Result<SysDictListVo, AppError>;
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
    pub async fn list(&self, query: SysDictListQueryDto) -> Result<SysDictListVo, AppError> {
        let items = self
            .repo
            .list(query)
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
        let normalized = normalize_update_dto(dto)?;
        let affected = self.repo.update_by_id(id, normalized).await?;
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
    async fn list(&self, query: SysDictListQueryDto) -> Result<SysDictListVo, AppError> {
        self.list(query).await
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

fn normalize_update_dto(mut dto: SysDictUpdateReqDto) -> Result<SysDictUpdateReqDto, AppError> {
    dto.dict_type = match dto.dict_type {
        Some(dict_type) => {
            let normalized = dict_type.trim().to_string();
            if normalized.is_empty() {
                return Err(AppError::bad_request("字典类型不能为空"));
            }
            Some(normalized)
        }
        None => None,
    };

    dto.label = match dto.label {
        Some(label) => {
            let normalized = label.trim().to_string();
            if normalized.is_empty() {
                return Err(AppError::bad_request("字典标签不能为空"));
            }
            Some(normalized)
        }
        None => None,
    };

    dto.value = match dto.value {
        Some(value) => {
            let normalized = value.trim().to_string();
            if normalized.is_empty() {
                return Err(AppError::bad_request("字典值不能为空"));
            }
            Some(normalized)
        }
        None => None,
    };

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
