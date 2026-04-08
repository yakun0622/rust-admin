use std::sync::Arc;

use anyhow::Context;

use crate::{
    core::{config::DatabaseDriver, db::DbPool},
    modules::{
        auth::repository::{AuthRepository, MySqlAuthRepository, PostgresAuthRepository},
        log::repository::{LogRepository, MySqlLogRepository, PostgresLogRepository},
        system::repository::{
            postgres::PostgresSystemRepository, MySqlSystemRepository, SystemRepository,
        },
    },
};

pub struct RepositoryBundle {
    pub auth_repo: Arc<dyn AuthRepository>,
    pub log_repo: Arc<dyn LogRepository>,
    pub system_repo: Arc<dyn SystemRepository>,
}

pub fn build_repository_bundle(
    driver: DatabaseDriver,
    db_pool: &DbPool,
) -> anyhow::Result<RepositoryBundle> {
    match driver {
        DatabaseDriver::MySql => {
            let mysql_pool = db_pool
                .as_mysql()
                .context("database.driver=mysql but MySQL pool is unavailable")?;

            Ok(RepositoryBundle {
                auth_repo: MySqlAuthRepository::new(mysql_pool.clone()),
                log_repo: MySqlLogRepository::new(mysql_pool.clone()),
                system_repo: MySqlSystemRepository::new(mysql_pool),
            })
        }
        DatabaseDriver::Postgres => {
            let pg_pool = db_pool
                .as_postgres()
                .context("database.driver=postgres but PostgreSQL pool is unavailable")?;

            Ok(RepositoryBundle {
                auth_repo: PostgresAuthRepository::new(pg_pool.clone()),
                log_repo: PostgresLogRepository::new(pg_pool.clone()),
                system_repo: PostgresSystemRepository::new(pg_pool),
            })
        }
    }
}
