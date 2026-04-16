use crate::core::dbal::query::fragments;
use crate::core::dto::sys_post_dto::{SysPostListQueryDto, SysPostUpdateReqDto};
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::core::{errors::AppError, model::sys_post::SysPostModel};

#[async_trait]
pub trait ISysPostRepository: Interface {
    async fn list(&self, query: SysPostListQueryDto) -> Result<Vec<SysPostModel>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysPostModel>, AppError>;
    async fn insert(&self, model: &SysPostModel) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysPostUpdateReqDto) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysPostRepository)]
pub(crate) struct SysPostRepository {
    pool: MySqlPool,
}

impl SysPostRepository {
    pub(crate) async fn list(
        &self,
        query: SysPostListQueryDto,
    ) -> Result<Vec<SysPostModel>, AppError> {
        let (name_kw, name_like) = fragments::keyword_args(query.name.as_deref());
        let (code_kw, code_like) = fragments::keyword_args(query.code.as_deref());
        let status = query.status.filter(|s| !s.trim().is_empty());

        sqlx::query_as::<_, SysPostModel>(
            r#"
            SELECT id, post_name, post_code, post_sort, status
            FROM sys_post
            WHERE is_deleted = 0
              AND (? = '' OR post_name LIKE ?)
              AND (? = '' OR post_code LIKE ?)
              AND (? IS NULL OR status = ?)
            ORDER BY id DESC
            "#,
        )
        .bind(&name_kw)
        .bind(&name_like)
        .bind(&code_kw)
        .bind(&code_like)
        .bind(&status)
        .bind(&status)
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
        dto: SysPostUpdateReqDto,
    ) -> Result<bool, AppError> {
        let mut builder = QueryBuilder::<MySql>::new("UPDATE sys_post SET ");
        let mut separated = builder.separated(", ");
        let mut has_update = false;

        if let Some(name) = dto.name {
            separated.push("post_name = ").push_bind(name);
            has_update = true;
        }
        if let Some(code) = dto.code {
            separated.push("post_code = ").push_bind(code);
            has_update = true;
        }
        if let Some(sort) = dto.sort {
            separated.push("post_sort = ").push_bind(sort);
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
    async fn list(&self, query: SysPostListQueryDto) -> Result<Vec<SysPostModel>, AppError> {
        self.list(query).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysPostModel>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(&self, model: &SysPostModel) -> Result<u64, AppError> {
        self.insert(model).await
    }

    async fn update_by_id(&self, id: u64, dto: SysPostUpdateReqDto) -> Result<bool, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}
