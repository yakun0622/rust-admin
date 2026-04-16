use crate::core::dbal::query::fragments;
use crate::core::dto::sys_config_dto::{SysConfigListQueryDto, SysConfigUpdateReqDto};
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::core::{errors::AppError, model::sys_config::SysConfigModel};

#[async_trait]
pub trait ISysConfigRepository: Interface {
    async fn list(&self, query: SysConfigListQueryDto) -> Result<Vec<SysConfigModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysConfigModel>, AppError>;
    async fn insert(&self, model: &SysConfigModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysConfigUpdateReqDto) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysConfigRepository)]
pub(crate) struct SysConfigRepository {
    pool: MySqlPool,
}

impl SysConfigRepository {
    pub(crate) async fn list(
        &self,
        query: SysConfigListQueryDto,
    ) -> Result<Vec<SysConfigModel>, AppError> {
        let (name_kw, name_like) = fragments::keyword_args(query.name.as_deref());
        let (key_kw, key_like) = fragments::keyword_args(query.key.as_deref());
        let status = query.status.filter(|s| !s.trim().is_empty());

        sqlx::query_as::<_, SysConfigModel>(
            r#"
            SELECT id, config_name, config_key, config_value, remark, status
            FROM sys_config
            WHERE is_deleted = 0
              AND (? = '' OR config_name LIKE ?)
              AND (? = '' OR config_key LIKE ?)
              AND (? IS NULL OR status = ?)
            ORDER BY id DESC
            "#,
        )
        .bind(&name_kw)
        .bind(&name_like)
        .bind(&key_kw)
        .bind(&key_like)
        .bind(&status)
        .bind(&status)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询配置失败: {err}")))
    }

    pub(crate) async fn get_by_id(&self, id: u64) -> Result<Option<SysConfigModel>, AppError> {
        sqlx::query_as::<_, SysConfigModel>(
            r#"
            SELECT id, config_name, config_key, config_value, remark, status
            FROM sys_config
            WHERE id = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询配置失败: {err}")))
    }

    pub(crate) async fn insert(&self, model: &SysConfigModel) -> Result<u64, AppError> {
        let result = sqlx::query(
            r#"
            INSERT INTO sys_config (
                config_name, config_key, config_value, status, remark,
                created_by, updated_by, is_deleted
            ) VALUES (?, ?, ?, ?, ?, 1, 1, 0)
            "#,
        )
        .bind(&model.config_name)
        .bind(&model.config_key)
        .bind(&model.config_value)
        .bind(model.status)
        .bind(model.remark.as_deref())
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增配置失败: {err}")))?;
        Ok(result.last_insert_id())
    }

    pub(crate) async fn update_by_id(
        &self,
        id: u64,
        dto: SysConfigUpdateReqDto,
    ) -> Result<bool, AppError> {
        let mut builder = QueryBuilder::<MySql>::new("UPDATE sys_config SET ");
        let mut separated = builder.separated(", ");
        let mut has_update = false;

        if let Some(name) = dto.name {
            separated.push("config_name = ").push_bind(name.clone());
            separated.push("config_key = ").push_bind(name);
            has_update = true;
        }
        if let Some(value) = dto.value {
            separated.push("config_value = ").push_bind(value);
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
        if let Some(remark) = dto.remark {
            separated.push("remark = ").push_bind(remark);
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
            .map_err(|err| AppError::internal(format!("更新配置失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    pub(crate) async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_config
            SET is_deleted = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("删除配置失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }
}

#[async_trait]
impl ISysConfigRepository for SysConfigRepository {
    async fn list(&self, query: SysConfigListQueryDto) -> Result<Vec<SysConfigModel>, AppError> {
        self.list(query).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysConfigModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysConfigModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, dto: SysConfigUpdateReqDto) -> Result<bool, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}
