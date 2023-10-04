use std::sync::{Arc, Mutex};
use redis::{Commands, Client, RedisResult};
use crate::libs::redis_connect::RedisConnection;

pub struct LineManager {
    client: Arc<Mutex<Client>>,
    auto_delete_time: Option<u64>
}

impl LineManager {
    fn new(config: RedisConnection) -> Result<Self, String> {
        Ok(Self {
            client: config.get_client(),
            auto_delete_time: config.auto_delete_time
        })
    }
    fn add_sender(&self, sender: String, line_id: u16) -> Result<bool, String> {
        let key = format!("sender:{}:line", line_id);
        let mut con = match self.client.lock().unwrap().get_connection() {
            Ok(con) => con,
            Err(e) => return Err(e.to_string()),
        };

        // Check if there's a record associated with the key.
        let existing_value: Option<String> = con.get(&key).map_err(|e| e.to_string())?;

        match existing_value {
            None => {
                // Add the new record.
                match con.set::<&String, String, ()>(&key, sender) {
                    Ok(_) => Ok(true),
                    Err(e) => Err(e.to_string()),
                }.expect("Failed to add sender to line.");
                Ok(true)
            }
            Some(value) => {
                // Check if sender is in the value.
                let senders: Vec<&str> = value.split(':').collect();
                if senders.contains(&sender.as_str()) {
                    Ok(true)
                } else if senders.len() == 1 {
                    let new_value = format!("{}:{}", value, sender);
                    match con.set::<&String, &String, ()>(&key, &new_value) {
                        Ok(_) => Ok(true),
                        Err(e) => Err(e.to_string()),
                    }.expect("Failed to add sender to line.");
                    Ok(true)
                } else {
                    Ok(true)
                }
            }
        }
    }
}