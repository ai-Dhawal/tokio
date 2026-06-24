use crate::io::AsyncWrite;
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};
pin_project! {
    #[doc = " A future to write some of the buffer to an `AsyncWrite`."] #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"] pub struct Write
    <'a, W : ? Sized > { writer : &'a mut W, buf : &'a[u8], #[pin] _pin : PhantomPinned,
    }
}
/// Tries to write some bytes from the given `buf` to the writer in an
/// asynchronous manner, returning a future.
pub(crate) fn write<'a, W>(writer: &'a mut W, buf: &'a [u8]) -> Write<'a, W>
where
    W: AsyncWrite + Unpin + ?Sized,
{
    panic!("STUB: not implemented");
}
impl<W> Future for Write<'_, W>
where
    W: AsyncWrite + Unpin + ?Sized,
{
    type Output = io::Result<usize>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
}
