pub enum WsResponseCode {
    Success,
    Error,
}

pub struct WsResponse {
    pub code: WsResponseCode,
}