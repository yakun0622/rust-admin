use std::sync::Arc;

use async_trait::async_trait;
use shaku::Component;

use crate::core::{
    converter::sys_dept_converter::{from_create_dto, from_update_dto, to_sys_dept_vo},
    dto::sys_dept_dto::{SysDeptCreateReqDto, SysDeptUpdateReqDto},
    errors::AppError,
    vo::sys_dept_vo::{SysDeptListVo, SysDeptVo},
};
use crate::modules::system::repository::interface::ISysDeptRepository;

use super::interface::ISysDeptService;

#[derive(Component, Clone)]
#[shaku(interface = ISysDeptService)]
pub struct SysDeptService {
    #[shaku(inject)]
    repo: Arc<dyn ISysDeptRepository>,
}

impl SysDeptService {
    pub async fn list(&self, keyword: Option<&str>) -> Result<SysDeptListVo, AppError> {
        let items = self
            .repo
            .list(keyword.and_then(normalize_optional_str))
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
        let current = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found(format!("dept 资源中不存在 id={id} 的记录")))?;
        let parent_id = dto.parent_id.unwrap_or(current.parent_id);
        if parent_id == id {
            return Err(AppError::bad_request("上级部门不能为自身"));
        }

        let model = from_update_dto(id, parent_id, dto)?;
        let affected = self.repo.update_by_id(id, &model).await?;
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
    async fn list(&self, keyword: Option<&str>) -> Result<SysDeptListVo, AppError> {
        self.list(keyword).await
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

fn normalize_optional_str(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
