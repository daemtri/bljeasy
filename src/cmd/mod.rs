use clap::Parser;
use std::time::Duration;

pub mod webserver;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("127.0.0.1:8080"), help = "监听地址")]
    addr: String,
}

pub fn execute() -> std::io::Result<()> {
    env_logger::init();

    let args = Args::parse();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("create tokio runtime failed");

    runtime.block_on(run(args.addr))
}

pub async fn run(addr: String) -> std::io::Result<()> {
    tokio::spawn(async {
        loop {
            println!("spawn");
            tokio::time::sleep(Duration::from_secs(1)).await
        }
    });
    webserver::run(addr).await
}
