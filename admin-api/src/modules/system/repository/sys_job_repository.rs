use sqlx::{MySqlPool, Row};

use crate::core::{
    dbal::query::fragments,
    dto::monitor_dto::{JobLogQueryDto, JobUpsertReqDto},
    errors::AppError,
    model::monitor::{JobLogPo, JobPo},
};

#[derive(Debug, Clone)]
pub struct JobExecutionLogInput {
    pub job_id: u64,
    pub job_name: String,
    pub job_group: String,
    pub invoke_target: String,
    pub cron_expression: String,
    pub status: bool,
    pub message: String,
    pub exception_info: Option<String>,
    pub trigger_type: String,
    pub started_at_millis: i64,
    pub finished_at_millis: i64,
    pub duration_ms: u32,
    pub next_run_at_millis: Option<i64>,
}

#[derive(Clone)]
pub struct SysJobRepository {
    pool: MySqlPool,
}

impl SysJobRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn list_jobs(&self, keyword: Option<&str>) -> Result<Vec<JobPo>, AppError> {
        let (kw, like) = fragments::keyword_args(keyword);
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                job_name,
                job_group,
                invoke_target,
                cron_expression,
                concurrent,
                status,
                IFNULL(remark, '') AS remark_text,
                CAST(UNIX_TIMESTAMP(last_run_at) * 1000 AS SIGNED) AS last_run_at_millis,
                CAST(UNIX_TIMESTAMP(next_run_at) * 1000 AS SIGNED) AS next_run_at_millis
            FROM sys_job
            WHERE is_deleted = 0
              AND (? = '' OR job_name LIKE ? OR job_group LIKE ? OR invoke_target LIKE ?)
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
        .map_err(|err| AppError::internal(format!("查询定时任务失败: {err}")))?;

