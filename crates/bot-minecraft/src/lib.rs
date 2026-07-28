// SPDX-License-Identifier: GPL-3.0-or-later

use shared::app_state::SharedState;

pub async fn run(_state: SharedState) {
    tracing::info!("Starting minecraft bot thread...");
    // TODO: connect to the Minecraft server and start handling events
}
