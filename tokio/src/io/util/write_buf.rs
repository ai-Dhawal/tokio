use crate::io::AsyncWrite;
use bytes::Buf;
use pin_project_lite::pin_project;
use std::future::Future;
use std::io::{self, IoSlice};
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
pin_project! {
    #[doc = " A future to write some of the buffer to an `AsyncWrite`."] #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"] pub struct
    WriteBuf <'a, W, B > { writer : &'a mut W, buf : &'a mut B, #[pin] _pin :
    PhantomPinned, }
}
/// Tries to write some bytes from the given `buf` to the writer in an
/// asynchronous manner, returning a future.
pub(crate) fn write_buf<'a, W, B>(
    writer: &'a mut W,
    buf: &'a mut B,
) -> WriteBuf<'a, W, B>
where
    W: AsyncWrite + Unpin,
    B: Buf,
{
    panic!("STUB: not implemented");
}
impl<W, B> Future for WriteBuf<'_, W, B>
where
    W: AsyncWrite + Unpin,
    B: Buf,
{
    type Output = io::Result<usize>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
}
