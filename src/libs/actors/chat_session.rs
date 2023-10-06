use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

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
                println!("Received pong from {:?}", self.name);
            }
            Ok(ws::Message::Text(text)) => {
                if let Some(ref name) = self.name {
                    println!("{}: {}", name, text);
                }
            }
            _ => {}
        }
    }
}
