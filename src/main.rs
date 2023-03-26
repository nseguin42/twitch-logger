#[macro_use]
extern crate log;

use twitch_irc::message::ServerMessage;
use twitch_logger::client::Client;
use twitch_logger::config::Config;
use twitch_logger::entities::chat::ChatMessage;

use twitch_logger::error::Error;
use twitch_logger::logger::console_logger::ConsoleLogger;
use twitch_logger::logger::db_logger::DbLogger;
use twitch_logger::logger::file_logger::FileLogger;
use twitch_logger::logger::value_logger::ValueLogger;
use twitch_logger::utils::chat_message_format::ChatMessageFormat;

fn setup_logger() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
}

#[tokio::main]
async fn main() {
    setup_logger();

    let config = Config::new(None);

    let mut db_logger = DbLogger::try_from(&config).await.unwrap();
    let mut console_logger = ConsoleLogger::try_from(&config).unwrap();
    let mut file_logger = FileLogger::try_from(&config).unwrap();

    let (tx, mut rx) = tokio::sync::mpsc::channel(1024);
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let ServerMessage::Privmsg(msg) = message {
                let chat_message = ChatMessage::from(msg);
                db_logger.create_log(&chat_message).await.unwrap();

                let simple = ChatMessageFormat::Simple.format(&chat_message);
                console_logger.log(simple.as_str());

                let json = ChatMessageFormat::Json.format(&chat_message);
                file_logger.log(json.as_str());
            }
        }
    });

    let mut client = Client::try_from(&config).unwrap();
    client.start(tx).await.unwrap_or_else(handle_error);
}

fn handle_error(e: Error) {
    panic!("Error: {}", e);
}
