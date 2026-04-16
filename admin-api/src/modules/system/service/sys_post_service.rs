use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::core::{
    converter::sys_post_converter::{from_create_dto, to_sys_post_vo},
    dto::sys_post_dto::{SysPostCreateReqDto, SysPostListQueryDto, SysPostUpdateReqDto},
    errors::AppError,
    vo::sys_post_vo::{SysPostListVo, SysPostVo},
};
use crate::modules::system::repository::ISysPostRepository;

#[async_trait]
pub trait ISysPostService: Interface {
    async fn list(&self, query: SysPostListQueryDto) -> Result<SysPostListVo, AppError>;
    async fn create(&self, dto: SysPostCreateReqDto) -> Result<SysPostVo, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysPostUpdateReqDto) -> Result<SysPostVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysPostService)]
pub struct SysPostService {
    #[shaku(inject)]
    repo: Arc<dyn ISysPostRepository>,
}

impl SysPostService {
    pub async fn list(&self, query: SysPostListQueryDto) -> Result<SysPostListVo, AppError> {
        let items = self
            .repo
            .list(query)
            .await?
            .into_iter()
            .map(to_sys_post_vo)
            .collect::<Vec<_>>();

        Ok(SysPostListVo {
            total: items.len(),
            items,
        })
    }

    pub async fn create(&self, dto: SysPostCreateReqDto) -> Result<SysPostVo, AppError> {
        let model = from_create_dto(dto)?;
        let id = self.repo.insert(&model).await?;
        let created = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("创建成功但读取岗位失败: id={id}")))?;
        Ok(to_sys_post_vo(created))
    }

    pub async fn update_by_id(
        &self,
        id: u64,
        dto: SysPostUpdateReqDto,
    ) -> Result<SysPostVo, AppError> {
        let normalized = normalize_update_dto(dto)?;
        let affected = self.repo.update_by_id(id, normalized).await?;
        if !affected {
            return Err(AppError::not_found(format!(
                "post 资源中不存在 id={id} 的记录"
            )));
        }
        let updated = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("更新成功但读取岗位失败: id={id}")))?;
        Ok(to_sys_post_vo(updated))
    }

    pub async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let deleted = self.repo.delete_by_id(id).await?;
        if !deleted {
            return Err(AppError::not_found(format!(
                "post 资源中不存在 id={id} 的记录"
            )));
        }
        Ok(true)
    }
}

#[async_trait]
impl ISysPostService for SysPostService {
    async fn list(&self, query: SysPostListQueryDto) -> Result<SysPostListVo, AppError> {
        self.list(query).await
    }

    async fn create(&self, dto: SysPostCreateReqDto) -> Result<SysPostVo, AppError> {
        self.create(dto).await
    }

    async fn update_by_id(&self, id: u64, dto: SysPostUpdateReqDto) -> Result<SysPostVo, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}

fn normalize_update_dto(mut dto: SysPostUpdateReqDto) -> Result<SysPostUpdateReqDto, AppError> {
    dto.name = match dto.name {
        Some(name) => {
            let normalized = name.trim().to_string();
            if normalized.is_empty() {
                return Err(AppError::bad_request("岗位名称不能为空"));
            }
            Some(normalized)
        }
        None => None,
    };

    dto.code = match dto.code {
        Some(code) => {
            let normalized = code.trim().to_string();
            if normalized.is_empty() {
                return Err(AppError::bad_request("岗位编码不能为空"));
            }
            Some(normalized)
        }
        None => None,
    };

    if let Some(sort) = dto.sort {
        if sort < 0 {
            return Err(AppError::bad_request("岗位排序值不能小于0"));
        }
    }

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
