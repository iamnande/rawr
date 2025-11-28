use sqlx::migrate::Migrator;
use std::path::Path;

use rawr_db::create_pool_from_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hang on, i'm DOING SOMETHING (database migrations)...");

    let pool = create_pool_from_env().await?;

    let migrations_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("migrations");

    let migrator = Migrator::new(migrations_path).await?;
    migrator.run(&pool).await?;

    println!("database migrations applied successfully, have a righteous query day!");
    Ok(())
}
