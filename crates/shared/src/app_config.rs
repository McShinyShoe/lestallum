// SPDX-License-Identifier: GPL-3.0-or-later

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use config::{Config, Environment};
use db::config::DatabaseConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn new() -> anyhow::Result<AppConfig> {
        tracing::info!("Loading config...");
        let mut builder = Config::builder()
            .set_default("host", Ipv4Addr::LOCALHOST.to_string())?
            .set_default("port", 3000)?
            .set_default("database.max_connections", 10)?
            .set_default("database.min_connections", 1)?
            .set_default("database.connect_timeout_secs", 8)?
            .set_default("database.acquire_timeout_secs", 8)?
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::File::with_name("config.local").required(false))
            .add_source(
                Environment::default()
                    .prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            );

        if let Ok(url) = std::env::var("DATABASE_URL") {
            builder = builder.set_default("database.url", url)?;
        }

        let cfg: AppConfig = builder.build()?.try_deserialize()?;
        tracing::info!("Config loaded.");
        tracing::debug!(config = ?cfg);

        Ok(cfg)
    }

    pub fn site_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}
