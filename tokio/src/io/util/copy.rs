use crate::io::{AsyncRead, AsyncWrite, ReadBuf};
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
#[derive(Debug)]
pub(super) struct CopyBuffer {
    read_done: bool,
    need_flush: bool,
    pos: usize,
    cap: usize,
    amt: u64,
    buf: Box<[u8]>,
}
impl CopyBuffer {
    pub(super) fn new(buf_size: usize) -> Self {
        panic!("STUB: not implemented");
    }
    fn poll_fill_buf<R>(
        &mut self,
        cx: &mut Context<'_>,
        reader: Pin<&mut R>,
    ) -> Poll<io::Result<()>>
    where
        R: AsyncRead + ?Sized,
    {
        panic!("STUB: not implemented");
    }
    fn poll_write_buf<R, W>(
        &mut self,
        cx: &mut Context<'_>,
        mut reader: Pin<&mut R>,
        mut writer: Pin<&mut W>,
    ) -> Poll<io::Result<usize>>
    where
        R: AsyncRead + ?Sized,
        W: AsyncWrite + ?Sized,
    {
        panic!("STUB: not implemented");
    }
    pub(super) fn poll_copy<R, W>(
        &mut self,
        cx: &mut Context<'_>,
        mut reader: Pin<&mut R>,
        mut writer: Pin<&mut W>,
    ) -> Poll<io::Result<u64>>
    where
        R: AsyncRead + ?Sized,
        W: AsyncWrite + ?Sized,
    {
        panic!("STUB: not implemented");
    }
}
/// A future that asynchronously copies the entire contents of a reader into a
/// writer.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
struct Copy<'a, R: ?Sized, W: ?Sized> {
    reader: &'a mut R,
    writer: &'a mut W,
    buf: CopyBuffer,
}
cfg_io_util! {
    #[doc = " Asynchronously copies the entire contents of a reader into a writer."]
    #[doc = ""] #[doc =
    " This function returns a future that will continuously read data from"] #[doc =
    " `reader` and then write it into `writer` in a streaming fashion until"] #[doc =
    " `reader` returns EOF or fails."] #[doc = ""] #[doc =
    " On success, the total number of bytes that were copied from `reader` to"] #[doc =
    " `writer` is returned."] #[doc = ""] #[doc =
    " This is an asynchronous version of [`std::io::copy`][std]."] #[doc = ""] #[doc =
    " A heap-allocated copy buffer with 8 KB is created to take data from the"] #[doc =
    " reader to the writer, check [`copy_buf`] if you want an alternative for"] #[doc =
    " [`AsyncBufRead`]. You can use `copy_buf` with [`BufReader`] to change the"] #[doc =
    " buffer capacity."] #[doc = ""] #[doc =
    " # When to use async alternatives instead of `SyncIoBridge`"] #[doc = ""] #[doc =
    " If you are looking to use [`std::io::copy`] with a synchronous consumer"] #[doc =
    " (like a `hasher` or compressor), consider using async alternatives instead of"]
    #[doc = " wrapping the reader with [`SyncIoBridge`]."] #[doc =
    " See the [`SyncIoBridge`] documentation for detailed examples and guidance."] #[doc
    = ""] #[doc = " [std]: std::io::copy"] #[doc = " [`copy_buf`]: crate::io::copy_buf"]
    #[doc = " [`AsyncBufRead`]: crate::io::AsyncBufRead"] #[doc =
    " [`BufReader`]: crate::io::BufReader"] #[doc =
    " [`SyncIoBridge`]: https://docs.rs/tokio-util/latest/tokio_util/io/struct.SyncIoBridge.html"]
    #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc =
    " The returned future will return an error immediately if any call to"] #[doc =
    " `poll_read` or `poll_write` returns an error."] #[doc = ""] #[doc = " # Examples"]
    #[doc = ""] #[doc = " ```"] #[doc = " use tokio::io;"] #[doc = ""] #[doc =
    " # async fn dox() -> std::io::Result<()> {"] #[doc =
    " let mut reader: &[u8] = b\"hello\";"] #[doc = " let mut writer: Vec<u8> = vec![];"]
    #[doc = ""] #[doc = " io::copy(&mut reader, &mut writer).await?;"] #[doc = ""] #[doc
    = " assert_eq!(&b\"hello\"[..], &writer[..]);"] #[doc = " # Ok(())"] #[doc = " # }"]
    #[doc = " ```"] pub async fn copy <'a, R, W > (reader : &'a mut R, writer : &'a mut
    W) -> io::Result < u64 > where R : AsyncRead + Unpin + ? Sized, W : AsyncWrite +
    Unpin + ? Sized, { Copy { reader, writer, buf :
    CopyBuffer::new(super::DEFAULT_BUF_SIZE) } . await }
}
impl<R, W> Future for Copy<'_, R, W>
where
    R: AsyncRead + Unpin + ?Sized,
    W: AsyncWrite + Unpin + ?Sized,
{
    type Output = io::Result<u64>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<u64>> {
        panic!("STUB: not implemented");
    }
}
