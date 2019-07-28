use crate::{FutureExt, SendFrame};

use h2::{self, RecvError, SendError};
use h2::frame::{self, Frame};

use futures::{ready, Stream, StreamExt, TryFutureExt, FutureExt as FutureExt_};
use futures::future::poll_fn;
use futures::channel::oneshot;

use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};

use std::{cmp, fmt, io, usize};
use std::sync::{Arc, Mutex};
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::future::Future;
use futures::executor::block_on;

/// A mock I/O
#[derive(Debug)]
pub struct Mock {
    pipe: Pipe,
}

#[derive(Debug)]
pub struct Handle {
    codec: crate::Codec<Pipe>,
}

#[derive(Debug)]
pub struct Pipe {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Debug)]
struct Inner {
    /// Data written by the test case to the h2 lib.
    rx: Vec<u8>,

    /// Notify when data is ready to be received.
    rx_task: Option<Waker>,

    /// Data written by the `h2` library to be read by the test case.
    tx: Vec<u8>,

    /// Notify when data is written. This notifies the test case waiters.
    tx_task: Option<Waker>,

    /// Number of bytes that can be written before `write` returns `NotReady`.
    tx_rem: usize,

    /// Task to notify when write capacity becomes available.
    tx_rem_task: Option<Waker>,

    /// True when the pipe is closed.
    closed: bool,
}

const PREFACE: &'static [u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

/// Create a new mock and handle
pub fn new() -> (Mock, Handle) {
    new_with_write_capacity(usize::MAX)
}

/// Create a new mock and handle allowing up to `cap` bytes to be written.
pub fn new_with_write_capacity(cap: usize) -> (Mock, Handle) {
    let inner = Arc::new(Mutex::new(Inner {
        rx: vec![],
        rx_task: None,
        tx: vec![],
        tx_task: None,
        tx_rem: cap,
        tx_rem_task: None,
        closed: false,
    }));

    let mock = Mock {
        pipe: Pipe {
            inner: inner.clone(),
        },
    };

    let handle = Handle {
        codec: h2::Codec::new(Pipe {
            inner,
        }),
    };

    (mock, handle)
}

// ===== impl Handle =====

impl Handle {
    /// Get a mutable reference to inner Codec.
    pub fn codec_mut(&mut self) -> &mut crate::Codec<Pipe> {
        &mut self.codec
    }

    /// Send a frame
    pub async fn send(&mut self, item: SendFrame) -> Result<(), SendError> {
        // Queue the frame
        self.codec.buffer(item).unwrap();

        // Flush the frame
        poll_fn(|cx| self.codec.flush(cx)).await?;
        Ok(())
    }

    /// Writes the client preface
    pub async fn write_preface(&mut self) {
        self.codec.get_mut().write_all(PREFACE).await.unwrap();
    }

    /// Read the client preface
    pub async fn read_preface(&mut self) -> io::Result<()> {
        let mut buf = vec![0u8; PREFACE.len()];
        self.read_exact(&mut buf).await?;
        Ok(())
    }

    /// Perform the H2 handshake
    pub async fn assert_client_handshake(
        &mut self,
    ) -> frame::Settings {
        self.assert_client_handshake_with_settings(frame::Settings::default()).await
    }

    /// Perform the H2 handshake
    pub async fn assert_client_handshake_with_settings<T>(
        &mut self,
        settings: T,
    ) -> frame::Settings
    where
        T: Into<frame::Settings>,
    {
        let settings = settings.into();
        // Send a settings frame
        self.send(settings.into()).await.unwrap();
        self.read_preface().await.unwrap();

        let frame = self.next().await;
        let settings = match frame {
            Some(frame) => match frame.unwrap() {
                Frame::Settings(settings) => {
                    // Send the ACK
                    let ack = frame::Settings::ack();

                    // TODO: Don't unwrap?
                    self.send(ack.into()).await.unwrap();

                    settings
                },
                frame => {
                    panic!("unexpected frame; frame={:?}", frame);
                },
            },
            None => {
                panic!("unexpected EOF");
            },
        };

        let frame = self.next().await;
        let f = assert_settings!(frame.unwrap().unwrap());

        // Is ACK
        assert!(f.is_ack());

        settings
    }


    /// Perform the H2 handshake
    pub async fn assert_server_handshake(
        &mut self,
    ) -> frame::Settings {
        self.assert_server_handshake_with_settings(frame::Settings::default()).await
    }

