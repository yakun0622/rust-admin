use crate::core::dbal::query::fragments;
use sqlx::{MySqlPool, Row};

use crate::core::{errors::AppError, model::system::SysNoticePo};

#[derive(Clone)]
pub(crate) struct SysNoticeRepository {
    pool: MySqlPool,
}

impl SysNoticeRepository {
    pub(crate) fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub(crate) async fn list(&self, keyword: Option<&str>) -> Result<Vec<SysNoticePo>, AppError> {
        let (kw, like) = fragments::keyword_args(keyword);
        let rows = sqlx::query(
            r#"
            SELECT n.id, n.title, n.notice_type, n.status, IFNULL(u.username, '') AS publisher
            FROM sys_notice n
            LEFT JOIN sys_user u ON n.published_by = u.id
            WHERE n.is_deleted = 0
              AND (? = '' OR n.title LIKE ? OR IFNULL(u.username, '') LIKE ?)
            ORDER BY n.id DESC
            "#,
        )
        .bind(kw)
        .bind(&like)
        .bind(&like)
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
        title: &str,
        notice_type: i16,
        status: i16,
        publisher: Option<&str>,
    ) -> Result<bool, AppError> {
        let publisher_id = self.resolve_user_id_by_username(publisher).await?;
        let content = format!("{title}\n（由系统管理页更新）");

        let result = sqlx::query(
            r#"
            UPDATE sys_notice
            SET title = ?, notice_type = ?, content = ?, status = ?, published_by = ?,
                published_at = IF(? = 1, NOW(3), NULL), updated_by = 1
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(title)
        .bind(notice_type)
        .bind(content)
        .bind(status)
        .bind(publisher_id)
        .bind(status)
        .bind(id)
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
