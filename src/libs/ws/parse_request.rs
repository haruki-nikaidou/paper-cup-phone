use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WsRequest {
    sender: String,
    line_id: u16,
    content: String,
}

pub fn parse_request(request: &str) -> Result<WsRequest, String> {
    match serde_json::from_str(request) {
        Ok(request) => Ok(request),
        Err(e) => Err(e.to_string()),
    }
}