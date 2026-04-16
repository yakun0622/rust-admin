use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::core::{
    converter::sys_user_converter::{from_create_dto, to_sys_user_vo},
    dto::sys_user_dto::{SysUserCreateReqDto, SysUserListQueryDto, SysUserUpdateReqDto},
    errors::AppError,
    vo::sys_user_vo::SysUserVo,
};
use crate::modules::system::repository::{ISysUserRepository, DEFAULT_PASSWORD_HASH};

#[async_trait]
pub trait ISysUserService: Interface {
    async fn list(&self, query: SysUserListQueryDto) -> Result<Vec<SysUserVo>, AppError>;
    async fn create(&self, dto: SysUserCreateReqDto) -> Result<SysUserVo, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysUserUpdateReqDto) -> Result<SysUserVo, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysUserService)]
pub struct SysUserService {
    #[shaku(inject)]
    repo: Arc<dyn ISysUserRepository>,
}

impl SysUserService {
    pub async fn list(&self, query: SysUserListQueryDto) -> Result<Vec<SysUserVo>, AppError> {
        let users = self.repo.list(query).await?;
        Ok(users.into_iter().map(to_sys_user_vo).collect())
    }

    pub async fn create(&self, dto: SysUserCreateReqDto) -> Result<SysUserVo, AppError> {
        let normalized = normalize_create_dto(dto)?;

        let model = from_create_dto(normalized, DEFAULT_PASSWORD_HASH);
        let created_id = self.repo.insert(&model).await?;
        let created = self.repo.get_by_id(created_id).await?.ok_or_else(|| {
            AppError::internal(format!("创建成功但读取用户失败: id={created_id}"))
        })?;
        Ok(to_sys_user_vo(created))
    }

    pub async fn update_by_id(
        &self,
        id: u64,
        dto: SysUserUpdateReqDto,
    ) -> Result<SysUserVo, AppError> {
        let normalized = normalize_update_dto(dto)?;
        let affected = self.repo.update_by_id(id, normalized).await?;
        if !affected {
            return Err(AppError::not_found(format!(
                "user 资源中不存在 id={id} 的记录"
            )));
        }

        let updated = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("更新成功但读取用户失败: id={id}")))?;
        Ok(to_sys_user_vo(updated))
    }

    pub async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let deleted = self.repo.delete_by_id(id).await?;
        if !deleted {
            return Err(AppError::not_found(format!(
                "user 资源中不存在 id={id} 的记录"
            )));
        }
        Ok(true)
    }
}

#[async_trait]
impl ISysUserService for SysUserService {
    async fn list(&self, query: SysUserListQueryDto) -> Result<Vec<SysUserVo>, AppError> {
        self.list(query).await
    }

    async fn create(&self, dto: SysUserCreateReqDto) -> Result<SysUserVo, AppError> {
        self.create(dto).await
    }

    async fn update_by_id(&self, id: u64, dto: SysUserUpdateReqDto) -> Result<SysUserVo, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}

fn normalize_create_dto(mut dto: SysUserCreateReqDto) -> Result<SysUserCreateReqDto, AppError> {
    dto.username = normalize_required_text("用户名", dto.username)?;
    dto.nickname = normalize_required_text("昵称", dto.nickname)?;
    dto.phone = normalize_optional_text(dto.phone.as_deref()).map(ToString::to_string);
    dto.status = Some(normalize_status(dto.status.as_deref())?.to_string());
    Ok(dto)
}

fn normalize_update_dto(mut dto: SysUserUpdateReqDto) -> Result<SysUserUpdateReqDto, AppError> {
    dto.username = match dto.username {
        Some(username) => Some(normalize_required_text("用户名", username)?),
        None => None,
    };
    dto.nickname = match dto.nickname {
        Some(nickname) => Some(normalize_required_text("昵称", nickname)?),
        None => None,
    };
    dto.phone = dto
        .phone
        .and_then(|phone| normalize_optional_str(phone.as_str()).map(ToString::to_string));
    dto.status = match dto.status {
        Some(status) => Some(normalize_status(Some(status.as_str()))?.to_string()),
        None => None,
    };
    Ok(dto)
}

fn normalize_required_text(field_name: &str, value: String) -> Result<String, AppError> {
    let normalized = value.trim().to_string();
    if normalized.is_empty() {
        return Err(AppError::bad_request(format!("{field_name}不能为空")));
    }
    Ok(normalized)
}

fn normalize_optional_text(value: Option<&str>) -> Option<&str> {
    value.and_then(normalize_optional_str)
}

fn normalize_optional_str(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn normalize_status(raw: Option<&str>) -> Result<&'static str, AppError> {
    let normalized = normalize_optional_text(raw)
        .map(|item| item.to_ascii_lowercase())
        .unwrap_or_else(|| "enabled".to_string());

    match normalized.as_str() {
        "enabled" | "1" => Ok("enabled"),
        "disabled" | "0" => Ok("disabled"),
        _ => Err(AppError::bad_request(
            "状态值非法，仅支持 enabled/disabled/1/0",
        )),
    }
}
