// SPDX-License-Identifier: GPL-3.0-or-later

use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}

pub fn router() -> Router {
    Router::new().route("/health", get(health))
}
