#![feature(async_await)]

use tokio::net::TcpStream;
use tower_h2::Connection;
use tower_service::Service;
use http::Request;
use tokio_buf::BufStream;
use std::task::{Context, Poll};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:5928";
    let io = TcpStream::connect(&addr.parse()?).await?;

    let mut svc = Connection::handshake(io).await?;

    let req = Request::get(format!("http://{}", addr)).body(Body::from(Vec::new()))?;
    let res = svc.send(req).await?;

    println!("RESPONSE={:?}", res);

    Ok(())
}

#[derive(Debug, Default, Clone)]
struct Body(Vec<u8>);

impl From<Vec<u8>> for Body {
    fn from(t: Vec<u8>) -> Self {
        Body(t)
    }
}

impl BufStream for Body {
    type Item = std::io::Cursor<Vec<u8>>;
    type Error = std::io::Error;

    fn poll_buf(&mut self, _cx: &mut Context<'_>) -> Poll<Option<Result<Self::Item, Self::Error>>> {
        if self.0.is_empty() {
            return None.into();
        }

        use std::{mem, io};

        let bytes = mem::replace(&mut self.0, Default::default());
        let buf = io::Cursor::new(bytes);

        Some(Ok(buf)).into()
    }
}
