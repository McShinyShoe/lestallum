// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::Arc;

use db::controllers::DatabaseController;

use crate::app_config::AppConfig;

pub type SharedState = Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub config: AppConfig,
    pub database: DatabaseController,
}

impl AppState {
    pub fn new(config: AppConfig, database: DatabaseController) -> SharedState {
        Arc::new(Self { config, database })
    }
}
