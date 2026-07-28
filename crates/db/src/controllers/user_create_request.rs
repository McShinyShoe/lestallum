// SPDX-License-Identifier: GPL-3.0-or-later

use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

use crate::entities::user_create_request;

#[derive(Debug, Clone)]
pub struct UserCreateRequestController {
    connection: DatabaseConnection,
}

impl UserCreateRequestController {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn create(
        &self,
        mc_user: String,
        code: String,
        active_until: DateTimeWithTimeZone,
    ) -> Result<user_create_request::Model, DbErr> {
        user_create_request::ActiveModel {
            mc_user: Set(mc_user),
            code: Set(code),
            active_until: Set(active_until),
            ..Default::default()
        }
        .insert(&self.connection)
        .await
    }

    pub async fn find_active_by_code(
        &self,
        code: &str,
        now: DateTimeWithTimeZone,
    ) -> Result<Option<user_create_request::Model>, DbErr> {
        user_create_request::Entity::find()
            .filter(user_create_request::Column::Code.eq(code))
            .filter(user_create_request::Column::ActiveUntil.gt(now))
            .one(&self.connection)
            .await
    }

    pub async fn find_by_mc_user(
        &self,
        mc_user: &str,
    ) -> Result<Vec<user_create_request::Model>, DbErr> {
        user_create_request::Entity::find()
            .filter(user_create_request::Column::McUser.eq(mc_user))
            .all(&self.connection)
            .await
    }

    pub async fn delete_by_id(&self, id: i32) -> Result<bool, DbErr> {
        let result = user_create_request::Entity::delete_by_id(id)
            .exec(&self.connection)
            .await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn delete_expired(&self, now: DateTimeWithTimeZone) -> Result<u64, DbErr> {
        let result = user_create_request::Entity::delete_many()
            .filter(user_create_request::Column::ActiveUntil.lte(now))
            .exec(&self.connection)
            .await?;
        Ok(result.rows_affected)
    }
}
