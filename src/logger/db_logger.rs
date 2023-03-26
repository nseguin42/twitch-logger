use crate::entities::chat::ChatMessage;

pub trait DbClient {
    fn create_log(&mut self, msg: ChatMessage);
}

pub struct DbLogger<'a> {
    pub client: &'a dyn DbClient,
}
