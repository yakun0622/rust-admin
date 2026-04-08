use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{MySqlPool, PgPool, Row};

use crate::core::{
    errors::AppError,
    model::log::{LoginLogPo, OperLogPo},
};

#[async_trait]
pub trait LogRepository: Send + Sync {
    async fn list_oper(&self, keyword: Option<&str>) -> Result<Vec<OperLogPo>, AppError>;
    async fn list_login(&self, keyword: Option<&str>) -> Result<Vec<LoginLogPo>, AppError>;
}

#[derive(Debug, Clone)]
pub struct MySqlLogRepository {
    pool: MySqlPool,
}

impl MySqlLogRepository {
    pub fn new(pool: MySqlPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    pub async fn list_oper(&self, keyword: Option<&str>) -> Result<Vec<OperLogPo>, AppError> {
        let (kw, like) = keyword_args(keyword);
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                module,
                CASE business_type
                    WHEN 1 THEN 'create'
                    WHEN 2 THEN 'update'
                    WHEN 3 THEN 'delete'
                    WHEN 4 THEN 'grant'
                    ELSE CONCAT('type-', business_type)
                END AS business_type_text,
                IFNULL(request_method, '') AS request_method_text,
                IFNULL(oper_name, '') AS oper_name_text,
                IFNULL(ip, '') AS ip_text,
                CASE status WHEN 1 THEN 'success' ELSE 'failed' END AS status_text,
                duration_ms,
                CAST(UNIX_TIMESTAMP(oper_at) * 1000 AS SIGNED) AS oper_at_millis
            FROM sys_oper_log
            WHERE (? = '' OR module LIKE ? OR IFNULL(oper_name, '') LIKE ? OR IFNULL(ip, '') LIKE ?)
            ORDER BY id DESC
            LIMIT 500
            "#,
        )
        .bind(&kw)
        .bind(&like)
        .bind(&like)
        .bind(&like)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询操作日志失败: {err}")))?;

        let result = rows
            .into_iter()
            .map(|row| OperLogPo {
                id: row.get::<u64, _>("id"),
                module: row.get::<String, _>("module"),
                business_type: row.get::<String, _>("business_type_text"),
                request_method: row.get::<String, _>("request_method_text"),
                oper_name: row.get::<String, _>("oper_name_text"),
                ip: row.get::<String, _>("ip_text"),
                status: row.get::<String, _>("status_text"),
                duration_ms: row.get::<u32, _>("duration_ms"),
                oper_at: row.get::<i64, _>("oper_at_millis"),
            })
            .collect::<Vec<_>>();
        Ok(result)
    }

    pub async fn list_login(&self, keyword: Option<&str>) -> Result<Vec<LoginLogPo>, AppError> {
        let (kw, like) = keyword_args(keyword);
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                IFNULL(username, '') AS username_text,
                CASE login_type
                    WHEN 1 THEN 'login'
                    WHEN 2 THEN 'logout'
                    ELSE 'fail'
                END AS login_type_text,
                IFNULL(ip, '') AS ip_text,
                CASE status WHEN 1 THEN 'success' ELSE 'failed' END AS status_text,
                IFNULL(message, '') AS message_text,
                CAST(UNIX_TIMESTAMP(login_at) * 1000 AS SIGNED) AS login_at_millis
            FROM sys_login_log
            WHERE (? = '' OR IFNULL(username, '') LIKE ? OR IFNULL(ip, '') LIKE ? OR IFNULL(message, '') LIKE ?)
            ORDER BY id DESC
            LIMIT 500
            "#,
        )
        .bind(&kw)
        .bind(&like)
        .bind(&like)
        .bind(&like)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询登录日志失败: {err}")))?;

        let result = rows
            .into_iter()
            .map(|row| LoginLogPo {
                id: row.get::<u64, _>("id"),
                username: row.get::<String, _>("username_text"),
                login_type: row.get::<String, _>("login_type_text"),
                ip: row.get::<String, _>("ip_text"),
                status: row.get::<String, _>("status_text"),
                message: row.get::<String, _>("message_text"),
                login_at: row.get::<i64, _>("login_at_millis"),
            })
            .collect::<Vec<_>>();
        Ok(result)
    }
}

#[async_trait]
impl LogRepository for MySqlLogRepository {
    async fn list_oper(&self, keyword: Option<&str>) -> Result<Vec<OperLogPo>, AppError> {
        MySqlLogRepository::list_oper(self, keyword).await
    }

