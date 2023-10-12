use actix::Message;
use serde_derive::{Deserialize, Serialize};
use crate::libs::message::Message as ChatMessage;

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    PushChatMessages(Vec<ChatMessage>),
    Error(String)
}