    /// Perform the H2 handshake
    pub async fn assert_server_handshake_with_settings<T>(
        &mut self,
        settings: T,
    ) -> frame::Settings
    where
        T: Into<frame::Settings>,
    {
        self.write_preface().await;

        let settings = settings.into();
        self.send(settings.into()).await.unwrap();

        let frame = self.next().await;
        let settings = match frame {
            Some(frame) => match frame.unwrap() {
                Frame::Settings(settings) => {
                    // Send the ACK
                    let ack = frame::Settings::ack();

                    // TODO: Don't unwrap?
                    self.send(ack.into()).await.unwrap();

                    settings
                },
                frame => panic!("unexpected frame; frame={:?}", frame)
            },
            None => panic!("unexpected EOF")
        };
        
        let frame = self.next().await;
        let f = assert_settings!(frame.unwrap().unwrap());

        // Is ACK
        assert!(f.is_ack());

        settings
    }
}

impl Stream for Handle {
    type Item = Result<Frame, RecvError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut Pin::get_mut(self).codec).poll_next(cx)
    }
}

impl io::Read for Handle {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        block_on(async {
            self.codec.get_mut().read(buf).await
        })
    }
}

impl AsyncRead for Handle {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(Pin::get_mut(self).codec.get_mut()).poll_read(cx, buf)
    }
}

impl AsyncWrite for Handle {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        Pin::new(Pin::get_mut(self).codec.get_mut()).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Pin::new(Pin::get_mut(self).codec.get_mut()).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Pin::new(Pin::get_mut(self).codec.get_mut()).poll_shutdown(cx)
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        block_on(async {
            poll_fn(|cx| {
                assert!(self.codec.shutdown(cx).is_ready());

                let mut me = self.codec.get_mut().inner.lock().unwrap();
                me.closed = true;

                if let Some(task) = me.rx_task.take() {
                    task.wake();
                }
                Poll::Ready(())
            }).await;
        });
    }
}

// ===== impl Mock =====

impl AsyncRead for Mock {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        assert!(
            buf.len() > 0,
            "attempted read with zero length buffer... wut?"
        );

        let mut me = Pin::get_mut(self).pipe.inner.lock().unwrap();

        if me.rx.is_empty() {
            if me.closed {
                return Poll::Ready(Ok(0));
            }

            me.rx_task = Some(cx.waker().clone());
            return Poll::Pending;
        }

        let n = cmp::min(buf.len(), me.rx.len());
        buf[..n].copy_from_slice(&me.rx[..n]);
        me.rx.drain(..n);

        Poll::Ready(Ok(n))
    }
}

impl AsyncWrite for Mock {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        let mut me = Pin::get_mut(self).pipe.inner.lock().unwrap();

        if me.closed {
            return Poll::Ready(Ok(buf.len()));
        }

        if me.tx_rem == 0 {
            me.tx_rem_task = Some(cx.waker().clone());
            return Poll::Pending;
        }

        if buf.len() > me.tx_rem {
            buf = &buf[..me.tx_rem];
        }

        me.tx.extend(buf);
        me.tx_rem -= buf.len();

        if let Some(task) = me.tx_task.take() {
            task.wake();
        }

        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl Drop for Mock {
    fn drop(&mut self) {
        let mut me = self.pipe.inner.lock().unwrap();
        me.closed = true;

        if let Some(task) = me.tx_task.take() {
            task.wake();
        }
    }
}

// ===== impl Pipe =====

impl AsyncRead for Pipe {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        assert!(
            buf.len() > 0,
            "attempted read with zero length buffer... wut?"
        );

        let mut me = self.inner.lock().unwrap();


        if me.tx.is_empty() {
            if me.closed {
                return Poll::Ready(Ok(0));
            }

            me.tx_task = Some(cx.waker().clone());
            return Poll::Pending;
        }

        let n = cmp::min(buf.len(), me.tx.len());
        buf[..n].copy_from_slice(&me.tx[..n]);
        me.tx.drain(..n);

        Poll::Ready(Ok(n))
    }
}

impl AsyncWrite for Pipe {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        let mut me = self.inner.lock().unwrap();
        me.rx.extend(buf);

