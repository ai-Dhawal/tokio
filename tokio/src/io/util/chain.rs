use crate::io::{AsyncBufRead, AsyncRead, ReadBuf};
use pin_project_lite::pin_project;
use std::fmt;
use std::io;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
pin_project! {
    #[doc = " Stream for the [`chain`](super::AsyncReadExt::chain) method."] #[must_use =
    "streams do nothing unless polled"] #[cfg_attr(docsrs, doc(cfg(feature =
    "io-util")))] pub struct Chain < T, U > { #[pin] first : T, #[pin] second : U,
    done_first : bool, }
}
pub(super) fn chain<T, U>(first: T, second: U) -> Chain<T, U>
where
    T: AsyncRead,
    U: AsyncRead,
{
    panic!("STUB: not implemented");
}
impl<T, U> Chain<T, U>
where
    T: AsyncRead,
    U: AsyncRead,
{
    /// Gets references to the underlying readers in this `Chain`.
    pub fn get_ref(&self) -> (&T, &U) {
        panic!("STUB: not implemented");
    }
    /// Gets mutable references to the underlying readers in this `Chain`.
    ///
    /// Care should be taken to avoid modifying the internal I/O state of the
    /// underlying readers as doing so may corrupt the internal state of this
    /// `Chain`.
    pub fn get_mut(&mut self) -> (&mut T, &mut U) {
        panic!("STUB: not implemented");
    }
    /// Gets pinned mutable references to the underlying readers in this `Chain`.
    ///
    /// Care should be taken to avoid modifying the internal I/O state of the
    /// underlying readers as doing so may corrupt the internal state of this
    /// `Chain`.
    pub fn get_pin_mut(self: Pin<&mut Self>) -> (Pin<&mut T>, Pin<&mut U>) {
        panic!("STUB: not implemented");
    }
    /// Consumes the `Chain`, returning the wrapped readers.
    pub fn into_inner(self) -> (T, U) {
        panic!("STUB: not implemented");
    }
}
impl<T, U> fmt::Debug for Chain<T, U>
where
    T: fmt::Debug,
    U: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<T, U> AsyncRead for Chain<T, U>
where
    T: AsyncRead,
    U: AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
impl<T, U> AsyncBufRead for Chain<T, U>
where
    T: AsyncBufRead,
    U: AsyncBufRead,
{
    fn poll_fill_buf(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<&[u8]>> {
        panic!("STUB: not implemented");
    }
    fn consume(self: Pin<&mut Self>, amt: usize) {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assert_unpin() {
        crate::is_unpin::<Chain<(), ()>>();
    }
}
