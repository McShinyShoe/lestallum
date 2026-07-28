// SPDX-License-Identifier: GPL-3.0-or-later

pub mod user;
pub mod user_create_request;

use sea_orm::{DatabaseConnection, DbErr};

use crate::config::DatabaseConfig;
use crate::controllers::user::UserController;
use crate::controllers::user_create_request::UserCreateRequestController;

#[derive(Debug, Clone)]
pub struct DatabaseController {
    pub connection: DatabaseConnection,
    pub user: UserController,
    pub user_create_request: UserCreateRequestController,
}

impl DatabaseController {
    pub async fn connect(config: &DatabaseConfig) -> Result<Self, DbErr> {
        Ok(Self::new(crate::connect(config).await?))
    }

    pub fn new(connection: DatabaseConnection) -> Self {
        Self {
            user: UserController::new(connection.clone()),
            user_create_request: UserCreateRequestController::new(connection.clone()),
            connection,
        }
    }
}
