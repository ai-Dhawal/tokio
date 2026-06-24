use crate::io::util::DEFAULT_BUF_SIZE;
use crate::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, ReadBuf};
use pin_project_lite::pin_project;
use std::io::{self, IoSlice, SeekFrom};
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use std::{cmp, fmt, mem};
pin_project! {
    #[doc = " The `BufReader` struct adds buffering to any reader."] #[doc = ""] #[doc =
    " It can be excessively inefficient to work directly with a [`AsyncRead`]"] #[doc =
    " instance. A `BufReader` performs large, infrequent reads on the underlying"] #[doc
    = " [`AsyncRead`] and maintains an in-memory buffer of the results."] #[doc = ""]
    #[doc = " `BufReader` can improve the speed of programs that make *small* and"] #[doc
    = " *repeated* read calls to the same file or network socket. It does not"] #[doc =
    " help when reading very large amounts at once, or reading just one or a few"] #[doc
    = " times. It also provides no advantage when reading from a source that is"] #[doc =
    " already in memory, like a `Vec<u8>`."] #[doc = ""] #[doc =
    " When the `BufReader` is dropped, the contents of its buffer will be"] #[doc =
    " discarded. Creating multiple instances of a `BufReader` on the same"] #[doc =
    " stream can cause data loss."] #[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
    pub struct BufReader < R > { #[pin] pub (super) inner : R, pub (super) buf : Box <
    [u8] >, pub (super) pos : usize, pub (super) cap : usize, pub (super) seek_state :
    SeekState, }
}
impl<R: AsyncRead> BufReader<R> {
    /// Creates a new `BufReader` with a default buffer capacity. The default is currently 8 KB,
    /// but may change in the future.
    pub fn new(inner: R) -> Self {
        panic!("STUB: not implemented");
    }
    /// Creates a new `BufReader` with the specified buffer capacity.
    pub fn with_capacity(capacity: usize, inner: R) -> Self {
        panic!("STUB: not implemented");
    }
    /// Gets a reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub fn get_ref(&self) -> &R {
        panic!("STUB: not implemented");
    }
    /// Gets a mutable reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        panic!("STUB: not implemented");
    }
    /// Gets a pinned mutable reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut R> {
        panic!("STUB: not implemented");
    }
    /// Consumes this `BufReader`, returning the underlying reader.
    ///
    /// Note that any leftover data in the internal buffer is lost.
    pub fn into_inner(self) -> R {
        panic!("STUB: not implemented");
    }
    /// Returns a reference to the internally buffered data.
    ///
    /// Unlike `fill_buf`, this will not attempt to fill the buffer if it is empty.
    pub fn buffer(&self) -> &[u8] {
        panic!("STUB: not implemented");
    }
    /// Invalidates all data in the internal buffer.
    #[inline]
    fn discard_buffer(self: Pin<&mut Self>) {
        panic!("STUB: not implemented");
    }
}
impl<R: AsyncRead> AsyncRead for BufReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
impl<R: AsyncRead> AsyncBufRead for BufReader<R> {
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
#[derive(Debug, Clone, Copy)]
pub(super) enum SeekState {
    /// `start_seek` has not been called.
    Init,
    /// `start_seek` has been called, but `poll_complete` has not yet been called.
    Start(SeekFrom),
    /// Waiting for completion of the first `poll_complete` in the `n.checked_sub(remainder).is_none()` branch.
    PendingOverflowed(i64),
    /// Waiting for completion of `poll_complete`.
    Pending,
}
/// Seeks to an offset, in bytes, in the underlying reader.
///
/// The position used for seeking with `SeekFrom::Current(_)` is the
/// position the underlying reader would be at if the `BufReader` had no
/// internal buffer.
///
/// Seeking always discards the internal buffer, even if the seek position
/// would otherwise fall within it. This guarantees that calling
/// `.into_inner()` immediately after a seek yields the underlying reader
/// at the same position.
///
/// See [`AsyncSeek`] for more details.
///
/// Note: In the edge case where you're seeking with `SeekFrom::Current(n)`
/// where `n` minus the internal buffer length overflows an `i64`, two
/// seeks will be performed instead of one. If the second seek returns
/// `Err`, the underlying reader will be left at the same position it would
/// have if you called `seek` with `SeekFrom::Current(0)`.
impl<R: AsyncRead + AsyncSeek> AsyncSeek for BufReader<R> {
    fn start_seek(self: Pin<&mut Self>, pos: SeekFrom) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    fn poll_complete(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<u64>> {
        panic!("STUB: not implemented");
    }
}
impl<R: AsyncRead + AsyncWrite> AsyncWrite for BufReader<R> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
    fn is_write_vectored(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
impl<R: fmt::Debug> fmt::Debug for BufReader<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assert_unpin() {
        crate::is_unpin::<BufReader<()>>();
    }
}
