use crate::core::dbal::query::fragments;
use crate::core::dto::sys_notice_dto::{SysNoticeListQueryDto, SysNoticeUpdateReqDto};
use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{MySql, MySqlPool, QueryBuilder, Row};

use crate::core::{errors::AppError, model::system::SysNoticePo};

#[async_trait]
pub trait ISysNoticeRepository: Interface {
    async fn list(&self, query: SysNoticeListQueryDto) -> Result<Vec<SysNoticePo>, AppError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<SysNoticePo>, AppError>;
    async fn insert(
        &self,
        title: &str,
        notice_type: i16,
        status: i16,
        publisher: Option<&str>,
    ) -> Result<u64, AppError>;
    async fn update_by_id(&self, id: u64, dto: SysNoticeUpdateReqDto) -> Result<bool, AppError>;
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysNoticeRepository)]
pub(crate) struct SysNoticeRepository {
    pool: MySqlPool,
}

impl SysNoticeRepository {
    pub(crate) async fn list(
        &self,
        query: SysNoticeListQueryDto,
    ) -> Result<Vec<SysNoticePo>, AppError> {
        let (title_kw, title_like) = fragments::keyword_args(query.title.as_deref());
        let notice_type = query.notice_type.filter(|s| !s.trim().is_empty());
        let status = query.status.filter(|s| !s.trim().is_empty());

        let rows = sqlx::query(
            r#"
            SELECT n.id, n.title, n.notice_type, n.status, IFNULL(u.username, '') AS publisher
            FROM sys_notice n
            LEFT JOIN sys_user u ON n.published_by = u.id
            WHERE n.is_deleted = 0
              AND (? = '' OR n.title LIKE ?)
              AND (? IS NULL OR n.notice_type = ?)
              AND (? IS NULL OR n.status = ?)
            ORDER BY n.id DESC
            "#,
        )
        .bind(&title_kw)
        .bind(&title_like)
        .bind(&notice_type)
        .bind(&notice_type)
        .bind(&status)
        .bind(&status)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询公告失败: {err}")))?;

        Ok(rows
            .into_iter()
            .map(|row| SysNoticePo {
                id: row.get::<u64, _>("id"),
                title: row.get::<String, _>("title"),
                notice_type: row.get::<i16, _>("notice_type"),
                status: row.get::<i16, _>("status"),
                publisher: row.get::<String, _>("publisher"),
            })
            .collect())
    }

    pub(crate) async fn get_by_id(&self, id: u64) -> Result<Option<SysNoticePo>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT n.id, n.title, n.notice_type, n.status, IFNULL(u.username, '') AS publisher
            FROM sys_notice n
            LEFT JOIN sys_user u ON n.published_by = u.id
            WHERE n.id = ? AND n.is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询公告失败: {err}")))?;

        Ok(row.map(|row| SysNoticePo {
            id: row.get::<u64, _>("id"),
            title: row.get::<String, _>("title"),
            notice_type: row.get::<i16, _>("notice_type"),
            status: row.get::<i16, _>("status"),
            publisher: row.get::<String, _>("publisher"),
        }))
    }

    pub(crate) async fn insert(
        &self,
        title: &str,
        notice_type: i16,
        status: i16,
        publisher: Option<&str>,
    ) -> Result<u64, AppError> {
        let publisher_id = self.resolve_user_id_by_username(publisher).await?;
        let content = format!("{title}\n（由系统管理页创建）");

        let result = sqlx::query(
            r#"
            INSERT INTO sys_notice (
                title, notice_type, summary, content, status, published_by, published_at,
                created_by, updated_by, is_deleted
            ) VALUES (?, ?, '', ?, ?, ?, IF(? = 1, NOW(3), NULL), 1, 1, 0)
            "#,
        )
        .bind(title)
        .bind(notice_type)
        .bind(content)
        .bind(status)
        .bind(publisher_id)
        .bind(status)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增公告失败: {err}")))?;
        Ok(result.last_insert_id())
    }

    pub(crate) async fn update_by_id(
        &self,
        id: u64,
        dto: SysNoticeUpdateReqDto,
    ) -> Result<bool, AppError> {
        let mut builder = QueryBuilder::<MySql>::new("UPDATE sys_notice SET ");
        let mut separated = builder.separated(", ");
        let mut has_update = false;

        if let Some(title) = dto.title {
            let content = format!("{title}\n（由系统管理页更新）");
            separated.push("title = ").push_bind(title);
            separated.push("content = ").push_bind(content);
            has_update = true;
        }

        if let Some(notice_type_raw) = dto.notice_type {
            let notice_type = parse_notice_type(Some(notice_type_raw.as_str()))?;
            separated.push("notice_type = ").push_bind(notice_type);
            has_update = true;
        }

        if let Some(status_raw) = dto.status {
            let status = parse_notice_status(Some(status_raw.as_str()))?;
            separated.push("status = ").push_bind(status);
            separated
                .push("published_at = IF(")
                .push_bind(status)
                .push(" = 1, NOW(3), NULL)");
            has_update = true;
        }

        if let Some(publisher) = dto.publisher {
            let publisher_id = self
                .resolve_user_id_by_username(Some(publisher.as_str()))
                .await?;
            separated.push("published_by = ").push_bind(publisher_id);
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
            .map_err(|err| AppError::internal(format!("更新公告失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    pub(crate) async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_notice
            SET is_deleted = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("删除公告失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    async fn resolve_user_id_by_username(
        &self,
        username: Option<&str>,
    ) -> Result<Option<u64>, AppError> {
        let Some(username) = username.map(str::trim).filter(|v| !v.is_empty()) else {
            return Ok(None);
        };

        let user_id = sqlx::query_scalar::<_, u64>(
            r#"
            SELECT id
            FROM sys_user
            WHERE username = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询发布人失败: {err}")))?;
        Ok(user_id)
    }
}

#[async_trait]
impl ISysNoticeRepository for SysNoticeRepository {
    async fn list(&self, query: SysNoticeListQueryDto) -> Result<Vec<SysNoticePo>, AppError> {
        self.list(query).await
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<SysNoticePo>, AppError> {
        self.get_by_id(id).await
    }

    async fn insert(
        &self,
        title: &str,
        notice_type: i16,
        status: i16,
        publisher: Option<&str>,
    ) -> Result<u64, AppError> {
        self.insert(title, notice_type, status, publisher).await
    }

    async fn update_by_id(&self, id: u64, dto: SysNoticeUpdateReqDto) -> Result<bool, AppError> {
        self.update_by_id(id, dto).await
    }

    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.delete_by_id(id).await
    }
}

fn parse_notice_type(raw: Option<&str>) -> Result<i16, AppError> {
    let value = raw
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .ok_or_else(|| AppError::bad_request("公告类型不能为空"))?;

    match value {
        "通知" | "1" => Ok(1),
        "公告" | "2" => Ok(2),
        _ => Err(AppError::bad_request("公告类型非法，仅支持 通知/公告/1/2")),
    }
}

fn parse_notice_status(raw: Option<&str>) -> Result<i16, AppError> {
    let value = raw
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .ok_or_else(|| AppError::bad_request("公告状态不能为空"))?;

    match value {
        "draft" | "0" => Ok(0),
        "published" | "1" => Ok(1),
        "offline" | "2" => Ok(2),
        _ => Err(AppError::bad_request(
            "公告状态非法，仅支持 draft/published/offline/0/1/2",
        )),
    }
}
