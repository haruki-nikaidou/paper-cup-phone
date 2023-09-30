use std::collections::{LinkedList, HashMap};
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
    hash_map: HashMap<String, SenderQueue>,
}

enum WhoseSenderIs {
    A,
    B,
    None,
}

impl MessageQueueStore<()> for LinkListQueue {
    fn new(config: &()) -> Result<Self, String> where Self: Sized {
        Ok(LinkListQueue {
            hash_map: HashMap::new(),
        })
    }

    fn push_message(&mut self, message: Message) -> Result<bool, String> {
        let key = format!("line:{}:{}", message.line_id, message.sender);
        let mut queue = self.hash_map.entry(key).or_insert(SenderQueue::new());
        queue.queue.push_back(message);
        Ok(true)
    }

    fn pop_all(&mut self, line_id: u16, sender: String) -> Result<Vec<Message>, String> {
        let key = format!("line:{}:{}", line_id, sender);
        let mut queue = self.hash_map.entry(key).or_insert(SenderQueue::new());
        let mut messages: Vec<Message> = Vec::new();
        while let Some(message) = queue.queue.pop_front() {
            messages.push(message);
        }
        Ok(messages)
    }

    fn get_head(&self, line_id: u16, sender: String) -> Result<Message, String> {
        let key = format!("line:{}:{}", line_id, sender);
        let queue = self.hash_map.get(&key);
        match queue {
            Some(queue) => {
                let sender = queue.get_sender();
                let message = queue.queue.front();
                match message {
                    Some(message) => Ok(Message {
                        line_id,
                        sender,
                        content: message.content.clone(),
                    }),
                    None => Err(format!("No messages in the queue for line: {}, sender: {}", line_id, sender)),
                }
            },
            None => Err(format!("No messages in the queue for line: {}, sender: {}", line_id, sender)),
        }
    }
}