use axum::extract::ws;
use axum::response::IntoResponse;

pub async fn websocket(ws: ws::WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: ws::WebSocket) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                ws::Message::Text(t) => {
                    println!("client sent str: {:?}", t);
                }
                ws::Message::Binary(_) => {
                    println!("client sent binary data");
                }
                ws::Message::Ping(_) => {
                    println!("socket ping");
                }
                ws::Message::Pong(_) => {
                    println!("socket pong");
                }
                ws::Message::Close(_) => {
                    println!("client disconnected");
                    return;
                }
            }
        } else {
            println!("client disconnected");
            return;
        }
    }

    loop {
        if socket
            .send(ws::Message::Text(String::from("Hi!")))
            .await
            .is_err()
        {
            println!("client disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}
