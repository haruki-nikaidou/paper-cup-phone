use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use tracing::{info,error,debug};
use crate::libs::ws::parse_request::parse_request;

pub(crate) struct WsChatSession {
    pub(crate) name: Option<String>,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(ping)) => {
                ctx.pong(&ping);
            }
            Ok(ws::Message::Pong(_)) => {
                debug!("Pong received");
            }
            Ok(ws::Message::Text(text)) => {
                let request = match parse_request(&text) {
                    Ok(request) => request,
                    Err(e) => {
                        error!("Failed to parse request: {}", e);
                        return;
                    }
                };
            }
            _ => {}
        }
    }
}
