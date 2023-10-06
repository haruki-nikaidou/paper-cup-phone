mod libs;
mod route;

use actix_web::{App, HttpServer, web};
use route::chat;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ws/", web::get().to(chat::chat_route))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
