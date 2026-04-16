use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{MySqlPool, Row};

use crate::core::{
    dbal::query::fragments,
    errors::AppError,
    model::log::{LoginLogPo, OperLogCreatePo, OperLogPo},
};

#[async_trait]
pub trait ISysLogRepository: Interface {
    async fn list_oper(&self, keyword: Option<&str>) -> Result<Vec<OperLogPo>, AppError>;
    async fn list_login(&self, keyword: Option<&str>) -> Result<Vec<LoginLogPo>, AppError>;
    async fn append_oper(&self, input: OperLogCreatePo) -> Result<(), AppError>;
}

#[derive(Component, Clone)]
#[shaku(interface = ISysLogRepository)]
pub struct SysLogRepository {
    pool: MySqlPool,
}

impl SysLogRepository {
    pub(crate) async fn append_oper(&self, input: OperLogCreatePo) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO sys_oper_log (
                module, business_type, method, request_method, operator_type, oper_name, dept_name,
                url, ip, location, request_params, response_data, status, error_msg, user_agent, os, duration_ms
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(input.module)
        .bind(input.business_type)
        .bind(input.method)
        .bind(input.request_method)
        .bind(input.operator_type)
        .bind(input.oper_name)
        .bind(input.dept_name)
        .bind(input.url)
        .bind(input.ip)
        .bind(input.location)
        .bind(input.request_params)
        .bind(input.response_data)
        .bind(input.status)
        .bind(input.error_msg)
        .bind(input.user_agent)
        .bind(input.os)
        .bind(input.duration_ms)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("写入操作日志失败: {err}")))?;
        Ok(())
    }

    pub(crate) async fn list_oper(
        &self,
        keyword: Option<&str>,
    ) -> Result<Vec<OperLogPo>, AppError> {
        let (kw, like) = fragments::keyword_args(keyword);
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                IFNULL(method, module) AS module_text,
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
                IFNULL(location, '') AS location_text,
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

        Ok(rows
            .into_iter()
            .map(|row| OperLogPo {
                id: row.get::<u64, _>("id"),
                module: row.get::<String, _>("module_text"),
                business_type: row.get::<String, _>("business_type_text"),
                request_method: row.get::<String, _>("request_method_text"),
                oper_name: row.get::<String, _>("oper_name_text"),
                ip: row.get::<String, _>("ip_text"),
                location: row.get::<String, _>("location_text"),
                status: row.get::<String, _>("status_text"),
                duration_ms: row.get::<u32, _>("duration_ms"),
                oper_at: row.get::<i64, _>("oper_at_millis"),
            })
            .collect::<Vec<_>>())
    }

    pub(crate) async fn list_login(
        &self,
        keyword: Option<&str>,
    ) -> Result<Vec<LoginLogPo>, AppError> {
        let (kw, like) = fragments::keyword_args(keyword);
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
                IFNULL(location, '') AS location_text,
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

        Ok(rows
            .into_iter()
            .map(|row| LoginLogPo {
                id: row.get::<u64, _>("id"),
                username: row.get::<String, _>("username_text"),
                login_type: row.get::<String, _>("login_type_text"),
                ip: row.get::<String, _>("ip_text"),
                location: row.get::<String, _>("location_text"),
                status: row.get::<String, _>("status_text"),
                message: row.get::<String, _>("message_text"),
                login_at: row.get::<i64, _>("login_at_millis"),
            })
            .collect::<Vec<_>>())
    }
}

#[async_trait]
impl ISysLogRepository for SysLogRepository {
    async fn list_oper(&self, keyword: Option<&str>) -> Result<Vec<OperLogPo>, AppError> {
        self.list_oper(keyword).await
    }

    async fn list_login(&self, keyword: Option<&str>) -> Result<Vec<LoginLogPo>, AppError> {
        self.list_login(keyword).await
    }

    async fn append_oper(&self, input: OperLogCreatePo) -> Result<(), AppError> {
        self.append_oper(input).await
    }
}
