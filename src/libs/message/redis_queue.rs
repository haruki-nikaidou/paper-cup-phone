use redis::{Commands, Client};
use super::queue_trait::MessageQueueStore;
use super::Message;

pub struct RedisQueue {
    client: Client,
}

impl MessageQueueStore<String> for RedisQueue {
    fn new(redis_url: &str) -> Result<Self, String> {
        let client = Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self { client })
    }

    fn push_message(&self, message: &Message) -> Result<bool, String> {
        let key = format!("line:{}:{}", message.line_id, message.sender);
        let mut con = self.client.get_connection().map_err(|e| e.to_string())?;

        con.lpush(&key, message).map_err(|e| e.to_string())?;
        Ok(true)
    }

    fn pop_all(&self, line_id: u16, sender: String) -> Result<Vec<String>, String> {
        let key = format!("line:{}:{}", line_id, sender);
        let mut con = self.client.get_connection().map_err(|e| e.to_string())?;

        let messages: Vec<String> = con.lrange(&key, 0, -1).map_err(|e| e.to_string())?;
        con.ltrim(&key, -1, 0).map_err(|e| e.to_string())?; // This will keep the list empty after reading all messages

        Ok(messages)
    }

    fn get_head(&self, line_id: u16, sender: String) -> Result<Message, String> {
        let key = format!("line:{}:{}", line_id, sender);
        let mut con = self.client.get_connection().map_err(|e| e.to_string())?;

        let head: Option<String> = con.lindex(&key, 0).map_err(|e| e.to_string())?;
        match head {
            Some(message) => Ok(Message {
                line_id,
                sender,
                content: message,
            }),
            None => Err(format!("No messages in the queue for line: {}, sender: {}", line_id, sender)),
        }
    }
}
