use anyhow::Context;
use sqlx::{query_scalar, MySqlPool, PgPool};

#[derive(Debug, Clone)]
pub enum DbPool {
    MySql(MySqlPool),
    Postgres(PgPool),
}

impl DbPool {
    pub fn driver_name(&self) -> &'static str {
        match self {
            Self::MySql(_) => "mysql",
            Self::Postgres(_) => "postgres",
        }
    }

    pub fn as_mysql(&self) -> Option<MySqlPool> {
        match self {
            Self::MySql(pool) => Some(pool.clone()),
            Self::Postgres(_) => None,
        }
    }

    pub fn as_postgres(&self) -> Option<PgPool> {
        match self {
            Self::MySql(_) => None,
            Self::Postgres(pool) => Some(pool.clone()),
        }
    }

    pub async fn ping(&self) -> anyhow::Result<()> {
        match self {
            Self::MySql(pool) => {
                let _: i32 = query_scalar("SELECT 1")
                    .fetch_one(pool)
                    .await
                    .context("mysql ping query failed")?;
                Ok(())
            }
            Self::Postgres(pool) => {
                let _: i32 = query_scalar("SELECT 1")
                    .fetch_one(pool)
                    .await
                    .context("postgres ping query failed")?;
                Ok(())
            }
        }
    }
}
