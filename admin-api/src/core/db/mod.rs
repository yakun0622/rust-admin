use sqlx::{query_scalar, MySqlPool};

use crate::core::errors::AppError;

#[derive(Debug, Clone)]
pub enum DbPool {
    MySql(MySqlPool),
}

impl DbPool {
    pub fn driver_name(&self) -> &'static str {
        match self {
            Self::MySql(_) => "mysql",
        }
    }

    pub fn as_mysql(&self) -> Option<MySqlPool> {
        match self {
            Self::MySql(pool) => Some(pool.clone()),
        }
    }

    pub async fn ping(&self) -> Result<(), AppError> {
        match self {
            Self::MySql(pool) => {
                let _: i32 = query_scalar("SELECT 1")
                    .fetch_one(pool)
                    .await
                    .map_err(|e| AppError::internal(format!("mysql ping failed: {e}")))?;
                Ok(())
            }
        }
    }
}
