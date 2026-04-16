use crate::core::dbal::query::fragments;
use crate::core::dto::sys_dict_dto::{SysDictListQueryDto, SysDictUpdateReqDto};
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::core::{errors::AppError, model::sys_dict::SysDictModel};

#[async_trait]
pub trait ISysDictRepository: Interface {
    async fn list(&self, query: SysDictListQueryDto) -> Result<Vec<SysDictModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysDictModel>, AppError>;
    async fn insert(&self, model: &SysDictModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysDictUpdateReqDto) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysDictRepository)]
pub(crate) struct SysDictRepository {
    pool: MySqlPool,
}

impl SysDictRepository {
    pub(crate) async fn list(
        &self,
        query: SysDictListQueryDto,
    ) -> Result<Vec<SysDictModel>, AppError> {
        let (type_kw, type_like) = fragments::keyword_args(query.dict_type.as_deref());
        let (label_kw, label_like) = fragments::keyword_args(query.dict_label.as_deref());
        let status = query.status.filter(|s| !s.trim().is_empty());

        sqlx::query_as::<_, SysDictModel>(
            r#"
            SELECT d.id, t.dict_type, d.label, d.value, d.status
            FROM sys_dict_data d
            INNER JOIN sys_dict_type t ON d.dict_type_id = t.id
            WHERE d.is_deleted = 0
              AND t.is_deleted = 0
              AND (? = '' OR t.dict_type LIKE ?)
              AND (? = '' OR d.label LIKE ?)
              AND (? IS NULL OR d.status = ?)
            ORDER BY d.id DESC
            "#,
        )
        .bind(&type_kw)
        .bind(&type_like)
        .bind(&label_kw)
        .bind(&label_like)
        .bind(&status)
        .bind(&status)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询字典失败: {err}")))
    }

    pub(crate) async fn get_by_id(&self, id: u64) -> Result<Option<SysDictModel>, AppError> {
        sqlx::query_as::<_, SysDictModel>(
            r#"
            SELECT d.id, t.dict_type, d.label, d.value, d.status
            FROM sys_dict_data d
            INNER JOIN sys_dict_type t ON d.dict_type_id = t.id
            WHERE d.id = ? AND d.is_deleted = 0 AND t.is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询字典失败: {err}")))
    }

    pub(crate) async fn insert(&self, model: &SysDictModel) -> Result<u64, AppError> {
        let dict_type_id = self.ensure_dict_type_id(&model.dict_type).await?;
        let result = sqlx::query(
            r#"
            INSERT INTO sys_dict_data (
                dict_type_id, label, value, status, sort,
                created_by, updated_by, is_deleted
            ) VALUES (?, ?, ?, ?, 0, 1, 1, 0)
            "#,
        )
        .bind(dict_type_id)
        .bind(&model.label)
        .bind(&model.value)
        .bind(model.status)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增字典失败: {err}")))?;
        Ok(result.last_insert_id())
    }

    pub(crate) async fn update_by_id(
        &self,
        id: u64,
        dto: SysDictUpdateReqDto,
    ) -> Result<bool, AppError> {
        let mut builder = QueryBuilder::<MySql>::new("UPDATE sys_dict_data SET ");
        let mut separated = builder.separated(", ");
        let mut has_update = false;

        if let Some(dict_type) = dto.dict_type {
            let dict_type_id = self.ensure_dict_type_id(&dict_type).await?;
            separated.push("dict_type_id = ").push_bind(dict_type_id);
            has_update = true;
        }
        if let Some(label) = dto.label {
            separated.push("label = ").push_bind(label);
            has_update = true;
        }
        if let Some(value) = dto.value {
            separated.push("value = ").push_bind(value);
            has_update = true;
        }
        if let Some(status) = dto.status {
            let status_value = if matches!(status.as_str(), "disabled" | "0") {
                0_i16
            } else {
                1_i16
            };
            separated.push("status = ").push_bind(status_value);
            has_update = true;
        }

        if !has_update {
            return Err(AppError::bad_request("没有可更新的字段"));
        }

        separated.push("updated_by = 1");
        builder
            .push(" WHERE id = ")
            .push_bind(id)
            .push(" AND is_deleted = 0");

        let result = builder
            .build()
            .execute(&self.pool)
            .await
            .map_err(|err| AppError::internal(format!("更新字典失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    pub(crate) async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_dict_data
            SET is_deleted = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("删除字典失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn ensure_dict_type_id(&self, dict_type: &str) -> Result<u64, AppError> {
        let found = sqlx::query_scalar::<_, u64>(
            r#"
            SELECT id
            FROM sys_dict_type
            WHERE dict_type = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(dict_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询字典类型失败: {err}")))?;

        if let Some(id) = found {
            return Ok(id);
        }

        let result = sqlx::query(
            r#"
            INSERT INTO sys_dict_type (dict_name, dict_type, status, created_by, updated_by, is_deleted)
            VALUES (?, ?, 1, 1, 1, 0)
            "#,
        )
        .bind(dict_type)
        .bind(dict_type)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("创建字典类型失败: {err}")))?;
        Ok(result.last_insert_id())
    }
}

#[async_trait]
impl ISysDictRepository for SysDictRepository {
    async fn list(&self, query: SysDictListQueryDto) -> Result<Vec<SysDictModel>, AppError> {
        self.list(query).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysDictModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysDictModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, dto: SysDictUpdateReqDto) -> Result<bool, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}
