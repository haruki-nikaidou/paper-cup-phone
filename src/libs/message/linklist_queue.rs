use std::collections::LinkedList;
use crate::libs::message::Message;
use super::queue_trait::MessageQueueStore;

struct SenderQueue {
    sender: String,
    queue: LinkedList<Message>,
}
impl SenderQueue {
    fn new() -> Self {
        SenderQueue {
            sender: "".to_string(),
            queue: LinkedList::new(),
        }
    }
    fn has_sender(&self) -> bool {
        self.sender.len() > 0
    }
    fn set_sender(&mut self, sender: String) {
        self.sender = sender;
    }
    fn get_sender(&self) -> String {
        self.sender.clone()
    }
}

pub struct LinkListQueue {
    sender_a_queue: SenderQueue,
    sender_b_queue: SenderQueue,
}

enum WhoseSenderIs {
    A,
    B,
    None,
}

impl LinkListQueue {
    fn whose_sender_is(&self, sender: String) -> WhoseSenderIs {
        if self.sender_a_queue.get_sender() == sender {
            WhoseSenderIs::A
        } else if self.sender_b_queue.get_sender() == sender {
            WhoseSenderIs::B
        } else {
            WhoseSenderIs::None
        }
    }
}

impl MessageQueueStore<()> for LinkListQueue {
    fn new(config: &()) -> Result<Self, String> where Self: Sized {
        Ok(LinkListQueue {
            sender_a_queue: SenderQueue::new(),
            sender_b_queue: SenderQueue::new(),
        })
    }

    fn push_message(&mut self, message: Message) -> Result<bool, String> {
        let push_to = if self.sender_a_queue.get_sender() == message.sender {
            "a"
        } else if self.sender_b_queue.get_sender() == message.sender {
            "b"
        } else if !self.sender_a_queue.has_sender() {
            "a"
        } else if !self.sender_b_queue.has_sender() {
            "b"
        } else {
            return Err("Unexpected Sender".to_string());
        };
        match push_to {
            "a" => {
                self.sender_a_queue.set_sender(message.sender.clone());
                self.sender_a_queue.queue.push_back(message);
            },
            "b" => {
                self.sender_b_queue.set_sender(message.sender.clone());
                self.sender_b_queue.queue.push_back(message);
            },
            _ => return Err("Unexpected Sender".to_string()),
        }
        Ok(true)
    }

    fn pop_all(&mut self, line_id: u16, sender: String) -> Result<Vec<Message>, String> {
        let pop_from = self.whose_sender_is(sender);
        match pop_from {
            WhoseSenderIs::A => {
                let mut result = Vec::new();
                while let Some(message) = self.sender_a_queue.queue.pop_front() {
                    result.push(message);
                }
                Ok(result)
            },
            WhoseSenderIs::B => {
                let mut result = Vec::new();
                while let Some(message) = self.sender_b_queue.queue.pop_front() {
                    result.push(message);
                }
                Ok(result)
            },
            WhoseSenderIs::None => Err("Unexpected Sender".to_string()),
        }
    }

    fn get_head(&self, line_id: u16, sender: String) -> Result<Message, String> {
        let get_from = self.whose_sender_is(sender);
        match get_from {
            WhoseSenderIs::A => {
                if let Some(&message) = self.sender_a_queue.queue.front() {
                    Ok(message.clone())
                } else {
                    Err("Unexpected Sender".to_string())
                }
            },
            WhoseSenderIs::B => {
                if let Some(&message) = self.sender_b_queue.queue.front() {
                    Ok(message.clone())
                } else {
                    Err("Unexpected Sender".to_string())
                }
            },
            WhoseSenderIs::None => Err("Unexpected Sender".to_string()),
        }
    }
}