#![feature(async_await)]

use h2::client;
use std::future::Future;
use std::pin::Pin;
use std::task::{Poll, Context};
use futures::{ready, Stream};
use h2::RecvStream;
use http::{Request, HeaderMap};

use std::error::Error;

use tokio::net::TcpStream;

struct Process {
    body: RecvStream,
    trailers: bool,
}

impl Future for Process {
    type Output = Result<(), h2::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pinned = Pin::get_mut(self);
        loop {
            if pinned.trailers {
                let trailers = ready!(pinned.body.poll_trailers(cx));

                println!("GOT TRAILERS: {:?}", trailers);

                return Poll::Ready(Ok(()))
            } else {
                match ready!(Pin::new(&mut pinned.body).poll_next(cx)) {
                    Some(Ok(chunk)) => {
                        println!("GOT CHUNK = {:?}", chunk);
                    },
                    Some(Err(e)) => return Poll::Ready(Err(e)),
                    None => {
                        pinned.trailers = true;
                    },
                }
            }
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let _ = env_logger::try_init();

    let tcp = TcpStream::connect(&"127.0.0.1:5928".parse().unwrap()).await?;
    let (mut client, h2) = client::handshake(tcp).await?;

    println!("sending request");

    let request = Request::builder()
        .uri("https://http2.akamai.com/")
        .body(())
        .unwrap();

    let mut trailers = HeaderMap::new();
    trailers.insert("zomg", "hello".parse().unwrap());

    let (response, mut stream) = client.send_request(request, false).unwrap();

    // send trailers
    stream.send_trailers(trailers).unwrap();

    // Spawn a task to run the conn...
    tokio::spawn(async move {
        if let Err(e) = h2.await {
            println!("GOT ERR={:?}", e);
        }
    });

    let response = response.await?;
    println!("GOT RESPONSE: {:?}", response);

    // Get the body
    let (_, body) = response.into_parts();

    Process {
        body,
        trailers: false,
    }.await?;
    
    Ok(())
}
