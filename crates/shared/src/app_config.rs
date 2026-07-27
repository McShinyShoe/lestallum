use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
}

impl AppConfig {
    pub fn new() -> anyhow::Result<AppConfig> {
        tracing::info!("Loading config...");
        let cfg = Config::builder()
            .set_default("host", Ipv4Addr::LOCALHOST.to_string())?
            .set_default("port", 3000)?
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::File::with_name("config.local").required(false))
            .add_source(Environment::default().prefix("APP").separator("_"))
            .build()?;
        tracing::info!("Config loaded.");

        Ok(cfg.try_deserialize()?)
    }

    pub fn site_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}
