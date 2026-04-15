use std::sync::Arc;

use crate::core::config::DatabaseDriver;

pub trait SqlDialect: Send + Sync {
    fn driver(&self) -> DatabaseDriver;

    fn driver_name(&self) -> &'static str {
        self.driver().as_str()
    }

    fn like_operator(&self) -> &'static str;

    fn coalesce(&self, expr: &str, fallback: &str) -> String;

    fn now_millis_expr(&self, column: &str) -> String;

    fn supports_returning(&self) -> bool;

    fn returning_id_clause(&self, id_column: &str) -> String;
}

#[derive(Debug, Default)]
pub struct MySqlDialect;

impl SqlDialect for MySqlDialect {
    fn driver(&self) -> DatabaseDriver {
        DatabaseDriver::MySql
    }

    fn like_operator(&self) -> &'static str {
        "LIKE"
    }

    fn coalesce(&self, expr: &str, fallback: &str) -> String {
        format!("IFNULL({expr}, {fallback})")
    }

    fn now_millis_expr(&self, column: &str) -> String {
        format!("CAST(UNIX_TIMESTAMP({column}) * 1000 AS SIGNED)")
    }

    fn supports_returning(&self) -> bool {
        false
    }

    fn returning_id_clause(&self, _id_column: &str) -> String {
        String::new()
    }
}
pub fn from_driver(_driver: DatabaseDriver) -> Arc<dyn SqlDialect> {
    Arc::new(MySqlDialect)
}
