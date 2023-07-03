use anyhow::*;
use clap::Parser;
use sqlx::sqlite::SqlitePool;

use backend::config::Config;
use backend::http;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env
    dotenv::dotenv()?;

    // Initialize logger
    pretty_env_logger::init();

    // Load configuration
    let config = Config::parse();

    // Connect to the sqlite database
    let db_pool = SqlitePool::connect(config.database_url.as_ref()).await?;

    // Automatically apply all the migrations to the database
    sqlx::migrate!()
        .run(&db_pool).await?;

    // Serve the web app
    http::serve(config, db_pool).await?;

    Ok(())
}
