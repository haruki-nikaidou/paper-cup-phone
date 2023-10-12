use serde_derive::{Deserialize, Serialize};
use crate::libs::message::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct WsRequest {
    sender: String,
    line_id: u16,
    content: String,
}


impl WsRequest {
    pub fn parse_request(request: &str) -> Result<Self, String> {
        match serde_json::from_str(request) {
            Ok(request) => Ok(request),
            Err(e) => Err(e.to_string()),
        }
    }
    pub fn to_message(self) -> Message {
        Message {
            sender: self.sender,
            line_id: self.line_id,
            content: self.content,
        }
    }
}