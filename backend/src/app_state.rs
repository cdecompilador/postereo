use std::sync::Arc;

use sqlx::sqlite::SqlitePool;

use crate::config::Config;

/// Shared state across all web controllers
#[derive(Clone)]
pub struct AppState {
    /// Initial web app configurations
    pub config: Arc<Config>,

    /// Connection to sqlite databse
    pub db_pool: SqlitePool,
}