        if let Some(task) = me.rx_task.take() {
            task.wake();
        }

        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub trait HandleFutureExt {
    fn recv_settings<E>(self)
        -> RecvFrame<Box<dyn Future<Output = (Option<Frame>, Handle)>>>
    where
        Self: Sized + Unpin + 'static,
        Self: Future<Output = Result<(frame::Settings, Handle), E>>,
        E: fmt::Debug,
    {
        self.recv_custom_settings(frame::Settings::default())
    }

    fn recv_custom_settings<T, E>(self, settings: T)
        -> RecvFrame<Box<dyn Future<Output = (Option<Frame>, Handle)>>>
    where
        Self: Sized + Unpin + 'static,
        Self: Future<Output = Result<(frame::Settings, Handle), E>>,
        E: fmt::Debug,
        T: Into<frame::Settings>,
    {
        let map = self
            .map(|result| result.map(|(settings, handle)| (Some(settings.into()), handle)))
            .unwrap();

        let boxed: Box<dyn Future<Output = (Option<Frame>, Handle)>> = Box::new(map);
        RecvFrame {
            inner: boxed,
            frame: Some(settings.into().into()),
        }
    }

    fn ignore_settings<E>(self) -> Box<dyn Future<Output = Handle>>
    where
        Self: Sized + Unpin + 'static,
        Self: Future<Output = Result<(frame::Settings, Handle), E>>,
        E: fmt::Debug,
    {
        Box::new(self.map(|result| result.map(|(_settings, handle)| handle)).unwrap())
    }

    fn recv_frame<T>(self, frame: T) -> RecvFrame<<Self as IntoRecvFrame>::Future>
    where
        Self: IntoRecvFrame + Sized,
        T: Into<Frame>,
    {
        self.into_recv_frame(Some(frame.into()))
    }

    fn recv_eof(self) -> RecvFrame<<Self as IntoRecvFrame>::Future>
    where
        Self: IntoRecvFrame + Sized,
    {
        self.into_recv_frame(None)
    }

    fn send_frame<T, E>(self, frame: T) -> Pin<Box<dyn Future<Output = Handle>>>
    where
        Self: Future<Output = Result<Handle, E>> + Sized + Unpin + 'static,
        T: Into<SendFrame> + 'static,
        E: fmt::Debug,
    {
        Box::pin(async move {
            let mut handle = self.await.unwrap();
            handle.send(frame.into()).await.unwrap();
            handle
        })
    }

    fn send_bytes<E>(self, data: &[u8]) -> Box<dyn Future<Output = Result<Handle, E>>>
    where
        Self: Future<Output = Result<Handle, E>> + Sized + 'static,
        E: fmt::Debug,
    {
        use bytes::Buf;
        use std::io::Cursor;

        let buf: Vec<_> = data.into();
        let mut buf = Cursor::new(buf);

        Box::new(self.and_then(move |handle| {
            let mut handle = Some(handle);

            poll_fn(move |cx| {
                while buf.has_remaining() {
                    let res = Pin::new(handle.as_mut().unwrap().codec.get_mut())
                        .poll_write_buf(cx, &mut buf)
                        .map_err(|e| panic!("write err={:?}", e));

                    ready!(res)?;
                }

                Poll::Ready(Ok(handle.take().unwrap().into()))
            })
        }))
    }

    // fn ping_pong<E>(self, payload: [u8; 8]) -> RecvFrame<<SendFrameFut<Self> as IntoRecvFrame>::Future>
    // where
    //     Self: Future<Output=Result<Handle, E>> + Sized + Unpin + 'static,
    //     E: fmt::Debug,
    // {
    //     self.send_frame(frames::ping(payload))
    //         .recv_frame(frames::ping(payload).pong())
    // }

    fn idle_ms<E>(self, ms: usize) -> Box<dyn Future<Output = Result<Handle, E>>>
    where
        Self: Sized + 'static,
        Self: Future<Output = Result<Handle, E>>,
        E: fmt::Debug,
    {
        use std::thread;
        use std::time::Duration;


        Box::new(self.and_then(move |handle| {
            // This is terrible... but oh well
            let (tx, rx) = oneshot::channel();

            thread::spawn(move || {
                thread::sleep(Duration::from_millis(ms as u64));
                tx.send(()).unwrap();
            });

            Idle {
                handle: Some(handle),
                timeout: rx,
            }.map_err(|_| unreachable!())
        }))
    }

    fn buffer_bytes<E>(self, num: usize) -> Box<dyn Future<Output = Result<Handle, E>>>
    where Self: Sized + 'static,
          Self: Future<Output = Result<Handle, E>>,
          E: fmt::Debug,
    {
        Box::new(self.and_then(move |mut handle| {
            // Set tx_rem to num
            {
                let mut i = handle.codec.get_mut().inner.lock().unwrap();
                i.tx_rem = num;
            }

            let mut handle = Some(handle);

            poll_fn(move |cx| {
                {
                    let mut inner = handle.as_mut().unwrap()
                        .codec.get_mut().inner.lock().unwrap();

                    if inner.tx_rem == 0 {
                        inner.tx_rem = usize::MAX;
                    } else {
                        inner.tx_task = Some(cx.waker().clone());
                        return Poll::Pending;
                    }
                }

                Poll::Ready(Ok(handle.take().unwrap().into()))
            })
        }))
    }

    fn unbounded_bytes<E>(self) -> Box<dyn Future<Output = Result<Handle, E>>>
    where Self: Sized + 'static,
          Self: Future<Output = Result<Handle, E>>,
          E: fmt::Debug,
    {
        Box::new(async {
            match self.await {
                Ok(mut handle) => {
                    {
                        let mut i = handle.codec.get_mut().inner.lock().unwrap();
                        i.tx_rem = usize::MAX;

                        if let Some(task) = i.tx_rem_task.take() {
                            task.wake();
                        }
                    }
                    Ok(handle)
                },
                Err(e) => Err(e),
            }
        })
    }

    fn then_notify(self, tx: oneshot::Sender<()>) -> Box<dyn Future<Output = Self::Output>>
    where Self: Future + Sized + 'static,
    {
        Box::new(async {
            let handle = self.await;
            tx.send(()).unwrap();
            handle
        })
    }

    fn wait_for<F>(self, other: F) -> Box<dyn Future<Output = Self::Output>>
    where
        F: Future + 'static,
        Self: Future + Sized + 'static
    {
        Box::new(async {
            let result = self.await;
            other.await;
            result
        })
    }

    fn close(self) -> Box<dyn Future<Output = ()>>
    where
        Self: Future<Output = ()> + Sized + 'static,
    {
        Box::new(self.map(drop))
    }
}

pub struct RecvFrame<T> {
    inner: T,
    frame: Option<Frame>,
}

impl<T, E> Future for RecvFrame<T>
where
    T: Future<Output = Result<(Option<Frame>, Handle), E>> + Unpin,
    E: fmt::Debug,
{
    type Output = Handle;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        use self::Frame::Data;

        let pinned = Pin::get_mut(self);
        let (frame, handle) = ready!(Pin::new(&mut pinned.inner).poll(cx)).unwrap();

        match (frame, &pinned.frame) {
            (Some(Data(ref a)), &Some(Data(ref b))) => {
                assert_eq!(a.payload().len(), b.payload().len(), "recv_frame data payload len");
                assert_eq!(a, b, "recv_frame");
            }
            (ref a, b) => {
                assert_eq!(a, b, "recv_frame");
            }
        }

        Poll::Ready(handle)
    }
}

