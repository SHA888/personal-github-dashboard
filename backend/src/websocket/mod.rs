use actix::{Actor, ActorContext, StreamHandler};
use actix_web::web;
use actix_web_actors::ws;
use actix_web_actors::ws::Message;

pub struct WebSocket;

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Ping(msg)) => ctx.pong(&msg),
            Ok(Message::Text(text)) => ctx.text(text),
            Ok(Message::Binary(bin)) => ctx.binary(bin),
            Ok(Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl Default for WebSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocket {
    pub fn new() -> Self {
        Self
    }
}

pub async fn ws_index(
    req: actix_web::HttpRequest,
    stream: web::Payload,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let resp = ws::start(WebSocket::new(), &req, stream)?;
    Ok(resp)
}
