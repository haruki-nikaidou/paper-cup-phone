use actix::{Actor, Handler, StreamHandler};
use actix_web_actors::ws;
use serde::Serialize;
use tracing::{info,error,debug};
use crate::libs::ws::{
    parse_request::parse_request,
    ws_sent_message::ServerMessage,
};

pub(crate) struct WsChatSession {
    pub(crate) name: Option<String>,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<ServerMessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) -> Self::Result {
        match serde_json::to_string(&msg) {
            Ok(msg) => {
                ctx.text(msg);
            }
            Err(e) => {
                error!("Failed to serialize message: {}", e);
                let error_json = serde_json::to_string(&ServerMessage::Error(e.to_string()));
                match error_json {
                    Ok(error_json) => {
                        ctx.text(error_json);
                    }
                    Err(e) => {
                        error!("Failed to serialize error message: {}", e);
                        panic!("Failed to serialize error message: {}", e);
                    }
                }
            }
        }
    }
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
