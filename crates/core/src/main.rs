use std::future::Future;
use std::thread;

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

    let state: SharedState = AppState::new(AppConfig::new()?);

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
