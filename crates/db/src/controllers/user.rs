use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};

use crate::entities::user;

#[derive(Debug, Clone)]
pub struct UserController {
    connection: DatabaseConnection,
}

impl UserController {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn create(&self, id: String, mc_name: String) -> Result<user::Model, DbErr> {
        user::ActiveModel {
            id: Set(id),
            mc_name: Set(mc_name),
        }
        .insert(&self.connection)
        .await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find_by_id(id).one(&self.connection).await
    }

    pub async fn find_by_mc_name(&self, mc_name: &str) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find_by_mc_name(mc_name)
            .one(&self.connection)
            .await
    }

    pub async fn list(&self) -> Result<Vec<user::Model>, DbErr> {
        user::Entity::find().all(&self.connection).await
    }

    pub async fn delete_by_id(&self, id: &str) -> Result<bool, DbErr> {
        let result = user::Entity::delete_by_id(id)
            .exec(&self.connection)
            .await?;
        Ok(result.rows_affected > 0)
    }
}
