use actix_web::{get, web, HttpRequest, Responder};
use actix_web_actors::ws;

// http://wstool.js.org/
// https://rust-book.junmajinlong.com/ch100/05_task_communication.html
// https://zhuanlan.zhihu.com/p/421571942
#[get("/ws")]
async fn websocket(req: HttpRequest, stream: web::Payload) -> impl Responder {
    log::info!("进入ws");
    let conn = WsConn { nick: "xxx".into() };

    let resp = ws::start(conn, &req, stream);

    match resp {
        Ok(ret) => ret,
        Err(e) => e.error_response(),
    }
}

pub struct WsConn {
    pub nick: String,
}

impl actix::Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("{} join!", self.nick);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("{} exit!", self.nick);
    }
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Text(text)) => match text.to_string().as_str() {
                "a" => ctx.text("a"),
                _ => ctx.text(text),
            },
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}
