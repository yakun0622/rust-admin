use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::core::{
    converter::sys_dept_converter::{from_create_dto, to_sys_dept_vo},
    dto::sys_dept_dto::{SysDeptCreateReqDto, SysDeptListQueryDto, SysDeptUpdateReqDto},
    errors::AppError,
    vo::sys_dept_vo::{SysDeptListVo, SysDeptVo},
};
use crate::modules::system::repository::ISysDeptRepository;

#[async_trait]
pub trait ISysDeptService: Interface {
    async fn list(&self, query: SysDeptListQueryDto) -> Result<SysDeptListVo, AppError>;
    async fn create(&self, dto: SysDeptCreateReqDto) -> Result<SysDeptVo, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysDeptUpdateReqDto) -> Result<SysDeptVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysDeptService)]
pub struct SysDeptService {
    #[shaku(inject)]
    repo: Arc<dyn ISysDeptRepository>,
}

impl SysDeptService {
    pub async fn list(&self, query: SysDeptListQueryDto) -> Result<SysDeptListVo, AppError> {
        let items = self
            .repo
            .list(query)
            .await?
            .into_iter()
            .map(to_sys_dept_vo)
            .collect::<Vec<_>>();

        Ok(SysDeptListVo {
            total: items.len(),
            items,
        })
    }

    pub async fn create(&self, dto: SysDeptCreateReqDto) -> Result<SysDeptVo, AppError> {
        let model = from_create_dto(dto)?;
        let id = self.repo.insert(&model).await?;
        let created = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("创建成功但读取部门失败: id={id}")))?;
        Ok(to_sys_dept_vo(created))
    }

    pub async fn update_by_id(
        &self,
        id: u64,
        dto: SysDeptUpdateReqDto,
    ) -> Result<SysDeptVo, AppError> {
        if dto.parent_id == Some(id) {
            return Err(AppError::bad_request("上级部门不能为自身"));
        }

        let normalized = normalize_update_dto(dto)?;
        let affected = self.repo.update_by_id(id, normalized).await?;
        if !affected {
            return Err(AppError::not_found(format!(
                "dept 资源中不存在 id={id} 的记录"
            )));
        }

        let updated = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("更新成功但读取部门失败: id={id}")))?;
        Ok(to_sys_dept_vo(updated))
    }

    pub async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let deleted = self.repo.delete_by_id(id).await?;
        if !deleted {
            return Err(AppError::not_found(format!(
                "dept 资源中不存在 id={id} 的记录"
            )));
        }
        Ok(true)
    }
}

#[async_trait]
impl ISysDeptService for SysDeptService {
    async fn list(&self, query: SysDeptListQueryDto) -> Result<SysDeptListVo, AppError> {
        self.list(query).await
    }

    async fn create(&self, dto: SysDeptCreateReqDto) -> Result<SysDeptVo, AppError> {
        self.create(dto).await
    }

    async fn update_by_id(&self, id: u64, dto: SysDeptUpdateReqDto) -> Result<SysDeptVo, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}

fn normalize_update_dto(mut dto: SysDeptUpdateReqDto) -> Result<SysDeptUpdateReqDto, AppError> {
    dto.name = match dto.name {
        Some(name) => {
            let normalized = name.trim().to_string();
            if normalized.is_empty() {
                return Err(AppError::bad_request("部门名称不能为空"));
            }
            Some(normalized)
        }
        None => None,
    };

    dto.leader = dto.leader.and_then(|leader| {
        let normalized = leader.trim().to_string();
        if normalized.is_empty() {
            None
        } else {
            Some(normalized)
        }
    });

    dto.phone = dto.phone.and_then(|phone| {
        let normalized = phone.trim().to_string();
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
