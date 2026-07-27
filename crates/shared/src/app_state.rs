use std::sync::Arc;

use crate::app_config::AppConfig;

pub type SharedState = Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub config: AppConfig,
}

impl AppState {
    pub fn new(config: AppConfig) -> SharedState {
        Arc::new(Self { config })
    }
}
