use clap::Parser;
use std::time::Duration;

pub mod webserver;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("127.0.0.1"), help = "监听IP")]
    host: String,

    #[arg(long, default_value_t = 8080, help = "监听端口")]
    port: u16,
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = Args::parse();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(10);

    tokio::spawn(async {
        loop {
            println!("spawn");
            tokio::time::sleep(Duration::from_secs(1)).await
        }
    });

    webserver::run(args.host, args.port).await
}
