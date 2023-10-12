use crate::libs::message::Message;

pub trait MessageQueueStore<Config> {
    fn new(config: &Config) -> Result<Self, String> where Self: Sized;
    fn push_message(&self, message: Message) -> Result<bool, String>;
    fn pop_all(&self, line_id: u16, sender: &String) -> Result<Vec<Message>, String>;
    fn get_head(&self, line_id: u16, sender: &String) -> Result<Message, String>;
}