use std::sync::{Arc, Mutex};
use redis::{Commands, Client};
use crate::libs::redis_connect::RedisConnection;
use super::queue_trait::MessageQueueStore;
use crate::libs::message::Message;

pub struct RedisQueue {
    client: Arc<Mutex<Client>>,
    auto_delete_time: Option<u64>
}

impl MessageQueueStore<RedisConnection> for RedisQueue {

    fn new(config: &RedisConnection) -> Result<Self, String> {
        Ok(Self {
            client: config.get_client(),
            auto_delete_time: config.auto_delete_time
        })
    }

    fn push_message(&self, message: Message) -> Result<bool, String> {
        let key = format!("line:{}:{}", message.line_id, message.sender);
        let value = message.content;
        let mut con = self.client.lock().unwrap().get_connection().map_err(|e| e.to_string())?;

        con.lpush(&key, value).map_err(|e| e.to_string())?;
        con.expire(&key, self.auto_delete_time.unwrap_or(0) as usize).map_err(|e| e.to_string())?; // If auto_delete_time is None, then the key will never expire
        Ok(true)
    }

    fn pop_all(&self, line_id: u16, sender: &String) -> Result<Vec<Message>, String> {
        let key = format!("line:{}:{}", line_id, sender);
        let mut con = self.client.lock().unwrap().get_connection().map_err(|e| e.to_string())?;

        let message_strings: Vec<String> = con.lrange(&key, 0, -1).map_err(|e| e.to_string())?;
        con.ltrim(&key, -1, 0).map_err(|e| e.to_string())?; // This will keep the list empty after reading all messages

        let messages: Vec<Message> = message_strings.iter()
            .map(|message_string| Message {
                line_id,
                sender: sender.clone(),
                content: message_string.clone(),
            })
            .collect();
        Ok(messages)
    }

    fn get_head(&self, line_id: u16, sender: &String) -> Result<Message, String> {
        let key = format!("line:{}:{}", line_id, sender);
        let mut con = self.client.lock().unwrap().get_connection().map_err(|e| e.to_string())?;

        let head: Option<String> = con.lindex(&key, 0).map_err(|e| e.to_string())?;
        match head {
            Some(message) => Ok(Message {
                line_id,
                sender: sender.clone(),
                content: message,
            }),
            None => Err(format!("No messages in the queue for line: {}, sender: {}", line_id, sender)),
        }
    }
}
