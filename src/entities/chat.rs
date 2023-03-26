use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub channel: String,
    pub sender: String,
    pub message: String,
    pub sent_at: DateTime<Utc>,
}

impl Display for ChatMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[#{}] {}: {}", self.channel, self.sender, self.message)
    }
}

impl ChatMessage {
    pub fn new(channel: String, sender: String, message: String) -> Self {
        Self {
            channel,
            sender,
            message,
            sent_at: Utc::now(),
        }
    }
}

impl From<twitch_irc::message::PrivmsgMessage> for ChatMessage {
    fn from(message: twitch_irc::message::PrivmsgMessage) -> Self {
        Self::new(
            message.channel_login,
            message.sender.login,
            message.message_text,
        )
    }
}
