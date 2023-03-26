use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub channel: String,
    pub username: String,
    pub message: String,
    pub sent_at: DateTime<Utc>,
}

impl Display for ChatMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[#{}] {}: {}", self.channel, self.username, self.message)
    }
}

impl ChatMessage {
    pub fn new(channel: String, username: String, message: String, sent_at: DateTime<Utc>) -> Self {
        Self {
            channel,
            username,
            message,
            sent_at,
        }
    }
}

impl From<twitch_irc::message::PrivmsgMessage> for ChatMessage {
    fn from(message: twitch_irc::message::PrivmsgMessage) -> Self {
        Self::new(
            message.channel_login,
            message.sender.login,
            message.message_text,
            message.server_timestamp,
        )
    }
}
