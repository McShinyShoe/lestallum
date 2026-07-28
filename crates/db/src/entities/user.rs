// SPDX-License-Identifier: GPL-3.0-or-later

use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub mc_name: String,
    pub password_hash: String,
}

impl ActiveModelBehavior for ActiveModel {}
