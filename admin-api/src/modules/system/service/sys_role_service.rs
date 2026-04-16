use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::core::{
    converter::sys_role_converter::{from_create_dto, to_sys_role_vo},
    dto::sys_role_dto::{SysRoleCreateReqDto, SysRoleListQueryDto, SysRoleUpdateReqDto},
    errors::AppError,
    vo::sys_role_vo::SysRoleVo,
};
use crate::modules::system::repository::ISysRoleRepository;

#[async_trait]
pub trait ISysRoleService: Interface {
    async fn list(&self, query: SysRoleListQueryDto) -> Result<Vec<SysRoleVo>, AppError>;
    async fn create(&self, dto: SysRoleCreateReqDto) -> Result<SysRoleVo, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysRoleUpdateReqDto) -> Result<SysRoleVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
    async fn get_role_menu_ids(&self, id: u64) -> Result<Vec<u64>, AppError>;
    async fn update_role_menus(&self, id: u64, menu_ids: Vec<u64>) -> Result<(), AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysRoleService)]
pub struct SysRoleService {
    #[shaku(inject)]
    repo: Arc<dyn ISysRoleRepository>,
}

impl SysRoleService {
    pub async fn list(&self, query: SysRoleListQueryDto) -> Result<Vec<SysRoleVo>, AppError> {
        let roles = self.repo.list(query).await?;
        Ok(roles.into_iter().map(to_sys_role_vo).collect())
    }

    pub async fn create(&self, dto: SysRoleCreateReqDto) -> Result<SysRoleVo, AppError> {
        let model = from_create_dto(dto)?;
        let id = self.repo.insert(&model).await?;
        let created = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("创建成功但读取角色失败: id={id}")))?;
        Ok(to_sys_role_vo(created))
    }

    pub async fn update_by_id(
        &self,
        id: u64,
        dto: SysRoleUpdateReqDto,
    ) -> Result<SysRoleVo, AppError> {
        let normalized = normalize_update_dto(dto)?;
        let affected = self.repo.update_by_id(id, normalized).await?;
        if !affected {
            return Err(AppError::not_found(format!(
                "role 资源中不存在 id={id} 的记录"
            )));
        }
        let updated = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("更新成功但读取角色失败: id={id}")))?;
        Ok(to_sys_role_vo(updated))
    }

    pub async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let deleted = self.repo.delete_by_id(id).await?;
        if !deleted {
            return Err(AppError::not_found(format!(
                "role 资源中不存在 id={id} 的记录"
            )));
        }
        Ok(true)
    }

    pub async fn get_role_menu_ids(&self, id: u64) -> Result<Vec<u64>, AppError> {
        self.repo.get_menu_ids_by_role_id(id).await
    }

    pub async fn update_role_menus(&self, id: u64, menu_ids: Vec<u64>) -> Result<(), AppError> {
        self.repo.update_role_menus(id, menu_ids).await
    }
}

#[async_trait]
impl ISysRoleService for SysRoleService {
    async fn list(&self, query: SysRoleListQueryDto) -> Result<Vec<SysRoleVo>, AppError> {
        self.list(query).await
    }

    async fn create(&self, dto: SysRoleCreateReqDto) -> Result<SysRoleVo, AppError> {
        self.create(dto).await
    }

    async fn update_by_id(&self, id: u64, dto: SysRoleUpdateReqDto) -> Result<SysRoleVo, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }

    async fn get_role_menu_ids(&self, id: u64) -> Result<Vec<u64>, AppError> {
        self.get_role_menu_ids(id).await
    }

    async fn update_role_menus(&self, id: u64, menu_ids: Vec<u64>) -> Result<(), AppError> {
        self.update_role_menus(id, menu_ids).await
    }
}

fn normalize_update_dto(mut dto: SysRoleUpdateReqDto) -> Result<SysRoleUpdateReqDto, AppError> {
    dto.name = match dto.name {
        Some(name) => {
            let normalized = name.trim().to_string();
            if normalized.is_empty() {
                return Err(AppError::bad_request("角色名称不能为空"));
            }
            Some(normalized)
        }
        None => None,
    };

    dto.key = match dto.key {
        Some(key) => {
            let normalized = key.trim().to_string();
            if normalized.is_empty() {
                return Err(AppError::bad_request("权限标识不能为空"));
            }
            Some(normalized)
        }
        None => None,
    };

    if let Some(sort) = dto.sort {
        if sort < 0 {
            return Err(AppError::bad_request("角色排序值不能小于0"));
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
