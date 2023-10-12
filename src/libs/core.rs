use std::collections::HashSet;
use tracing::{info, error, debug};

use crate::libs::message::Message;
use super::message::queue_trait::MessageQueueStore;
use super::load_config::{Queue, LoadResult};
use super::message::line_manage::{LineManager, AddSenderActuallyDone};
use super::parse_config::Profile;

pub type Sender = [u8; 64];

const INTERNAL_SERVER_ERROR: &str = "Internal server error.";
const TRY_TO_JOIN_BUSY_LINE: &str = "Try to join busy line.";
const SENDING_TO_LINE_THAT_YOU_ARE_NOT_IN: &str = "Sending to the line that you are not in";
const ILLEGAL_INPUT: &str = "Illegal input.";

pub enum BehaviorAfterReceiveMessage {
    SendToAnotherSender,
    PushedToQueue,
}

pub fn sender_to_string(sender: Sender) -> Result<String, String> {
    match std::str::from_utf8(&sender) {
        Ok(v) => Ok(v.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn string_to_sender(sender: String) -> Result<Sender, String> {
    match sender.as_bytes().len() {
        64 => Ok(Sender::try_from(sender.as_bytes()).unwrap()),
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
    Rejoin(Vec<Message>),
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
    // fn new
    pub fn join_line(&mut self, sender: Sender, line_id: u16) -> Result<JoinLineResult, String> {
        // log
        info!("{} join line {}", sender_to_string(sender).unwrap(), line_id);

        // When the sender is already online, return true.
        if self.is_online(sender) {
            info!("{} is already online, but tried to join again", sender_to_string(sender).unwrap());
            return Ok(JoinLineResult::Refresh);
        }

        self.online.insert(sender);
        let sender = sender_to_string(sender)?;
        match self.line_manager.add_sender(sender.clone(), line_id) {

            // When Sender is the first sender. Just add he to senders list.
            Ok(AddSenderActuallyDone::AddTheFirstSender) => Ok(JoinLineResult::BeTheFirst),

            // When Sender is the second sender. Get the messages from the queue.
            Ok(AddSenderActuallyDone::AddTheSecondSender) => {
                // get the messages from the queue
                let another_sender = &self.line_manager.get_senders(line_id)?[0];
                let messages = match &self.queue {
                    Queue::Redis(q) => {
                        q.pop_all(line_id, another_sender)
                    }
                };
                debug!("{} get messages from queue", sender.clone());
                match messages {
                    Ok(messages) => Ok(JoinLineResult::BeTheSecond(messages)),
                    Err(e) => {
                        error!("Failed to get messages from queue: {}", e);
                        Err(INTERNAL_SERVER_ERROR.to_string())
                    }
                }
            }

            // When Sender is already in the senders list. Get the messages from the queue.
            Ok(AddSenderActuallyDone::AlreadyInLine) => {
                // get the message from another sender
                let senders = self.line_manager.get_senders(line_id)?;
                let another_sender = if senders[0] == sender {
                    senders[1].clone()
                } else {
                    senders[0].clone()
                };
                let messages = match &self.queue {
                    Queue::Redis(q) => {
                        q.pop_all(line_id, &another_sender)
                    }
                };
                debug!("{} get messages from queue", sender);
                match messages {
                    Ok(messages) => Ok(JoinLineResult::Rejoin(messages)),
                    Err(e) => {
                        error!("Failed to get messages from queue: {}", e);
                        Err(INTERNAL_SERVER_ERROR.to_string())
                    }
                }
            }

            // When Sender is the third sender. Return error.
            Ok(AddSenderActuallyDone::TryToAddTheThirdSender) => {
                // return error
                info!("{} try to join busy line {}", sender, line_id);
                Err(TRY_TO_JOIN_BUSY_LINE.to_string())
            }

            // Internal server error.
            Err(e) => {
                error!("Failed to add sender to line: {}", e);
                Err(INTERNAL_SERVER_ERROR.to_string())
            }
        }
    }
    // fn join_line
    pub fn set_offline(&mut self, sender: Sender) {
        info!("{} offline", sender_to_string(sender).unwrap());
        self.online.remove(&sender);
    }
    pub fn is_online(&self, sender: Sender) -> bool {
        self.online.contains(&sender)
    }

    pub fn exit_line(&mut self, sender: Sender, line_id: u16) -> Result<(), String> {
        info!("{} exit line {}", sender_to_string(sender).unwrap(), line_id);
        self.set_offline(sender);
        let sender = sender_to_string(sender)?;
        self.line_manager.remove_sender(sender, line_id)
    }

    pub fn receive_message(&mut self, message: &Message) -> Result<BehaviorAfterReceiveMessage, String> {
        /// If the sender who is in the same line with the `message.sender`
        /// is online, send the message to him.
        /// else, push the message to the queue.

        let Message { sender, line_id, content } = message.clone();

        let sender_line_in = line_id;
        let another_sender = match self.line_manager.get_senders(*line_id) {
            Ok(senders) => {
                match senders.iter().find(|s| **s == sender) {
                    None => {
                        return Err(SENDING_TO_LINE_THAT_YOU_ARE_NOT_IN.to_string());
                    }
                    _ => {}
                }

                // only 1 sender, push to the queue
                if senders.len() == 1 {
                    self.queue.push(message)?;
                    return Ok(BehaviorAfterReceiveMessage::PushedToQueue);
                }
                if senders[0] == sender {
                    senders[1].clone()
                } else {
                    senders[0].clone()
                }
            }
            Err(e) => {
                error!("Failed to get senders: {}", e);
                return Err(INTERNAL_SERVER_ERROR.to_string());
            }
        };

        // if the another sender is online, send the message to him.
        return if self.is_online(match string_to_sender(another_sender) {
            Ok(sender) => sender,
            Err(e) => {
                debug!("Failed to convert string to sender: {}", e);
                return Err(ILLEGAL_INPUT.to_string());
            }
        }) {
            Ok(BehaviorAfterReceiveMessage::SendToAnotherSender)
        } else {
            self.queue.push(message)?;
            Ok(BehaviorAfterReceiveMessage::PushedToQueue)
        }
    }

    pub fn push_message_to_queue(&mut self, message: Message) -> Result<(),String> {
        self.queue.push(message)?
    }
} // impl Core