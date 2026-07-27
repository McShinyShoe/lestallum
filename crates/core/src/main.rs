use std::future::Future;
use std::thread;

use db::controllers::DatabaseController;
use shared::app_config::AppConfig;
use shared::app_state::{AppState, SharedState};

fn spawn_service(
    task: impl Future<Output = ()> + Send + 'static,
) -> thread::JoinHandle<anyhow::Result<()>> {
    thread::spawn(move || -> anyhow::Result<()> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(task);
        Ok(())
    })
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::try_init().map_err(|e| anyhow::anyhow!(e))?;

    let config = AppConfig::new()?;

    let database_runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(1)
        .build()?;
    let database = database_runtime.block_on(DatabaseController::connect(&config.database))?;

    let state: SharedState = AppState::new(config, database);

    let services = [
        ("web", spawn_service(lestallum_web::run(state.clone()))),
        (
            "bot-discord",
            spawn_service(bot_discord::run(state.clone())),
        ),
        ("bot-minecraft", spawn_service(bot_minecraft::run(state))),
    ];

    for (name, handle) in services {
        handle
            .join()
            .unwrap_or_else(|_| panic!("{name} thread panicked"))?;
    }
    Ok(())
}
