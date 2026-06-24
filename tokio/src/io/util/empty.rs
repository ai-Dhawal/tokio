use crate::io::util::poll_proceed_and_make_progress;
use crate::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, ReadBuf};
use std::fmt;
use std::io::{self, SeekFrom};
use std::pin::Pin;
use std::task::{ready, Context, Poll};
cfg_io_util! {
    #[doc =
    " `Empty` ignores any data written via [`AsyncWrite`], and will always be empty"]
    #[doc = " (returning zero bytes) when read via [`AsyncRead`]."] #[doc = ""] #[doc =
    " This struct is generally created by calling [`empty`]. Please see"] #[doc =
    " the documentation of [`empty()`][`empty`] for more details."] #[doc = ""] #[doc =
    " This is an asynchronous version of [`std::io::empty`][std]."] #[doc = ""] #[doc =
    " [`empty`]: fn@empty"] #[doc = " [std]: std::io::empty"] pub struct Empty { _p : (),
    } #[doc =
    " Creates a value that is always at EOF for reads, and ignores all data written."]
    #[doc = ""] #[doc =
    " All writes on the returned instance will return `Poll::Ready(Ok(buf.len()))`"]
    #[doc = " and the contents of the buffer will not be inspected."] #[doc = ""] #[doc =
    " All reads from the returned instance will return `Poll::Ready(Ok(0))`."] #[doc =
    ""] #[doc = " This is an asynchronous version of [`std::io::empty`][std]."] #[doc =
    ""] #[doc = " [std]: std::io::empty"] #[doc = ""] #[doc = " # Examples"] #[doc = ""]
    #[doc = " A slightly sad example of not reading anything into a buffer:"] #[doc = ""]
    #[doc = " ```"] #[doc = " use tokio::io::{self, AsyncReadExt};"] #[doc = ""] #[doc =
    " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
    #[doc = "     let mut buffer = String::new();"] #[doc =
    "     io::empty().read_to_string(&mut buffer).await.unwrap();"] #[doc =
    "     assert!(buffer.is_empty());"] #[doc = " # }"] #[doc = " ```"] #[doc = ""] #[doc
    = " A convoluted way of getting the length of a buffer:"] #[doc = ""] #[doc = " ```"]
    #[doc = " use tokio::io::{self, AsyncWriteExt};"] #[doc = ""] #[doc =
    " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
    #[doc = " let buffer = vec![1, 2, 3, 5, 8];"] #[doc =
    " let num_bytes = io::empty().write(&buffer).await.unwrap();"] #[doc =
    " assert_eq!(num_bytes, 5);"] #[doc = " # }"] #[doc = " ```"] pub fn empty() -> Empty
    { Empty { _p : () } }
}
impl AsyncRead for Empty {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        _: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
impl AsyncBufRead for Empty {
    #[inline]
    fn poll_fill_buf(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<&[u8]>> {
        panic!("STUB: not implemented");
    }
    #[inline]
    fn consume(self: Pin<&mut Self>, _: usize) {
        panic!("STUB: not implemented");
    }
}
impl AsyncWrite for Empty {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
    #[inline]
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        panic!("STUB: not implemented");
    }
    #[inline]
    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        panic!("STUB: not implemented");
    }
    #[inline]
    fn is_write_vectored(&self) -> bool {
        panic!("STUB: not implemented");
    }
    #[inline]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<Result<usize, io::Error>> {
        panic!("STUB: not implemented");
    }
}
impl AsyncSeek for Empty {
    #[inline]
    fn start_seek(self: Pin<&mut Self>, _position: SeekFrom) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    #[inline]
    fn poll_complete(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<u64>> {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for Empty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assert_unpin() {
        crate::is_unpin::<Empty>();
    }
}
