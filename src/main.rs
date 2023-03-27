#[macro_use]
extern crate log;

use sqlx::PgPool;
use tokio::spawn;
use tokio::task::JoinError;
use twitch_logger::client::Client;
use twitch_logger::config::Config;

use twitch_logger::error::Error;
use twitch_logger::handler::MessageHandler;
use twitch_logger::logger::db_logger::DbLogger;

fn setup_logger() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
}

async fn setup_db_pool(config: &Config) -> Result<PgPool, Error> {
    let pool = PgPool::connect(&config.db_url.clone().unwrap()).await?;
    Ok(pool)
}

#[tokio::main]
async fn main() {
    setup_logger();
    let config = Config::new(None);
    let pool = setup_db_pool(&config).await.unwrap();
    let db_logger = DbLogger::new(&config, pool);

    let (tx, rx) = tokio::sync::mpsc::channel(1024);
    let mut handler = MessageHandler::new(&config, rx, db_logger);

    let client_handle = spawn(async move {
        let mut client = Client::try_from(&config).unwrap();
        client.start(tx).await.unwrap();
    });

    let handler_handle = spawn(async move {
        handler.run().await;
    });

    let result = tokio::select!(
        client_result = client_handle => client_result,
        handler_result = handler_handle => handler_result,
    );

    match result {
        Ok(_) => info!("Exited."),
        Err(err) => handle_join_error(err),
    }
}

fn handle_join_error(err: JoinError) {
    panic!("Join error: {}", err);
}
