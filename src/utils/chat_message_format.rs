use crate::entities::chat::ChatMessage;

pub trait ChatMessageFormatter {
    fn format(&self, message: ChatMessage) -> String;
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ChatMessageFormat {
    #[default]
    Json,
    Simple,
}

impl ChatMessageFormat {
    pub fn format(&self, message: &ChatMessage) -> String {
        match self {
            ChatMessageFormat::Simple => format_simple(message),
            ChatMessageFormat::Json => format_json(message),
        }
    }
}

fn format_simple(message: &ChatMessage) -> String {
    format!(
        "{} (#{}) {}: {}",
        message.sent_at.format("%Y-%m-%d %H:%M:%S"),
        message.channel,
        message.username,
        message.message
    )
}

fn format_json(message: &ChatMessage) -> String {
    serde_json::to_string(message).unwrap()
}
