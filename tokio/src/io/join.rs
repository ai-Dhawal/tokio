//! Join two values implementing `AsyncRead` and `AsyncWrite` into a single one.
use crate::io::{AsyncBufRead, AsyncRead, AsyncWrite, ReadBuf};
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
/// Join two values implementing `AsyncRead` and `AsyncWrite` into a
/// single handle.
pub fn join<R, W>(reader: R, writer: W) -> Join<R, W>
where
    R: AsyncRead,
    W: AsyncWrite,
{
    panic!("STUB: not implemented");
}
pin_project_lite::pin_project! {
    #[doc = " Joins two values implementing `AsyncRead` and `AsyncWrite` into a"] #[doc =
    " single handle."] #[derive(Debug)] pub struct Join < R, W > { #[pin] reader : R,
    #[pin] writer : W, }
}
impl<R, W> Join<R, W>
where
    R: AsyncRead,
    W: AsyncWrite,
{
    /// Splits this `Join` back into its `AsyncRead` and `AsyncWrite`
    /// components.
    pub fn into_inner(self) -> (R, W) {
        panic!("STUB: not implemented");
    }
    /// Returns a reference to the inner reader.
    pub fn reader(&self) -> &R {
        panic!("STUB: not implemented");
    }
    /// Returns a reference to the inner writer.
    pub fn writer(&self) -> &W {
        panic!("STUB: not implemented");
    }
    /// Returns a mutable reference to the inner reader.
    pub fn reader_mut(&mut self) -> &mut R {
        panic!("STUB: not implemented");
    }
    /// Returns a mutable reference to the inner writer.
    pub fn writer_mut(&mut self) -> &mut W {
        panic!("STUB: not implemented");
    }
    /// Returns a pinned mutable reference to the inner reader.
    pub fn reader_pin_mut(self: Pin<&mut Self>) -> Pin<&mut R> {
        panic!("STUB: not implemented");
    }
    /// Returns a pinned mutable reference to the inner writer.
    pub fn writer_pin_mut(self: Pin<&mut Self>) -> Pin<&mut W> {
        panic!("STUB: not implemented");
    }
}
impl<R, W> AsyncRead for Join<R, W>
where
    R: AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), io::Error>> {
        panic!("STUB: not implemented");
    }
}
impl<R, W> AsyncWrite for Join<R, W>
where
    W: AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        panic!("STUB: not implemented");
    }
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        panic!("STUB: not implemented");
    }
    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        panic!("STUB: not implemented");
    }
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<Result<usize, io::Error>> {
        panic!("STUB: not implemented");
    }
    fn is_write_vectored(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
impl<R, W> AsyncBufRead for Join<R, W>
where
    R: AsyncBufRead,
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
