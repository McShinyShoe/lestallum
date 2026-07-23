use std::future::Future;
use std::thread;

fn spawn_service(
    task: impl Future<Output = ()> + Send + 'static,
) -> thread::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
    thread::spawn(move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(task);
        Ok(())
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let services = [
        ("web", spawn_service(lestallum_web::run())),
        ("bot-discord", spawn_service(bot_discord::run())),
        ("bot-minecraft", spawn_service(bot_minecraft::run())),
    ];

    for (name, handle) in services {
        handle
            .join()
            .unwrap_or_else(|_| panic!("{name} thread panicked"))
            .map_err(|e| e as Box<dyn std::error::Error>)?;
    }
    Ok(())
}
