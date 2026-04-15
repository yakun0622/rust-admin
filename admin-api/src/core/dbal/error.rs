use sqlx::Error;

use crate::core::errors::AppError;

pub fn map_sqlx_error(context: &str, error: Error) -> AppError {
    match error {
        Error::RowNotFound => AppError::not_found(format!("{context}: 记录不存在")),
        Error::Database(db_err) => AppError::internal(format!(
            "{context}: {} (code: {})",
            db_err.message(),
            db_err.code().unwrap_or_default()
        )),
        other => AppError::internal(format!("{context}: {other}")),
    }
}
