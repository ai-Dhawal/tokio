use crate::io::util::DEFAULT_BUF_SIZE;
use crate::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, ReadBuf};
use pin_project_lite::pin_project;
use std::fmt;
use std::io::{self, IoSlice, SeekFrom, Write};
use std::pin::Pin;
use std::task::{ready, Context, Poll};
pin_project! {
    #[doc = " Wraps a writer and buffers its output."] #[doc = ""] #[doc =
    " It can be excessively inefficient to work directly with something that"] #[doc =
    " implements [`AsyncWrite`]. A `BufWriter` keeps an in-memory buffer of data and"]
    #[doc = " writes it to an underlying writer in large, infrequent batches."] #[doc =
    ""] #[doc = " `BufWriter` can improve the speed of programs that make *small* and"]
    #[doc = " *repeated* write calls to the same file or network socket. It does not"]
    #[doc =
    " help when writing very large amounts at once, or writing just one or a few"] #[doc
    = " times. It also provides no advantage when writing to a destination that is"]
    #[doc = " in memory, like a `Vec<u8>`."] #[doc = ""] #[doc =
    " When the `BufWriter` is dropped, the contents of its buffer will be"] #[doc =
    " discarded. Creating multiple instances of a `BufWriter` on the same"] #[doc =
    " stream can cause data loss. If you need to write out the contents of its"] #[doc =
    " buffer, you must manually call flush before the writer is dropped."] #[doc = ""]
    #[doc = " [`AsyncWrite`]: AsyncWrite"] #[doc =
    " [`flush`]: super::AsyncWriteExt::flush"] #[doc = ""] #[cfg_attr(docsrs,
    doc(cfg(feature = "io-util")))] pub struct BufWriter < W > { #[pin] pub (super) inner
    : W, pub (super) buf : Vec < u8 >, pub (super) written : usize, pub (super)
    seek_state : SeekState, }
}
impl<W: AsyncWrite> BufWriter<W> {
    /// Creates a new `BufWriter` with a default buffer capacity. The default is currently 8 KB,
    /// but may change in the future.
    pub fn new(inner: W) -> Self {
        panic!("STUB: not implemented");
    }
    /// Creates a new `BufWriter` with the specified buffer capacity.
    pub fn with_capacity(cap: usize, inner: W) -> Self {
        panic!("STUB: not implemented");
    }
    fn flush_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
    /// Gets a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        panic!("STUB: not implemented");
    }
    /// Gets a mutable reference to the underlying writer.
    ///
    /// It is inadvisable to directly write to the underlying writer.
    pub fn get_mut(&mut self) -> &mut W {
        panic!("STUB: not implemented");
    }
    /// Gets a pinned mutable reference to the underlying writer.
    ///
    /// It is inadvisable to directly write to the underlying writer.
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut W> {
        panic!("STUB: not implemented");
    }
    /// Consumes this `BufWriter`, returning the underlying writer.
    ///
    /// Note that any leftover data in the internal buffer is lost.
    pub fn into_inner(self) -> W {
        panic!("STUB: not implemented");
    }
    /// Returns a reference to the internally buffered data.
    pub fn buffer(&self) -> &[u8] {
        panic!("STUB: not implemented");
    }
}
impl<W: AsyncWrite> AsyncWrite for BufWriter<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut bufs: &[IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
    fn is_write_vectored(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) enum SeekState {
    /// `start_seek` has not been called.
    Init,
    /// `start_seek` has been called, but `poll_complete` has not yet been called.
    Start(SeekFrom),
    /// Waiting for completion of `poll_complete`.
    Pending,
}
/// Seek to the offset, in bytes, in the underlying writer.
///
/// Seeking always writes out the internal buffer before seeking.
impl<W: AsyncWrite + AsyncSeek> AsyncSeek for BufWriter<W> {
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
impl<W: AsyncWrite + AsyncRead> AsyncRead for BufWriter<W> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
impl<W: AsyncWrite + AsyncBufRead> AsyncBufRead for BufWriter<W> {
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
impl<W: fmt::Debug> fmt::Debug for BufWriter<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assert_unpin() {
        crate::is_unpin::<BufWriter<()>>();
    }
}