        Ok(rows.into_iter().map(map_job_row).collect())
    }

    pub async fn list_job_logs(&self, query: &JobLogQueryDto) -> Result<Vec<JobLogPo>, AppError> {
        let (kw, like) = fragments::keyword_args(query.keyword.as_deref());
        let job_id_filter = query.job_id.unwrap_or(0);
        let status_filter = match query.status.as_deref().map(str::trim) {
            Some("success") => 1_i8,
            Some("failed") => 0_i8,
            _ => -1_i8,
        };
        let safe_limit = query.limit.unwrap_or(200).clamp(1, 500) as i64;

        let rows = sqlx::query(
            r#"
            SELECT
                id,
                job_id,
                job_name,
                job_group,
                invoke_target,
                cron_expression,
                CASE status WHEN 1 THEN 'success' ELSE 'failed' END AS status_text,
                IFNULL(message, '') AS message_text,
                exception_info,
                IFNULL(trigger_type, 'auto') AS trigger_type_text,
                CAST(UNIX_TIMESTAMP(started_at) * 1000 AS SIGNED) AS started_at_millis,
                CAST(UNIX_TIMESTAMP(finished_at) * 1000 AS SIGNED) AS finished_at_millis,
                duration_ms
            FROM sys_job_log
            WHERE (? = 0 OR job_id = ?)
              AND (? < 0 OR status = ?)
              AND (? = '' OR job_name LIKE ? OR job_group LIKE ? OR invoke_target LIKE ? OR IFNULL(message, '') LIKE ?)
            ORDER BY id DESC
            LIMIT ?
            "#,
        )
        .bind(job_id_filter)
        .bind(job_id_filter)
        .bind(status_filter)
        .bind(status_filter)
        .bind(&kw)
        .bind(&like)
        .bind(&like)
        .bind(&like)
        .bind(&like)
        .bind(safe_limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询任务执行日志失败: {err}")))?;

        Ok(rows
            .into_iter()
            .map(|row| JobLogPo {
                id: row.get::<u64, _>("id"),
                job_id: row.get::<u64, _>("job_id"),
                job_name: row.get::<String, _>("job_name"),
                job_group: row.get::<String, _>("job_group"),
                invoke_target: row.get::<String, _>("invoke_target"),
                cron_expression: row.get::<String, _>("cron_expression"),
                status: row.get::<String, _>("status_text"),
                message: row.get::<String, _>("message_text"),
                exception_info: row.get::<Option<String>, _>("exception_info"),
                trigger_type: row.get::<String, _>("trigger_type_text"),
                started_at: row.get::<i64, _>("started_at_millis"),
                finished_at: row.get::<Option<i64>, _>("finished_at_millis"),
                duration_ms: row.get::<u32, _>("duration_ms"),
            })
            .collect::<Vec<_>>())
    }

    pub async fn get_job_by_id(&self, id: u64) -> Result<Option<JobPo>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                job_name,
                job_group,
                invoke_target,
                cron_expression,
                concurrent,
                status,
                IFNULL(remark, '') AS remark_text,
                CAST(UNIX_TIMESTAMP(last_run_at) * 1000 AS SIGNED) AS last_run_at_millis,
                CAST(UNIX_TIMESTAMP(next_run_at) * 1000 AS SIGNED) AS next_run_at_millis
            FROM sys_job
            WHERE id = ? AND is_deleted = 0
            LIMIT 1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("查询定时任务失败: {err}")))?;

        Ok(row.map(map_job_row))
    }

    pub async fn create_job(&self, payload: JobUpsertReqDto) -> Result<JobPo, AppError> {
        let status = to_status_value(payload.status.as_deref());
        let concurrent = payload.concurrent.unwrap_or(false);
        let remark = payload.remark.unwrap_or_default();

        let result = sqlx::query(
            r#"
            INSERT INTO sys_job (
                job_name,
                job_group,
                invoke_target,
                cron_expression,
                misfire_policy,
                concurrent,
                status,
                remark,
                created_by,
                updated_by,
                is_deleted
            ) VALUES (?, ?, ?, ?, 1, ?, ?, ?, 1, 1, 0)
            "#,
        )
        .bind(&payload.job_name)
        .bind(&payload.job_group)
        .bind(&payload.invoke_target)
        .bind(&payload.cron_expression)
        .bind(bool_to_i8(concurrent))
        .bind(status)
        .bind(&remark)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("新增定时任务失败: {err}")))?;

        self.get_job_by_id(result.last_insert_id())
            .await?
            .ok_or_else(|| AppError::internal("新增定时任务后读取失败"))
    }

    pub async fn update_job(
        &self,
        id: u64,
        payload: JobUpsertReqDto,
    ) -> Result<Option<JobPo>, AppError> {
        let status = to_status_value(payload.status.as_deref());
        let concurrent = payload.concurrent.unwrap_or(false);
        let remark = payload.remark.unwrap_or_default();
        let result = sqlx::query(
            r#"
            UPDATE sys_job
            SET
                job_name = ?,
                job_group = ?,
                invoke_target = ?,
                cron_expression = ?,
                concurrent = ?,
                status = ?,
                remark = ?,
                next_run_at = CASE WHEN ? = 1 THEN next_run_at ELSE NULL END,
                updated_by = 1,
                updated_at = CURRENT_TIMESTAMP(3)
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(&payload.job_name)
        .bind(&payload.job_group)
        .bind(&payload.invoke_target)
        .bind(&payload.cron_expression)
        .bind(bool_to_i8(concurrent))
        .bind(status)
        .bind(&remark)
        .bind(status)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新定时任务失败: {err}")))?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        self.get_job_by_id(id).await
    }

    pub async fn delete_job(&self, id: u64) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE sys_job
            SET is_deleted = 1, updated_by = 1, updated_at = CURRENT_TIMESTAMP(3)
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("删除定时任务失败: {err}")))?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn set_job_status(&self, id: u64, status: &str) -> Result<Option<JobPo>, AppError> {
        let status_value = to_status_value(Some(status));
        let result = sqlx::query(
            r#"
            UPDATE sys_job
            SET
                status = ?,
                next_run_at = CASE WHEN ? = 1 THEN next_run_at ELSE NULL END,
                updated_by = 1,
                updated_at = CURRENT_TIMESTAMP(3)
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(status_value)
        .bind(status_value)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新任务状态失败: {err}")))?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        self.get_job_by_id(id).await
    }

    pub async fn update_next_run_at(
        &self,
        id: u64,
        next_run_at_millis: Option<i64>,
    ) -> Result<(), AppError> {
        let next_secs = next_run_at_millis.map(millis_to_unix_seconds);
        sqlx::query(
            r#"
            UPDATE sys_job
            SET
                next_run_at = CASE
                    WHEN ? IS NULL THEN NULL
                    ELSE FROM_UNIXTIME(?)
                END,
                updated_by = 1,
                updated_at = CURRENT_TIMESTAMP(3)
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(next_secs)
        .bind(next_secs)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|err| AppError::internal(format!("更新下次执行时间失败: {err}")))?;
        Ok(())
    }

    pub async fn record_job_execution(&self, input: JobExecutionLogInput) -> Result<(), AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|err| AppError::internal(format!("开启任务日志事务失败: {err}")))?;

        let started_secs = millis_to_unix_seconds(input.started_at_millis);
        let finished_secs = millis_to_unix_seconds(input.finished_at_millis);
        let next_secs = input.next_run_at_millis.map(millis_to_unix_seconds);
        let status_value = if input.status { 1_i8 } else { 0_i8 };

        sqlx::query(
            r#"
            INSERT INTO sys_job_log (
                job_id,
                job_name,
                job_group,
                invoke_target,
                cron_expression,
                status,
                message,
                exception_info,
                trigger_type,
                started_at,
                finished_at,
                duration_ms,
                created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, FROM_UNIXTIME(?), FROM_UNIXTIME(?), ?, CURRENT_TIMESTAMP(3))
            "#,
        )
        .bind(input.job_id)
        .bind(&input.job_name)
        .bind(&input.job_group)
        .bind(&input.invoke_target)
        .bind(&input.cron_expression)
        .bind(status_value)
        .bind(&input.message)
        .bind(input.exception_info.as_deref())
        .bind(&input.trigger_type)
        .bind(started_secs)
        .bind(finished_secs)
        .bind(input.duration_ms)
        .execute(&mut *tx)
        .await
        .map_err(|err| AppError::internal(format!("写入任务日志失败: {err}")))?;

        sqlx::query(
            r#"
            UPDATE sys_job
            SET
                last_run_at = FROM_UNIXTIME(?),
                next_run_at = CASE
                    WHEN ? IS NULL THEN NULL
                    ELSE FROM_UNIXTIME(?)
                END,
                updated_by = 1,
                updated_at = CURRENT_TIMESTAMP(3)
            WHERE id = ? AND is_deleted = 0
            "#,
        )
        .bind(started_secs)
        .bind(next_secs)
        .bind(next_secs)
        .bind(input.job_id)
        .execute(&mut *tx)
        .await
        .map_err(|err| AppError::internal(format!("更新任务执行时间失败: {err}")))?;

        tx.commit()
            .await
            .map_err(|err| AppError::internal(format!("提交任务日志事务失败: {err}")))?;
        Ok(())
    }
}

