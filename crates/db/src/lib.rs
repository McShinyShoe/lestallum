pub mod entities;

use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use shared::app_config::DatabaseConfig;

pub async fn connect(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(config.url.clone());
    options
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(Duration::from_secs(config.connect_timeout_secs))
        .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    Database::connect(options).await
}
