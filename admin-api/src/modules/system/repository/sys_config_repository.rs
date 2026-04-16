use crate::core::dbal::query::fragments;
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::MySqlPool;

use crate::core::{errors::AppError, model::sys_config::SysConfigModel};

#[async_trait]
pub trait ISysConfigRepository: Interface {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysConfigModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysConfigModel>, AppError>;
    async fn insert(&self, model: &SysConfigModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, model: &SysConfigModel) -> Result<bool, AppError>;
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
        keyword: Option<&str>,
    ) -> Result<Vec<SysConfigModel>, AppError> {
        let (kw, like) = fragments::keyword_args(keyword);
        sqlx::query_as::<_, SysConfigModel>(
            r#"
            SELECT id, config_name, config_key, config_value, remark, status
            FROM sys_config
            WHERE is_deleted = 0
              AND (? = '' OR config_key LIKE ? OR config_value LIKE ? OR remark LIKE ?)
            ORDER BY id DESC
            "#,
        )
        .bind(&kw)
        .bind(&like)
        .bind(&like)
        .bind(&like)
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
        model: &SysConfigModel,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_config
            SET config_name = ?, config_key = ?, config_value = ?, status = ?, remark = ?, updated_by = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(&model.config_name)
        .bind(&model.config_key)
        .bind(&model.config_value)
        .bind(model.status)
        .bind(model.remark.as_deref())
        .bind(id)
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
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysConfigModel>, AppError> {
        self.list(keyword).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysConfigModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysConfigModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, model: &SysConfigModel) -> Result<bool, AppError> {
        self.update_by_id(id, model).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}
