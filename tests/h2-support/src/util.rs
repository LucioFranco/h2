use h2;

use string::{String, TryFrom};
use bytes::Bytes;
use std::future::Future;
use std::task::{Poll, Context};
use std::pin::Pin;
use futures::ready;

pub fn byte_str(s: &str) -> String<Bytes> {
    String::try_from(Bytes::from(s)).unwrap()
}

pub fn yield_once() -> impl Future<Output=()> {
    let mut yielded = false;
    futures::future::poll_fn(move |cx| {
        if yielded {
           Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().clone().wake();
            Poll::Pending
        }
    })
}

pub fn wait_for_capacity(stream: h2::SendStream<Bytes>, target: usize) -> WaitForCapacity {
    WaitForCapacity {
        stream: Some(stream),
        target: target,
    }
}

pub struct WaitForCapacity {
    stream: Option<h2::SendStream<Bytes>>,
    target: usize,
}

impl WaitForCapacity {
    fn stream(&mut self) -> &mut h2::SendStream<Bytes> {
        self.stream.as_mut().unwrap()
    }
}

impl Future for WaitForCapacity {
    type Output = h2::SendStream<Bytes>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let _ = ready!(self.stream().poll_capacity(cx)).unwrap();

        let act = self.stream().capacity();

        if act >= self.target {
            return Poll::Ready(self.stream.take().unwrap().into());
        }

        Poll::Pending
    }
}
