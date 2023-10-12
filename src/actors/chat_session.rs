use actix::{Actor, Addr, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws;
use tracing::{info, error, debug};
use crate::libs::ws::{
    parse_request::WsRequest,
    ws_sent_message::ServerMessage,
};
use crate::libs::core::{BehaviorAfterReceiveMessage, Core, Sender};

pub(crate) struct WsChatSession {
    pub(crate) name: Option<String>,
    core: Core,
    user_id: Addr<Sender>,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        // Register self in the shared state of connected clients
        self.user_id.do_send(self.user_id.clone());
        // TODO
    }
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
} // impl Handler<ServerMessage> for WsChatSession

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
                let request = match WsRequest::parse_request(&text) {
                    Ok(request) => request,
                    Err(e) => {
                        error!("Failed to parse request: {}", e);
                        return;
                    }
                };
                let message = request.to_message();
                match self.core.receive_message(&message) {
                    Ok(behavior) => {
                        match behavior {
                            BehaviorAfterReceiveMessage::SendToAnotherSender => {
                                let to_sender = &message.sender;
                                let to_sender = match crate::libs::core::string_to_sender(to_sender.to_string()) {
                                    Ok(to_sender) => to_sender,
                                    Err(e) => {
                                        error!("Failed to convert string to sender: {}", e);
                                        return;
                                    }
                                };
                                // TODO
                                match self.core.push_message_to_queue(message) {
                                    Err(_) => {
                                        error!("Failed to push message to queue: {}", e);
                                        ctx.text(ServerMessage::Error("Internal server error.".to_string()));
                                        return;
                                    },
                                    _ => {}
                                };

                            }
                            BehaviorAfterReceiveMessage::PushedToQueue => {
                                // TODO
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to receive message: {}", e);
                        return;
                    }
                }
            }
            _ => {}
        }
    }
}
