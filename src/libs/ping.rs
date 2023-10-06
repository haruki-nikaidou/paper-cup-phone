use actix::Message;
use serde_derive::{Deserialize, Serialize};

#[derive(Message, Serialize, Deserialize, Debug, Clone)]
#[rtype(result = "()")]
pub struct Ping;