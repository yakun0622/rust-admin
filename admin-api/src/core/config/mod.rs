use anyhow::Context;
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app: RuntimeConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct AppConfigRaw {
    pub app: RuntimeConfig,
    pub server: ServerConfig,
    pub database: Option<DatabaseConfig>,
    pub mysql: Option<LegacyMySqlConfig>,
    pub redis: RedisConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuntimeConfig {
    pub name: String,
    pub env: String,
    pub log_level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    #[serde(default)]
    pub driver: DatabaseDriver,
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_secs: u64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseDriver {
    MySql,
    Postgres,
}

impl Default for DatabaseDriver {
    fn default() -> Self {
        Self::MySql
    }
}

impl DatabaseDriver {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MySql => "mysql",
            Self::Postgres => "postgres",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::MySql => "MySQL",
            Self::Postgres => "PostgreSQL",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LegacyMySqlConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_secs: u64,
}

impl From<LegacyMySqlConfig> for DatabaseConfig {
    fn from(value: LegacyMySqlConfig) -> Self {
        Self {
            driver: DatabaseDriver::MySql,
            url: value.url,
            max_connections: value.max_connections,
            min_connections: value.min_connections,
            acquire_timeout_secs: value.acquire_timeout_secs,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
    pub connection_timeout_secs: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_expires_secs: u64,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let _ = dotenvy::dotenv();
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());

        let settings = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{env}")).required(false))
            .build()
            .context("failed to build app config")?;

        let raw = settings
            .try_deserialize::<AppConfigRaw>()
            .context("failed to deserialize app config")?;

        let database = match (raw.database, raw.mysql) {
            (Some(database), _) => database,
            (None, Some(mysql)) => mysql.into(),
            (None, None) => {
                return Err(anyhow::anyhow!(
                    "missing [database] config (or legacy [mysql] config)"
                ))
            }
        };

        Ok(Self {
            app: raw.app,
            server: raw.server,
            database,
            redis: raw.redis,
            security: raw.security,
        })
    }
}
