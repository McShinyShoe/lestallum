// SPDX-License-Identifier: GPL-3.0-or-later

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("user_create_requests")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("mc_user"))
                    .col(string_len_uniq("code", 6))
                    .col(timestamp_with_time_zone("active_until"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_user_create_requests_mc_user")
                    .table("user_create_requests")
                    .col("mc_user")
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("user_create_requests").to_owned())
            .await
    }
}
