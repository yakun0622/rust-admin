use std::sync::Arc;

use async_trait::async_trait;
use serde_json::{json, Value};
use shaku::{Component, Interface};

use crate::core::{
    dto::sys_notice_dto::{SysNoticeCreateReqDto, SysNoticeListQueryDto, SysNoticeUpdateReqDto},
    errors::AppError,
    model::system::SysNoticePo,
    vo::system_vo::SystemCrudListVo,
};
use crate::modules::system::repository::ISysNoticeRepository;

#[async_trait]
pub trait ISysNoticeService: Interface {
    async fn list(&self, query: SysNoticeListQueryDto) -> Result<SystemCrudListVo, AppError>;
    async fn create(&self, dto: SysNoticeCreateReqDto) -> Result<Value, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysNoticeUpdateReqDto) -> Result<Value, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysNoticeService)]
pub struct SysNoticeService {
    #[shaku(inject)]
    repo: Arc<dyn ISysNoticeRepository>,
}

impl SysNoticeService {
    pub async fn list(&self, query: SysNoticeListQueryDto) -> Result<SystemCrudListVo, AppError> {
        let items = self
            .repo
            .list(query)
            .await?
            .into_iter()
            .map(notice_to_value)
            .collect::<Vec<_>>();

        Ok(SystemCrudListVo {
            total: items.len(),
            items,
        })
    }

    pub async fn create(&self, dto: SysNoticeCreateReqDto) -> Result<Value, AppError> {
        let normalized = normalize_create_dto(dto)?;
        let title = required_text("公告标题", normalized.title)?;
        let notice_type = normalize_notice_type(normalized.notice_type.as_deref())?;
        let status = normalize_notice_status(normalized.status.as_deref())?;
        let id = self
            .repo
            .insert(&title, notice_type, status, normalized.publisher.as_deref())
            .await?;
        let created = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("创建成功但读取公告失败: id={id}")))?;
        Ok(notice_to_value(created))
    }

    pub async fn update_by_id(
        &self,
        id: u64,
        dto: SysNoticeUpdateReqDto,
    ) -> Result<Value, AppError> {
        let normalized = normalize_update_dto(dto)?;
        let affected = self.repo.update_by_id(id, normalized).await?;
        if !affected {
            return Err(AppError::not_found(format!(
                "notice 资源中不存在 id={id} 的记录"
            )));
        }
        let updated = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::internal(format!("更新成功但读取公告失败: id={id}")))?;
        Ok(notice_to_value(updated))
    }

    pub async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let deleted = self.repo.delete_by_id(id).await?;
        if !deleted {
            return Err(AppError::not_found(format!(
                "notice 资源中不存在 id={id} 的记录"
            )));
        }
        Ok(true)
    }
}

#[async_trait]
impl ISysNoticeService for SysNoticeService {
    async fn list(&self, query: SysNoticeListQueryDto) -> Result<SystemCrudListVo, AppError> {
        self.list(query).await
    }

    async fn create(&self, dto: SysNoticeCreateReqDto) -> Result<Value, AppError> {
        self.create(dto).await
    }

    async fn update_by_id(&self, id: u64, dto: SysNoticeUpdateReqDto) -> Result<Value, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}

fn notice_to_value(item: SysNoticePo) -> Value {
    json!({
        "id": item.id,
        "title": item.title,
        "type": notice_type_label(item.notice_type),
        "status": notice_status_label(item.status),
        "publisher": item.publisher,
    })
}

fn required_text(field_name: &str, raw: String) -> Result<String, AppError> {
    let normalized = raw.trim().to_string();
    if normalized.is_empty() {
        return Err(AppError::bad_request(format!("{field_name}不能为空")));
    }
    Ok(normalized)
}

fn normalize_create_dto(mut dto: SysNoticeCreateReqDto) -> Result<SysNoticeCreateReqDto, AppError> {
    dto.title = required_text("公告标题", dto.title)?;
    dto.notice_type = Some(normalize_notice_type(dto.notice_type.as_deref())?.to_string());
    dto.status = Some(normalize_notice_status(dto.status.as_deref())?.to_string());
    dto.publisher = dto
        .publisher
        .and_then(|publisher| normalize_optional_text(publisher.as_str()).map(ToString::to_string));
    Ok(dto)
}

fn normalize_update_dto(mut dto: SysNoticeUpdateReqDto) -> Result<SysNoticeUpdateReqDto, AppError> {
    dto.title = match dto.title {
        Some(title) => Some(required_text("公告标题", title)?),
        None => None,
    };
    dto.notice_type = match dto.notice_type {
        Some(notice_type) => Some(normalize_notice_type(Some(notice_type.trim()))?.to_string()),
        None => None,
    };
    dto.status = match dto.status {
        Some(status) => Some(normalize_notice_status(Some(status.trim()))?.to_string()),
        None => None,
    };
    dto.publisher = match dto.publisher {
        Some(publisher) => Some(publisher.trim().to_string()),
        None => None,
    };
    Ok(dto)
}

fn normalize_optional_text(raw: &str) -> Option<&str> {
    let normalized = raw.trim();
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

fn normalize_notice_type(raw: Option<&str>) -> Result<i16, AppError> {
    let value = raw
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .unwrap_or("通知");
    match value {
        "通知" | "1" => Ok(1),
        "公告" | "2" => Ok(2),
        _ => Err(AppError::bad_request("公告类型非法，仅支持 通知/公告/1/2")),
    }
}

fn normalize_notice_status(raw: Option<&str>) -> Result<i16, AppError> {
    let value = raw
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .unwrap_or("draft");
    match value {
        "draft" | "0" => Ok(0),
        "published" | "1" => Ok(1),
        "offline" | "2" => Ok(2),
        _ => Err(AppError::bad_request(
            "公告状态非法，仅支持 draft/published/offline/0/1/2",
        )),
    }
}

fn notice_type_label(value: i16) -> &'static str {
    if value == 2 {
        "公告"
    } else {
        "通知"
    }
}

fn notice_status_label(value: i16) -> &'static str {
    match value {
        1 => "published",
        2 => "offline",
        _ => "draft",
    }
}
