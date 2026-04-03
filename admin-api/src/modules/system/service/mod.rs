pub mod integration;

use std::sync::Arc;

use serde_json::{Map, Value};

use crate::{
    core::{errors::AppError, vo::system::SystemCrudListVo},
    modules::system::repository::{CrudRecord, MySqlSystemRepository},
};

#[derive(Clone)]
pub struct SystemService {
    repo: Arc<MySqlSystemRepository>,
}

impl SystemService {
    pub fn new(repo: Arc<MySqlSystemRepository>) -> Self {
        Self { repo }
    }

    pub async fn list(
        &self,
        resource: &str,
        keyword: Option<&str>,
    ) -> Result<SystemCrudListVo, AppError> {
        validate_resource(resource)?;
        let items = self.repo.list(resource, keyword).await?;

        let data = items.into_iter().map(Value::Object).collect::<Vec<_>>();
        Ok(SystemCrudListVo {
            total: data.len(),
            items: data,
        })
    }

    pub async fn create(&self, resource: &str, payload: Value) -> Result<Value, AppError> {
        validate_resource(resource)?;
        let record = normalize_payload(payload)?;
        let created = self.repo.create(resource, record).await?;
        Ok(Value::Object(created))
    }

    pub async fn update(&self, resource: &str, id: u64, payload: Value) -> Result<Value, AppError> {
        validate_resource(resource)?;
        let record = normalize_payload(payload)?;
        let updated = self
            .repo
            .update(resource, id, record)
            .await?
            .ok_or_else(|| AppError::not_found(format!("{resource} 资源中不存在 id={id} 的记录")))?;
        Ok(Value::Object(updated))
    }

    pub async fn delete(&self, resource: &str, id: u64) -> Result<bool, AppError> {
        validate_resource(resource)?;
        let deleted = self.repo.delete(resource, id).await?;
        if !deleted {
            return Err(AppError::not_found(format!(
                "{resource} 资源中不存在 id={id} 的记录"
            )));
        }
        Ok(true)
    }
}

fn validate_resource(resource: &str) -> Result<(), AppError> {
    if MySqlSystemRepository::is_supported(resource) {
        return Ok(());
    }

    let allowed = MySqlSystemRepository::supported_resources().join(", ");
    Err(AppError::bad_request(format!(
        "不支持的资源类型: {resource}，可选值: {allowed}"
    )))
}

fn normalize_payload(payload: Value) -> Result<CrudRecord, AppError> {
    let mut record = payload
        .as_object()
        .cloned()
        .ok_or_else(|| AppError::bad_request("请求体必须是 JSON 对象"))?;

    drop_system_fields(&mut record);
    if record.is_empty() {
        return Err(AppError::bad_request("至少需要一个可写字段"));
    }

    Ok(record)
}

fn drop_system_fields(record: &mut Map<String, Value>) {
    record.remove("id");
}
