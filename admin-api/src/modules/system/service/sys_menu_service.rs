use crate::core::{
    converter::sys_menu_converter::{from_create_dto, from_update_dto, to_sys_menu_vo},
    dto::sys_menu_dto::{SysMenuCreateReqDto, SysMenuUpdateReqDto},
    errors::AppError,
    vo::sys_menu_vo::{SysMenuListVo, SysMenuVo},
};
use crate::modules::system::repository::SysMenuRepository;

#[derive(Clone)]
pub struct SysMenuService {
    repo: SysMenuRepository,
}

impl SysMenuService {
    pub(crate) fn new(repo: SysMenuRepository) -> Self {
        Self { repo }
    }

    pub async fn list(&self, keyword: Option<&str>) -> Result<SysMenuListVo, AppError> {
        let items = self
            .repo
            .list(keyword.and_then(normalize_optional_str))
            .await?
            .into_iter()
            .map(to_sys_menu_vo)
            .collect::<Vec<_>>();

        Ok(SysMenuListVo {
            total: items.len(),
            items,
        })
    }

    pub async fn create(&self, dto: SysMenuCreateReqDto) -> Result<SysMenuVo, AppError> {
        let model = from_create_dto(dto)?;
        let id = self.repo.insert(&model).await?;
        let created = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("创建成功但读取菜单失败: id={id}")))?;
        Ok(to_sys_menu_vo(created))
    }

    pub async fn update_by_id(
        &self,
        id: u64,
        dto: SysMenuUpdateReqDto,
    ) -> Result<SysMenuVo, AppError> {
        let current = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found(format!("menu 资源中不存在 id={id} 的记录")))?;

        let parent_id = dto.parent_id.unwrap_or(current.parent_id);
        if parent_id == id {
            return Err(AppError::bad_request("上级菜单不能为自身"));
        }

        let model = from_update_dto(id, parent_id, dto)?;
        let affected = self.repo.update_by_id(id, &model).await?;
        if !affected {
            return Err(AppError::not_found(format!(
                "menu 资源中不存在 id={id} 的记录"
            )));
        }

        let updated = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("更新成功但读取菜单失败: id={id}")))?;
        Ok(to_sys_menu_vo(updated))
    }

    pub async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let deleted = self.repo.delete_by_id(id).await?;
        if !deleted {
            return Err(AppError::not_found(format!(
                "menu 资源中不存在 id={id} 的记录"
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
