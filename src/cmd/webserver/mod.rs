use axum::routing::get;
use std::{net::SocketAddr, str::FromStr};

mod index;
mod ws;

pub async fn run(addr: String) -> std::io::Result<()> {
    log::info!("服务已启动 {}", addr);

    let app = axum::Router::new()
        .route("/", get(index::root))
        .route("/ws", get(ws::websocket));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from_str(addr.as_str()).expect("解析socketaddr失败");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
