use crate::core::dbal::query::fragments;
use async_trait::async_trait;
use shaku::Component;
use sqlx::MySqlPool;

use crate::core::{errors::AppError, model::sys_dict::SysDictModel};

use super::interface::ISysDictRepository;

#[derive(Component, Clone)]
#[shaku(interface = ISysDictRepository)]
pub(crate) struct SysDictRepository {
    pool: MySqlPool,
}

impl SysDictRepository {
    pub(crate) async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysDictModel>, AppError> {
        let (kw, like) = fragments::keyword_args(keyword);
        sqlx::query_as::<_, SysDictModel>(
            r#"
            SELECT d.id, t.dict_type, d.label, d.value, d.status
            FROM sys_dict_data d
            INNER JOIN sys_dict_type t ON d.dict_type_id = t.id
            WHERE d.is_deleted = 0
              AND t.is_deleted = 0
              AND (? = '' OR t.dict_type LIKE ? OR d.label LIKE ? OR d.value LIKE ?)
            ORDER BY d.id DESC
            "#,
        )
        .bind(&kw)
        .bind(&like)
        .bind(&like)
        .bind(&like)
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
        model: &SysDictModel,
    ) -> Result<bool, AppError> {
        let dict_type_id = self.ensure_dict_type_id(&model.dict_type).await?;
        let result = sqlx::query(
            r#"
            UPDATE sys_dict_data
            SET dict_type_id = ?, label = ?, value = ?, status = ?, updated_by = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(dict_type_id)
        .bind(&model.label)
        .bind(&model.value)
        .bind(model.status)
        .bind(id)
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
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysDictModel>, AppError> {
        self.list(keyword).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysDictModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysDictModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, model: &SysDictModel) -> Result<bool, AppError> {
        self.update_by_id(id, model).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}
