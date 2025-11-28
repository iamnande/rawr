use sqlx::{PgPool, Pool, Postgres};

pub type DbPool = Pool<Postgres>;

mod config;

pub use config::{DbConfig, create_pool, create_pool_from_env};

pub async fn run_migrations(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use sqlx::migrate::Migrator;
    use std::path::Path;

    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(pool).await?;
    Ok(())
}
