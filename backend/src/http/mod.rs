use std::sync::Arc;
use std::net::SocketAddr;

use anyhow::Context;
use sqlx::sqlite::SqlitePool;

mod posts;
mod error;
mod www;

use crate::config::Config;
use crate::app_state::AppState;

/// Result type used on all services
pub type Result<T> = std::result::Result<T, error::Error>;

/// Main application router
fn api_router(state: AppState) -> Result<axum::Router> {
    Ok(axum::Router::new()
        .merge(posts::router())
        .merge(www::router(&state.config.www_dir)?)
        .with_state(state))
}

pub async fn serve(
    config: Config,
    db_pool: SqlitePool
) -> anyhow::Result<()> {
    let config = Arc::new(config);
    let state = AppState {
        config: config.clone(),
        db_pool
    };

    // Create the server app
    let app = api_router(state)?;

    // Serve
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("Error running HTTP server")
}
