use std::net::TcpListener;
use myweb::run;
#[tokio::main]//过程宏
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    run(listener)?.await
}
