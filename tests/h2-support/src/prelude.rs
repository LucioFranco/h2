// Re-export H2 crate
pub use h2;

pub use h2::client;
pub use h2::frame::StreamId;
pub use h2::server;
pub use h2::*;

// Re-export mock
pub use super::mock::{self, idle_ms, HandleFutureExt};

// Re-export frames helpers
pub use super::frames;

// Re-export mock notify
pub use super::notify::MockNotify;

// Re-export utility mod
pub use super::util;

// Re-export some type defines
pub use super::{Codec, SendFrame};

// Re-export macros
pub use super::{
    assert_closed, assert_data, assert_default_settings, assert_headers, assert_ping, poll_err,
    poll_frame, raw_codec,
};

pub use super::assert::assert_frame_eq;

// Re-export useful crates
pub use super::mock_io;
pub use {bytes, env_logger, futures, http, tokio::io as tokio_io};

// Re-export primary future types
pub use futures::{Future, Sink, Stream};

// And our Future extensions
pub use super::future_ext::{FutureExt, Unwrap};

// Our client_ext helpers
pub use super::client_ext::SendRequestExt;

// Re-export HTTP types
pub use http::{uri, HeaderMap, Method, Request, Response, StatusCode, Version};

pub use bytes::{Buf, BufMut, Bytes, BytesMut, IntoBuf};

pub use tokio::io::{AsyncRead, AsyncWrite};

pub use std::thread;
pub use std::time::Duration;

// ===== Everything under here shouldn't be used =====
// TODO: work on deleting this code

pub use futures::future::poll_fn;

pub trait MockH2 {
    fn handshake(&mut self) -> &mut Self;
}

impl MockH2 for super::mock_io::Builder {
    fn handshake(&mut self) -> &mut Self {
        self.write(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n")
            // Settings frame
            .write(frames::SETTINGS)
            .read(frames::SETTINGS)
            .read(frames::SETTINGS_ACK)
    }
}

pub trait ClientExt {
    fn run<F: Future + Unpin>(&mut self, f: F) -> F::Output;
}

impl<T, B> ClientExt for client::Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Unpin + 'static,
    B: IntoBuf + Unpin + 'static,
    B::Buf: Unpin,
{
    fn run<F: Future + Unpin>(&mut self, f: F) -> F::Output {
        use futures::future::Either::*;
        use futures::future::{self, FutureExt};

        let res = future::select(self.fuse(), f.fuse());

        // This is not right
        let res = futures::executor::block_on(res);
        match res {
            Left((Ok(_), b)) => {
                // Connection is done...
                futures::executor::block_on(b)
            }
            Right((v, _)) => return v,
            Left((Err(e), _)) => panic!("err: {:?}", e),
        }
    }
}

pub fn build_large_headers() -> Vec<(&'static str, String)> {
    vec![
        ("one", "hello".to_string()),
        ("two", build_large_string('2', 4 * 1024)),
        ("three", "three".to_string()),
        ("four", build_large_string('4', 4 * 1024)),
        ("five", "five".to_string()),
        ("six", build_large_string('6', 4 * 1024)),
        ("seven", "seven".to_string()),
        ("eight", build_large_string('8', 4 * 1024)),
        ("nine", "nine".to_string()),
        ("ten", build_large_string('0', 4 * 1024)),
    ]
}

fn build_large_string(ch: char, len: usize) -> String {
    let mut ret = String::new();

    for _ in 0..len {
        ret.push(ch);
    }

    ret
}
