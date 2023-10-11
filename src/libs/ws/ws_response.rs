use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum WsResponseCode {
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsResponse {
    pub code: WsResponseCode,
    pub error_message: Option<String>,
}

pub fn get_send_response(result: Result<(),String>) -> String {
    serde_json::to_string(&WsResponse {
        code: match result {
            Ok(_) => WsResponseCode::Success,
            Err(_) => WsResponseCode::Error,
        },
        error_message: match result {
            Ok(_) => None,
            Err(e) => Some(e),
        },
    }).unwrap()
}