use crate::config::Config;
use crate::entities::chat::ChatMessage;
use crate::logger::db_logger::DbLogger;
use std::ops::Add;

use log::{debug, trace};

use chrono::{DateTime, Duration, Utc};

use tokio::sync::mpsc::Receiver;

use tokio::time::{interval, sleep, sleep_until};
use tokio::{join, pin, select};

pub struct MessageHandler {
    rx: Receiver<ChatMessage>,
    config: Config,
    db_logger: DbLogger,
}

impl MessageHandler {
    pub fn new(config: &Config, rx: Receiver<ChatMessage>, db_logger: DbLogger) -> Self {
        Self {
            rx,
            config: config.clone(),
            db_logger,
        }
    }

    async fn recv_with_timeout(&mut self) -> Option<ChatMessage> {
        let interval = Duration::seconds(1).to_std().unwrap();
        let sleep = sleep(interval);

        pin!(sleep, interval);

        select! {
            message = self.rx.recv() => {
               message
            },
            _ = &mut sleep => {
                 None
            }
        }
    }

    pub async fn run(&mut self) {
        let mut buffer = vec![];
        let interval = Duration::seconds(5);
        let mut next_flush_at = Utc::now().add(interval);

        loop {
            let message = self.recv_with_timeout().await;

            if let Some(message) = message {
                debug!("{:?}", message);
                buffer.push(message);
            }

            let now = Utc::now();

            if now >= next_flush_at {
                next_flush_at = now.add(interval);
                self.db_logger.create_log_batch(&buffer).await;
                buffer = vec![];
            }
        }
    }
}
