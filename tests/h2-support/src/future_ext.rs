use futures::ready;
use futures::{FutureExt as _, TryFuture};
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Future extension helpers that are useful for tests
pub trait FutureExt {
    /// Panic on error
    fn unwrap(self) -> Unwrap<Self>
    where
        Self: TryFuture + Sized,
        Self::Error: fmt::Debug,
    {
        Unwrap { inner: self }
    }

    /// Panic on success, yielding the content of an `Err`.
    fn unwrap_err(self) -> UnwrapErr<Self>
    where
        Self: TryFuture + Sized,
        Self::Error: fmt::Debug,
    {
        UnwrapErr { inner: self }
    }

    /// Panic on success, with a message.
    fn expect_err<T>(self, msg: T) -> ExpectErr<Self>
    where
        Self: TryFuture + Sized,
        Self::Error: fmt::Debug,
        T: fmt::Display,
    {
        ExpectErr {
            inner: self,
            msg: msg.to_string(),
        }
    }

    /// Panic on error, with a message.
    fn expect<T>(self, msg: T) -> Expect<Self>
    where
        Self: TryFuture + Sized,
        Self::Error: fmt::Debug,
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
    fn drive<T>(&mut self, other: T) -> Drive<'_, Self, T>
    where
        T: Future,
        Self: Future + Sized,
    {
        Drive {
            driver: self,
            future: other,
        }
    }

    /// Wrap this future in one that will yield NotReady once before continuing.
    ///
    /// This allows the executor to poll other futures before trying this one
    /// again.
    fn yield_once(self) -> Box<dyn Future<Output = Self::Output>>
    where
        Self: Future + Sized + 'static,
    {
        Box::new(super::util::yield_once().then(move |_| self))
    }
}

impl<T: Future> FutureExt for T {}

// ===== Unwrap ======

/// Panic on error
pub struct Unwrap<T> {
    inner: T,
}

impl<T> Future for Unwrap<T>
where
    T: TryFuture + Unpin,
    T::Ok: fmt::Debug,
    T::Error: fmt::Debug,
{
    type Output = T::Ok;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pinned = Pin::get_mut(self);
        let result = ready!(Pin::new(&mut pinned.inner).try_poll(cx));
        Poll::Ready(result.unwrap())
    }
}

// ===== UnwrapErr ======

/// Panic on success.
pub struct UnwrapErr<T> {
    inner: T,
}

impl<T> Future for UnwrapErr<T>
where
    T: TryFuture + Unpin,
    T::Ok: fmt::Debug,
    T::Error: fmt::Debug,
{
    type Output = T::Error;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T::Error> {
        let pinned = Pin::get_mut(self);
        match ready!(Pin::new(&mut pinned.inner).try_poll(cx)) {
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

impl<T> Future for Expect<T>
where
    T: TryFuture + Unpin,
    T::Ok: fmt::Debug,
    T::Error: fmt::Debug,
{
    type Output = T::Ok;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pinned = Pin::get_mut(self);
        let result = ready!(Pin::new(&mut pinned.inner).try_poll(cx));
        Poll::Ready(result.expect(&pinned.msg))
    }
}

// ===== ExpectErr ======

/// Panic on success
pub struct ExpectErr<T> {
    inner: T,
    msg: String,
}

impl<T> Future for ExpectErr<T>
where
    T: TryFuture + Unpin,
    T::Ok: fmt::Debug,
    T::Error: fmt::Debug,
{
    type Output = T::Error;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pinned = Pin::get_mut(self);
        match ready!(Pin::new(&mut pinned.inner).try_poll(cx)) {
            Ok(v) => panic!("{}: {:?}", pinned.msg, v),
            Err(e) => Poll::Ready(e),
        }
    }
}

// ===== Drive ======

/// Drive a future to completion while also polling the driver
///
/// This is useful for H2 futures that also require the connection to be polled.
pub struct Drive<'a, T, U> {
    driver: &'a mut T,
    future: U,
}

impl<'a, T, U> Future for Drive<'a, T, U>
where
    T: Future + Unpin,
    U: Future + Unpin,
{
    type Output = U::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut looped = false;
        let pinned = Pin::get_mut(self);
        loop {
            match pinned.future.poll_unpin(cx) {
                Poll::Ready(val) => return Poll::Ready(val),
                Poll::Pending => {}
            }

            match pinned.driver.poll_unpin(cx) {
                Poll::Ready(_) => {
                    if looped {
                        // Try polling the future one last time
                        panic!("driver resolved before future")
                    } else {
                        looped = true;
                        continue;
                    }
                }
                Poll::Pending => {}
            }

            return Poll::Pending;
        }
    }
}