    async fn list_login(&self, keyword: Option<&str>) -> Result<Vec<LoginLogPo>, AppError> {
        MySqlLogRepository::list_login(self, keyword).await
    }
}

#[derive(Debug, Clone)]
pub struct PostgresLogRepository {
    pool: PgPool,
}

impl PostgresLogRepository {
    pub fn new(pool: PgPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }
}

#[async_trait]
impl LogRepository for PostgresLogRepository {
    async fn list_oper(&self, keyword: Option<&str>) -> Result<Vec<OperLogPo>, AppError> {
        let like = keyword_like(keyword);
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                module,
                CASE business_type
                    WHEN 1 THEN 'create'
                    WHEN 2 THEN 'update'
                    WHEN 3 THEN 'delete'
                    WHEN 4 THEN 'grant'
                    ELSE 'type-' || business_type::text
                END AS business_type_text,
                COALESCE(request_method, '') AS request_method_text,
                COALESCE(oper_name, '') AS oper_name_text,
                COALESCE(ip, '') AS ip_text,
                CASE status WHEN 1 THEN 'success' ELSE 'failed' END AS status_text,
                duration_ms,
                (EXTRACT(EPOCH FROM oper_at) * 1000)::BIGINT AS oper_at_millis
            FROM sys_oper_log
            WHERE ($1::text IS NULL OR module ILIKE $1 OR COALESCE(oper_name, '') ILIKE $1 OR COALESCE(ip, '') ILIKE $1)
            ORDER BY id DESC
            LIMIT 500
            "#,
        )
        .bind(like)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询操作日志失败: {err}")))?;

        let result = rows
            .into_iter()
            .map(|row| OperLogPo {
                id: row.get::<i64, _>("id") as u64,
                module: row.get::<String, _>("module"),
                business_type: row.get::<String, _>("business_type_text"),
                request_method: row.get::<String, _>("request_method_text"),
                oper_name: row.get::<String, _>("oper_name_text"),
                ip: row.get::<String, _>("ip_text"),
                status: row.get::<String, _>("status_text"),
                duration_ms: row.get::<i32, _>("duration_ms") as u32,
                oper_at: row.get::<i64, _>("oper_at_millis"),
            })
            .collect::<Vec<_>>();
        Ok(result)
    }

    async fn list_login(&self, keyword: Option<&str>) -> Result<Vec<LoginLogPo>, AppError> {
        let like = keyword_like(keyword);
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                COALESCE(username, '') AS username_text,
                CASE login_type
                    WHEN 1 THEN 'login'
                    WHEN 2 THEN 'logout'
                    ELSE 'fail'
                END AS login_type_text,
                COALESCE(ip, '') AS ip_text,
                CASE status WHEN 1 THEN 'success' ELSE 'failed' END AS status_text,
                COALESCE(message, '') AS message_text,
                (EXTRACT(EPOCH FROM login_at) * 1000)::BIGINT AS login_at_millis
            FROM sys_login_log
            WHERE ($1::text IS NULL OR COALESCE(username, '') ILIKE $1 OR COALESCE(ip, '') ILIKE $1 OR COALESCE(message, '') ILIKE $1)
            ORDER BY id DESC
            LIMIT 500
            "#,
        )
        .bind(like)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询登录日志失败: {err}")))?;

        let result = rows
            .into_iter()
            .map(|row| LoginLogPo {
                id: row.get::<i64, _>("id") as u64,
                username: row.get::<String, _>("username_text"),
                login_type: row.get::<String, _>("login_type_text"),
                ip: row.get::<String, _>("ip_text"),
                status: row.get::<String, _>("status_text"),
                message: row.get::<String, _>("message_text"),
                login_at: row.get::<i64, _>("login_at_millis"),
            })
            .collect::<Vec<_>>();
        Ok(result)
    }
}

fn keyword_args(keyword: Option<&str>) -> (String, String) {
    let kw = keyword.unwrap_or_default().trim().to_string();
    let like = format!("%{kw}%");
    (kw, like)
}

fn keyword_like(keyword: Option<&str>) -> Option<String> {
    let kw = keyword.unwrap_or_default().trim();
    if kw.is_empty() {
        None
    } else {
        Some(format!("%{kw}%"))
    }
}
