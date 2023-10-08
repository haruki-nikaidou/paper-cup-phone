use std::collections::HashSet;
use tracing::{info,error};
use crate::libs::message::redis_queue::RedisQueue;

use super::message::Message;
use super::message::queue_trait::MessageQueueStore;
use super::load_config::{Queue, LoadResult};
use super::message::line_manage::{LineManager, AddSenderActuallyDone};
use super::parse_config::Profile;

type Sender = [u8;64];

const INTERNAL_SERVER_ERROR: &str = "Internal server error.";
const TRY_TO_JOIN_BUSY_LINE: &str = "Try to join busy line.";

pub fn sender_to_string(sender: Sender) -> Result<String,String> {
    match std::str::from_utf8(&self.data) {
        Ok(v) => Ok(v.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn string_to_sender(sender: String) -> Result<Sender,String> {
    match sender.as_bytes().len() {
        64 => Ok(sender.as_bytes()),
        _ => Err("Sender must be 64 bytes".to_string()),
    }
}

pub struct Core {
    online: HashSet<Sender>,
    queue: Queue,
    line_manager: LineManager,
    profile: Profile,
}

pub enum JoinLineResult {
    Refresh,
    BeTheFirst,
    BeTheSecond(Vec<Message>),
    Rejoin(Vec<Message>)
}

impl Core {
    pub fn new(config: LoadResult) -> Self {
        Core {
            online: HashSet::new(),
            queue: config.queue,
            line_manager: config.line_manager,
            profile: config.profile,
        }
    }
    pub fn join_line(&mut self, sender: Sender, line_id: u16) -> Result<JoinLineResult, &str> {        
        // log
        info!("{} join line {}", sender_to_string(sender), line_id);
        
        // When the sender is already online, return true.
        if self.is_online(sender) {
            return Ok(JoinLineResult::Refresh);
        }


        self.online.insert(sender);
        match self.line_manager.add_sender(sender, line_id) {
            Ok(AddSenderActuallyDone::AddTheFirstSender) => Ok(true),
            Ok(AddSenderActuallyDone::AddTheSecondSender) => {
                // get the messages from the queue
                let another_sender = self.line_manager.get_senders(line_id)?[0];
                let messages = match self.queue {
                    Queue::Redis(q) => {
                        q.pop_all(line_id, another_sender)
                    }
                };
                match messages {
                    Ok(messages) => Ok(JoinLineResult::BeTheSecond(messages)),
                    Err(e) => {
                        error!("Failed to get messages from queue: {}", e);
                        Err(INTERNAL_SERVER_ERROR)
                    },
                }
            },
            Ok(AddSenderActuallyDone::AlreadyInLine) => {
                // get the message from another sender
                let senders = self.line_manager.get_senders(line_id)?;
                let another_sender = if senders[0] == sender_to_string(sender)? {
                    senders[1].clone()
                } else {
                    senders[0].clone()
                };
                let messages = match self.queue {
                    Queue::Redis(q) => {
                        q.pop_all(line_id, another_sender)
                    }
                };
                match messages {
                    Ok(messages) => Ok(JoinLineResult::Rejoin(messages)),
                    Err(e) => {
                        error!("Failed to get messages from queue: {}", e);
                        Err(INTERNAL_SERVER_ERROR)
                    },
                }
            },
            Ok(AddSenderActuallyDone::TryToAddTheThirdSender) => {
                // return error
                Err(TRY_TO_JOIN_BUSY_LINE)
            },
            Err(e) => {
                error!("Failed to add sender to line: {}", e);
                Err(INTERNAL_SERVER_ERROR)
            },
        }
    }
    pub fn set_offline(&mut self, sender: Sender) {
        info!("{} offline", sender_to_string(sender));
        self.online.remove(&sender);
    }
    pub fn is_online(&self, sender: Sender) -> bool {
        self.online.contains(&sender)
    }
} // impl Core