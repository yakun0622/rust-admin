use crate::core::dbal::query::fragments;
use async_trait::async_trait;
use shaku::Component;
use sqlx::MySqlPool;

use crate::core::{errors::AppError, model::sys_post::SysPostModel};

use super::interface::ISysPostRepository;

#[derive(Component, Clone)]
#[shaku(interface = ISysPostRepository)]
pub(crate) struct SysPostRepository {
    pool: MySqlPool,
}

impl SysPostRepository {
    pub(crate) async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysPostModel>, AppError> {
        let (kw, like) = fragments::keyword_args(keyword);
        sqlx::query_as::<_, SysPostModel>(
            r#"
            SELECT id, post_name, post_code, post_sort, status
            FROM sys_post
            WHERE is_deleted = 0
              AND (? = '' OR post_name LIKE ? OR post_code LIKE ?)
            ORDER BY id DESC
            "#,
        )
        .bind(&kw)
        .bind(&like)
        .bind(&like)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询岗位失败: {err}")))
    }

    pub(crate) async fn get_by_id(&self, id: u64) -> Result<Option<SysPostModel>, AppError> {
        sqlx::query_as::<_, SysPostModel>(
            r#"
            SELECT id, post_name, post_code, post_sort, status
            FROM sys_post
            WHERE id = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询岗位失败: {err}")))
    }

    pub(crate) async fn insert(&self, model: &SysPostModel) -> Result<u64, AppError> {
        let result = sqlx::query(
            r#"
            INSERT INTO sys_post (
                post_name, post_code, post_sort, status,
                created_by, updated_by, is_deleted
            ) VALUES (?, ?, ?, ?, 1, 1, 0)
            "#,
        )
        .bind(&model.post_name)
        .bind(&model.post_code)
        .bind(model.post_sort)
        .bind(model.status)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增岗位失败: {err}")))?;
        Ok(result.last_insert_id())
    }

    pub(crate) async fn update_by_id(
        &self,
        id: u64,
        model: &SysPostModel,
    ) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_post
            SET post_name = ?, post_code = ?, post_sort = ?, status = ?, updated_by = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(&model.post_name)
        .bind(&model.post_code)
        .bind(model.post_sort)
        .bind(model.status)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新岗位失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    pub(crate) async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_post
            SET is_deleted = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("删除岗位失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }
}

#[async_trait]
impl ISysPostRepository for SysPostRepository {
    async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysPostModel>, AppError> {
        self.list(keyword).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysPostModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysPostModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, model: &SysPostModel) -> Result<bool, AppError> {
        self.update_by_id(id, model).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}
