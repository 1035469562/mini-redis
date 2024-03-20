use mini_redis::server;
use tokio::{net::TcpListener, signal};
#[tokio::main]
pub async fn main() {
    let listen = TcpListener::bind("127.0.0.1:7799").await.unwrap();
    server::run(listen);
}