fn map_job_row(row: sqlx::mysql::MySqlRow) -> JobPo {
    JobPo {
        id: row.get::<u64, _>("id"),
        job_name: row.get::<String, _>("job_name"),
        job_group: row.get::<String, _>("job_group"),
        invoke_target: row.get::<String, _>("invoke_target"),
        cron_expression: row.get::<String, _>("cron_expression"),
        concurrent: row.get::<i8, _>("concurrent") == 1,
        status: from_status_value(row.get::<i8, _>("status")).to_string(),
        remark: row.get::<String, _>("remark_text"),
        last_run_at: row.get::<Option<i64>, _>("last_run_at_millis"),
        next_run_at: row.get::<Option<i64>, _>("next_run_at_millis"),
    }
}

fn to_status_value(status: Option<&str>) -> i8 {
    if normalize_job_status(status) == "running" {
        1
    } else {
        0
    }
}

fn from_status_value(status: i8) -> &'static str {
    if status == 1 {
        "running"
    } else {
        "paused"
    }
}

fn bool_to_i8(value: bool) -> i8 {
    if value { 1 } else { 0 }
}

fn millis_to_unix_seconds(value: i64) -> i64 {
    value.div_euclid(1000)
}

fn normalize_job_status(status: Option<&str>) -> &'static str {
    match status.map(|value| value.trim().to_lowercase()) {
        Some(value) if value == "paused" => "paused",
        _ => "running",
    }
}