pub struct Idle {
    handle: Option<Handle>,
    timeout: oneshot::Receiver<()>,
}

impl Future for Idle {
    type Output = Result<Handle, ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pinned = Pin::get_mut(self);
        if Pin::new(&mut pinned.timeout).poll(cx).is_ready() {
            return Poll::Ready(Ok(pinned.handle.take().unwrap()));
        }

        let res = ready!(Pin::new(&mut pinned.handle.as_mut().unwrap()).poll_next(cx));
        panic!("Idle received unexpected frame on handle; frame={:?}", res);
    }
}

impl<T> HandleFutureExt for T
where
    T: Future + 'static,
{
}

pub trait IntoRecvFrame {
    type Future: Future;
    fn into_recv_frame(self, frame: Option<Frame>) -> RecvFrame<Self::Future>;
}

impl IntoRecvFrame for Handle {
    type Future = ::futures::stream::StreamFuture<Self>;

    fn into_recv_frame(self, frame: Option<Frame>) -> RecvFrame<Self::Future> {
        RecvFrame {
            inner: self.into_future(),
            frame: frame,
        }
    }
}

impl<T, E> IntoRecvFrame for T
where
    T: Future<Output = Result<Handle, E>> + Unpin + 'static,
    E: fmt::Debug,
{
    type Future = Pin<Box<dyn Future<Output = (Option<Frame>, Handle)>>>;

    fn into_recv_frame(self, frame: Option<Frame>) -> RecvFrame<Self::Future> {
        let into_fut = Box::pin(async {
            let mut handle = self.await.unwrap();
            let frame = handle.next().await;
            let frame = if let Some(frame) = frame {
                Some(frame.unwrap())
            } else {
                None
            };
            (frame, handle)
        });
        RecvFrame {
            inner: into_fut,
            frame: frame,
        }
    }
}
