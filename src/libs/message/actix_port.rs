use actix::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Message, Serialize, Deserialize, Debug, Clone)]
#[rtype(result = "()")]
pub struct ChatMessage {
    pub from: String,
    pub content: String,
}
