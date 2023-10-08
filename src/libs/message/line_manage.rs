use std::sync::{Arc, Mutex};
use redis::{Commands, Client, RedisResult};
use crate::libs::redis_connect::RedisConnection;

pub struct LineManager {
    client: Arc<Mutex<Client>>,
    auto_delete_time: Option<u64>
}

fn set_ttl_for_key(con: &mut redis::Connection, key: &String, time: u64) -> Result<bool, String> {
    match con.expire::<&String, usize>(&key, time as usize) {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}

pub enum AddSenderActuallyDone {
    AddTheFirstSender,
    AddTheSecondSender,
    TryToAddTheThirdSender,     // failed
    AlreadyInLine,
}

impl LineManager {
    pub fn new(config: RedisConnection) -> Result<Self, String> {
        Ok(Self {
            client: config.get_client(),
            auto_delete_time: config.auto_delete_time
        })
    }
    pub fn add_sender(&self, sender: String, line_id: u16) -> Result<AddSenderActuallyDone, String> {
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
                match self.auto_delete_time {
                    Some(time) => {
                        set_ttl_for_key(&mut con, &key, time)
                    },
                    None => Ok(true),
                }.expect("Failed to set auto delete time.");
                Ok(AddSenderActuallyDone::AddTheFirstSender)
            }
            Some(value) => {
                // Check if sender is in the value.
                let senders: Vec<&str> = value.split(':').collect();
                if senders.contains(&sender.as_str()) {
                    Ok(AddSenderActuallyDone::AlreadyInLine)
                } else if senders.len() == 1 {
                    let new_value = format!("{}:{}", value, sender);
                    match con.set::<&String, &String, ()>(&key, &new_value) {
                        Ok(_) => Ok(true),
                        Err(e) => Err(e.to_string()),
                    }.expect("Failed to add sender to line.");
                    Ok(AddSenderActuallyDone::AddTheSecondSender)
                } else {
                    Ok(AddSenderActuallyDone::TryToAddTheThirdSender)
                }
            }
        }
    }

    pub fn refresh_ttl(&self, line_id: u16) -> Result<bool,String> {
        let key = format!("sender:{}:line", line_id);
        let mut con = match self.client.lock().unwrap().get_connection() {
            Ok(con) => con,
            Err(e) => return Err(e.to_string()),
        };

        match self.auto_delete_time {
            Some(time) => {
                set_ttl_for_key(&mut con, &key, time)
            },
            None => Ok(true),
        }.expect("Failed to set auto delete time.");
        Ok(true)
    }
}