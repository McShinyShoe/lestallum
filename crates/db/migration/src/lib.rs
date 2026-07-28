// SPDX-License-Identifier: GPL-3.0-or-later

pub use sea_orm_migration::prelude::*;

mod m20260727_000001_create_users_table;
mod m20260727_000002_create_user_create_requests_table;
mod m20260728_000001_add_password_hash_to_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260727_000001_create_users_table::Migration),
            Box::new(m20260727_000002_create_user_create_requests_table::Migration),
            Box::new(m20260728_000001_add_password_hash_to_users::Migration),
        ]
    }
}
