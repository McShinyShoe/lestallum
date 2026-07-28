// SPDX-License-Identifier: GPL-3.0-or-later

use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_create_requests")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub mc_user: String,
    #[sea_orm(unique)]
    pub code: String,
    pub active_until: DateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}
