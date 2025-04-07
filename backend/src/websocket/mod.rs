use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::{error, info};
use serde_json::json;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct WebSocket {
    hb: Instant,
}

impl Default for WebSocket {
    fn default() -> Self {
        Self { hb: Instant::now() }
    }
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket connection started");
        self.hb(ctx);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!("WebSocket connection stopped");
    }
}

impl WebSocket {
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                error!("Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                info!("Received text message: {}", text);
                // Handle incoming messages here
                let response = json!({
                    "type": "ack",
                    "message": "Message received",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });
                ctx.text(response.to_string());
            }
            Ok(ws::Message::Binary(bin)) => {
                info!("Received binary message: {:?}", bin);
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Err(e) => {
                error!("WebSocket error: {:?}", e);
                ctx.stop();
            }
            _ => (),
        }
    }
}

pub async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    info!("New WebSocket connection request");
    let resp = ws::start(WebSocket::default(), &req, stream);
    info!("WebSocket connection established");
    resp
}
