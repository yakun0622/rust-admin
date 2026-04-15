use serde_json::{json, Value};

use crate::core::{
    dto::system_dto::{SysNoticeCreateReqDto, SysNoticeUpdateReqDto},
    errors::AppError,
    model::system::SysNoticePo,
    vo::system_vo::SystemCrudListVo,
};
use crate::modules::system::repository::SysNoticeRepository;

#[derive(Clone)]
pub struct SysNoticeService {
    repo: SysNoticeRepository,
}

impl SysNoticeService {
    pub(crate) fn new(repo: SysNoticeRepository) -> Self {
        Self { repo }
    }

    pub async fn list(&self, keyword: Option<&str>) -> Result<SystemCrudListVo, AppError> {
        let items = self
            .repo
            .list(keyword)
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
        let title = required_text("公告标题", dto.title)?;
        let notice_type = normalize_notice_type(dto.notice_type.as_deref())?;
        let status = normalize_notice_status(dto.status.as_deref())?;
        let id = self
            .repo
            .insert(&title, notice_type, status, dto.publisher.as_deref())
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
        let title = required_text("公告标题", dto.title)?;
        let notice_type = normalize_notice_type(dto.notice_type.as_deref())?;
        let status = normalize_notice_status(dto.status.as_deref())?;
        let affected = self
            .repo
            .update_by_id(id, &title, notice_type, status, dto.publisher.as_deref())
            .await?;
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
