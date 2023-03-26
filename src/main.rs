#[macro_use]
extern crate log;

use log::Level;

use std::path::PathBuf;
use twitch_irc::message::ServerMessage;
use twitch_logger::client::Client;
use twitch_logger::config::Config;
use twitch_logger::entities::chat::ChatMessage;

use twitch_logger::error::Error;
use twitch_logger::logger::console_logger::ConsoleLogger;
use twitch_logger::logger::file_logger::FileLogger;
use twitch_logger::logger::value_logger::ValueLogger;
use twitch_logger::utils::chat_message_format::ChatMessageFormat;

async fn setup_logger() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
}

#[tokio::main]
async fn main() {
    setup_logger().await;

    let config = Config::new(None);
    let mut client = Client::new(config);

    let console_logger = ConsoleLogger::new(Level::Info);
    let file_logger = FileLogger::new(PathBuf::from("log.txt"));

    let (tx, mut rx) = tokio::sync::mpsc::channel(1024);

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let ServerMessage::Privmsg(msg) = message {
                let chat_message = ChatMessage::from(msg);

                let simple = ChatMessageFormat::Simple.format(&chat_message);
                console_logger.log(simple.as_str());

                let json = ChatMessageFormat::Json.format(&chat_message);
                file_logger.log(json.as_str());
            }
        }
    });

    client.start(tx).await.unwrap_or_else(handle_error);
}

fn handle_error(e: Error) {
    panic!("Error: {}", e);
}
