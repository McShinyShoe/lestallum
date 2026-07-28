// SPDX-License-Identifier: GPL-3.0-or-later

use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
