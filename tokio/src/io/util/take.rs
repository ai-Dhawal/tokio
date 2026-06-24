use crate::io::{AsyncBufRead, AsyncRead, ReadBuf};
use pin_project_lite::pin_project;
use std::convert::TryFrom;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use std::{cmp, io};
pin_project! {
    #[doc = " Stream for the [`take`](super::AsyncReadExt::take) method."]
    #[derive(Debug)] #[must_use = "streams do nothing unless you `.await` or poll them"]
    #[cfg_attr(docsrs, doc(cfg(feature = "io-util")))] pub struct Take < R > { #[pin]
    inner : R, limit_ : u64, }
}
pub(super) fn take<R: AsyncRead>(inner: R, limit: u64) -> Take<R> {
    panic!("STUB: not implemented");
}
impl<R: AsyncRead> Take<R> {
    /// Returns the remaining number of bytes that can be
    /// read before this instance will return EOF.
    ///
    /// # Note
    ///
    /// This instance may reach `EOF` after reading fewer bytes than indicated by
    /// this method if the underlying [`AsyncRead`] instance reaches EOF.
    pub fn limit(&self) -> u64 {
        panic!("STUB: not implemented");
    }
    /// Sets the number of bytes that can be read before this instance will
    /// return EOF. This is the same as constructing a new `Take` instance, so
    /// the amount of bytes read and the previous limit value don't matter when
    /// calling this method.
    pub fn set_limit(&mut self, limit: u64) {
        panic!("STUB: not implemented");
    }
    /// Gets a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        panic!("STUB: not implemented");
    }
    /// Gets a mutable reference to the underlying reader.
    ///
    /// Care should be taken to avoid modifying the internal I/O state of the
    /// underlying reader as doing so may corrupt the internal limit of this
    /// `Take`.
    pub fn get_mut(&mut self) -> &mut R {
        panic!("STUB: not implemented");
    }
    /// Gets a pinned mutable reference to the underlying reader.
    ///
    /// Care should be taken to avoid modifying the internal I/O state of the
    /// underlying reader as doing so may corrupt the internal limit of this
    /// `Take`.
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut R> {
        panic!("STUB: not implemented");
    }
    /// Consumes the `Take`, returning the wrapped reader.
    pub fn into_inner(self) -> R {
        panic!("STUB: not implemented");
    }
}
impl<R: AsyncRead> AsyncRead for Take<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), io::Error>> {
        panic!("STUB: not implemented");
    }
}
impl<R: AsyncBufRead> AsyncBufRead for Take<R> {
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
        crate::is_unpin::<Take<()>>();
    }
}
