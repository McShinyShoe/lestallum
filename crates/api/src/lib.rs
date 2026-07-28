// SPDX-License-Identifier: GPL-3.0-or-later

mod controllers;

use axum::Router;

pub fn router() -> Router {
    Router::new().merge(controllers::health::router())
}
