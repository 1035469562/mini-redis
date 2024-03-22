use crate::Connection;
use std::{fmt::Error, future::Future};
use tokio::net::{TcpListener, TcpStream};
struct Listener {
    listener: TcpListener,
}
pub async fn run(listener: TcpListener) {
    let mut lis = Listener { listener };
    tokio::select! {
        res = lis.listen() =>{

        }
    }
}
impl Listener {
    // 开启监听
    async fn listen(&mut self) -> crate::Result<TcpStream> {
        loop {
            let socket = self.accept().await?;
            let mut handler = Handler {
                connection: Connection::new(socket),
            };
            tokio::spawn(async move {
                handler.handle().await;
            });
        }
    }
    async fn accept(&mut self) -> crate::Result<TcpStream> {
        match self.listener.accept().await {
            Ok((socket, _)) => return Ok(socket),
            Err(err) => return Err(err.into()),
        };
    }
}

struct Handler {
    connection: Connection,
}
impl Handler {
    async fn handle(&mut self) {
        loop {
            let maybe_frame = tokio::select! {
               res =  self.connection.read_frame() => res,
            };
        }
    }
}
