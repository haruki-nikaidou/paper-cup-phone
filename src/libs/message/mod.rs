pub mod redis_queue;

pub mod queue_trait;
pub mod line_manage;
pub mod actix_port;

use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub line_id: u16,
    pub sender: String,
    pub content: String,
}
