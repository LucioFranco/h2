use std::future::Future;
use std::task::{Poll, Context};
use std::pin::Pin;
use std::fmt;
use futures::ready;
use futures::FutureExt as _;

/// Future extension helpers that are useful for tests
pub trait FutureExt<V, E>: Future<Output = Result<V, E>> {
    /// Panic on error
    fn unwrap(self) -> Unwrap<Self>
    where
        Self: Sized,
        E: fmt::Debug,
    {
        Unwrap {
            inner: self,
        }
    }

    /// Panic on success, yielding the content of an `Err`.
    fn unwrap_err(self) -> UnwrapErr<Self>
    where
        Self: Sized,
        E: fmt::Debug,
    {
        UnwrapErr {
            inner: self,
        }
    }

    /// Panic on success, with a message.
    fn expect_err<T>(self, msg: T) -> ExpectErr<Self>
    where
        Self: Sized,
        E: fmt::Debug,
        T: fmt::Display,
    {
        ExpectErr{
            inner: self,
            msg: msg.to_string(),
        }
    }

    /// Panic on error, with a message.
    fn expect<T>(self, msg: T) -> Expect<Self>
    where
        Self: Sized,
        E: fmt::Debug,
        T: fmt::Display,
    {
        Expect {
            inner: self,
            msg: msg.to_string(),
        }
    }

    /// Drive `other` by polling `self`.
    ///
    /// `self` must not resolve before `other` does.
    fn drive<T>(self, other: T) -> Drive<Self, T>
    where
        T: Future,
        Self: Future + Sized,
    {
        Drive {
            driver: Some(self),
            future: other,
        }
    }

    /// Wrap this future in one that will yield NotReady once before continuing.
    ///
    /// This allows the executor to poll other futures before trying this one
    /// again.
    fn yield_once(self) -> Box<dyn Future<Output=Self::Output>>
    where
        Self: Future + Sized + 'static,
    {
        Box::new(super::util::yield_once().then(move |_| self))
    }
}

impl<T: Future<Output = Result<V, E>>, V, E> FutureExt<V, E> for T {}

// ===== Unwrap ======

/// Panic on error
pub struct Unwrap<T> {
    inner: T,
}

impl<T, V, E> Future for Unwrap<T>
where
    T: Future<Output=Result<V, E>> + Unpin,
    V: fmt::Debug,
    E: fmt::Debug,
{
    type Output = V;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<V> {
        let pinned = Pin::get_mut(self);
        let result = ready!(Pin::new(&mut pinned.inner).poll(cx));
        Poll::Ready(result.unwrap())
    }
}

// ===== UnwrapErr ======

/// Panic on success.
pub struct UnwrapErr<T> {
    inner: T,
}

impl<T, V, E> Future for UnwrapErr<T>
where
    T: Future<Output=Result<V, E>> + Unpin,
    V: fmt::Debug,
    E: fmt::Debug,
{
    type Output = E;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<E> {
        let pinned = Pin::get_mut(self);
        match ready!(Pin::new(&mut pinned.inner).poll(cx)) {
            Ok(v) => panic!("Future::unwrap_err() on an Ok value: {:?}", v),
            Err(e) => Poll::Ready(e),
        }
    }
}



// ===== Expect ======

/// Panic on error
pub struct Expect<T> {
    inner: T,
    msg: String,
}

impl<T, V, E> Future for Expect<T>
where
    T: Future<Output=Result<V, E>> + Unpin,
    V: fmt::Debug,
    E: fmt::Debug,
{
    type Output = V;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<V> {
        let pinned = Pin::get_mut(self);
        let result = ready!(Pin::new(&mut pinned.inner).poll(cx));
        Poll::Ready(result.expect(&pinned.msg))
    }
}

// ===== ExpectErr ======

/// Panic on success
pub struct ExpectErr<T> {
    inner: T,
    msg: String,
}

impl<T, V, E> Future for ExpectErr<T>
where
    T: Future<Output=Result<V, E>> + Unpin,
    V: fmt::Debug,
    E: fmt::Debug,
{
    type Output = E;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<E> {
        let pinned = Pin::get_mut(self);
        match ready!(Pin::new(&mut pinned.inner).poll(cx)) {
            Ok(v) => panic!("{}: {:?}", pinned.msg, v),
            Err(e) => Poll::Ready(e),
        }
    }
}

// ===== Drive ======

/// Drive a future to completion while also polling the driver
///
/// This is useful for H2 futures that also require the connection to be polled.
pub struct Drive<T, U> {
    driver: Option<T>,
    future: U,
}

impl<T, U, V, E1, E2> Future for Drive<T, U>
where
    T: Future<Output = Result<(), E1>> + Unpin,
    U: Future<Output = Result<V, E2>> + Unpin,
    E1: fmt::Debug,
    E2: fmt::Debug,
{
    type Output = (T, V);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut looped = false;
        let pinned = Pin::get_mut(self);
        loop {
            match Pin::new(&mut pinned.future).poll(cx) {
                Poll::Ready(Ok(val)) => {
                    // Get the driver
                    let driver = pinned.driver.take().unwrap();

                    return Poll::Ready((driver, val));
                },
                Poll::Pending => {},
                Poll::Ready(Err(e)) => panic!("unexpected error; {:?}", e),
            }

            match Pin::new(pinned.driver.as_mut().unwrap()).poll(cx) {
                Poll::Ready(Ok(_)) => {
                    if looped {
                        // Try polling the future one last time
                        panic!("driver resolved before future")
                    } else {
                        looped = true;
                        continue;
                    }
                },
                Poll::Pending => {},
                Poll::Ready(Err(e)) => panic!("unexpected error; {:?}", e),
            }

            return Poll::Pending;
        }
    }
}
