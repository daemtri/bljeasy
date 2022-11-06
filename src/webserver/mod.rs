use actix_web::{App, HttpServer};

mod index;
mod ws;

pub async fn run(host: String, port: u16) -> std::io::Result<()> {
    log::info!("服务已启动 {}:{}", host, port);
    HttpServer::new(|| {
        App::new()
            .service(index::index)
            .service(index::hello)
            .service(ws::websocket)
    })
    .bind((host, port))?
    .run()
    .await
}
