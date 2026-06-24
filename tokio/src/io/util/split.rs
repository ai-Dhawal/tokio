use crate::io::util::read_until::read_until_internal;
use crate::io::AsyncBufRead;
use pin_project_lite::pin_project;
use std::io;
use std::mem;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
pin_project! {
    #[doc = " Splitter for the [`split`](crate::io::AsyncBufReadExt::split) method."]
    #[doc = ""] #[doc = " A `Split` can be turned into a `Stream` with [`SplitStream`]."]
    #[doc = ""] #[doc =
    " [`SplitStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.SplitStream.html"]
    #[derive(Debug)] #[must_use = "streams do nothing unless polled"] #[cfg_attr(docsrs,
    doc(cfg(feature = "io-util")))] pub struct Split < R > { #[pin] reader : R, buf : Vec
    < u8 >, delim : u8, read : usize, }
}
pub(crate) fn split<R>(reader: R, delim: u8) -> Split<R>
where
    R: AsyncBufRead,
{
    panic!("STUB: not implemented");
}
impl<R> Split<R>
where
    R: AsyncBufRead + Unpin,
{
    /// Returns the next segment in the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::AsyncBufRead;
    /// use tokio::io::AsyncBufReadExt;
    ///
    /// # async fn dox(my_buf_read: impl AsyncBufRead + Unpin) -> std::io::Result<()> {
    /// let mut segments = my_buf_read.split(b'f');
    ///
    /// while let Some(segment) = segments.next_segment().await? {
    ///     println!("length = {}", segment.len())
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn next_segment(&mut self) -> io::Result<Option<Vec<u8>>> {
        panic!("STUB: not implemented");
    }
}
impl<R> Split<R>
where
    R: AsyncBufRead,
{
    /// Polls for the next segment in the stream.
    ///
    /// This method returns:
    ///
    ///  * `Poll::Pending` if the next segment is not yet available.
    ///  * `Poll::Ready(Ok(Some(segment)))` if the next segment is available.
    ///  * `Poll::Ready(Ok(None))` if there are no more segments in this stream.
    ///  * `Poll::Ready(Err(err))` if an IO error occurred while reading the
    ///    next segment.
    ///
    /// When the method returns `Poll::Pending`, the `Waker` in the provided
    /// `Context` is scheduled to receive a wakeup when more bytes become
    /// available on the underlying IO resource.
    ///
    /// Note that on multiple calls to `poll_next_segment`, only the `Waker`
    /// from the `Context` passed to the most recent call is scheduled to
    /// receive a wakeup.
    pub fn poll_next_segment(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<Option<Vec<u8>>>> {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assert_unpin() {
        crate::is_unpin::<Split<()>>();
    }
}
