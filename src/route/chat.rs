use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use crate::actors::chat_session::WsChatSession;

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsChatSession { name: None, core: (), user_id: () }, &req, stream);
    resp
}
