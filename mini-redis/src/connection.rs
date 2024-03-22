use crate::frame::Frame;
use bytes::{Buf, BytesMut};
use std::io::Cursor;
use tokio::io::BufWriter;
use tokio::net::TcpStream;
#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}
impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }
    pub async fn read_frame(&mut self) {
        loop {
            self.parse_frame();
        }
    }
    fn parse_frame(&mut self) {
        let mut buf = Cursor::new(&self.buffer[..]);
        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let frame = Frame::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(frame()))
            }
            Err(Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
