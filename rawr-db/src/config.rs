use sqlx::PgPool;
use envconfig::Envconfig;

use rawr_config::{ConfigError, load_config};

#[derive(Envconfig, Debug, Clone)]
pub struct DbConfig {
    #[envconfig(from = "DB_HOST", default = "localhost")]
    pub host: String,
    #[envconfig(from = "DB_PORT", default = "5432")]
    pub port: u16,
    #[envconfig(from = "DB_USER")]
    pub user: String,
    #[envconfig(from = "DB_PASSWORD")]
    pub password: String,
    #[envconfig(from = "DB_NAME", default = "rawr")]
    pub database: String,
    #[envconfig(from = "DB_MAX_CONNECTIONS", default = "10")]
    pub max_connections: u32,
}

impl DbConfig {
    pub fn load() -> Result<Self, ConfigError> {
        load_config()
    }

    pub fn from_env() -> Result<Self, ConfigError> {
        Self::load()
    }

    pub fn connection_string(&self) -> String {
        // TODO(nick): ?sslmode=verify-full
        format!(
            "postgresql://{user}:{password}@{host}:{port}/{database}",
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port,
            database = self.database,
        )
    }
}

pub async fn create_pool(config: &DbConfig) -> Result<PgPool, sqlx::Error> {
    let connection_string = config.connection_string();
    PgPool::connect_with(connection_string.parse().map_err(|e| {
        sqlx::Error::Configuration(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("connection string is invalid: {}", e),
        )))
    })?)
    .await
}

pub async fn create_pool_from_env() -> Result<PgPool, Box<dyn std::error::Error>> {
    let config = DbConfig::from_env().map_err(|e| format!("failed to load database config: {}", e))?;
    Ok(create_pool(&config).await?)
}

