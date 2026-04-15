use crate::core::{
    converter::sys_post_converter::{from_create_dto, from_update_dto, to_sys_post_vo},
    dto::sys_post_dto::{SysPostCreateReqDto, SysPostUpdateReqDto},
    errors::AppError,
    vo::sys_post_vo::{SysPostListVo, SysPostVo},
};
use crate::modules::system::repository::SysPostRepository;

#[derive(Clone)]
pub struct SysPostService {
    repo: SysPostRepository,
}

impl SysPostService {
    pub(crate) fn new(repo: SysPostRepository) -> Self {
        Self { repo }
    }

    pub async fn list(&self, keyword: Option<&str>) -> Result<SysPostListVo, AppError> {
        let items = self
            .repo
            .list(keyword.and_then(normalize_optional_str))
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
        let model = from_update_dto(id, dto)?;
        let affected = self.repo.update_by_id(id, &model).await?;
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

fn normalize_optional_str(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
