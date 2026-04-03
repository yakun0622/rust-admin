use std::sync::Arc;

use sqlx::{MySqlPool, Row};

use crate::{
    core::{errors::AppError, model::log::{LoginLogPo, OperLogPo}},
};

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
                UNIX_TIMESTAMP(oper_at) * 1000 AS oper_at_millis
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
                UNIX_TIMESTAMP(login_at) * 1000 AS login_at_millis
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

fn keyword_args(keyword: Option<&str>) -> (String, String) {
    let kw = keyword.unwrap_or_default().trim().to_string();
    let like = format!("%{kw}%");
    (kw, like)
}
