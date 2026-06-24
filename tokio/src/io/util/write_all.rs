use crate::io::AsyncWrite;
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::marker::PhantomPinned;
use std::mem;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
pin_project! {
    #[derive(Debug)] #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct WriteAll <'a, W : ? Sized > { writer : &'a mut W, buf : &'a[u8], #[pin]
    _pin : PhantomPinned, }
}
pub(crate) fn write_all<'a, W>(writer: &'a mut W, buf: &'a [u8]) -> WriteAll<'a, W>
where
    W: AsyncWrite + Unpin + ?Sized,
{
    panic!("STUB: not implemented");
}
impl<W> Future for WriteAll<'_, W>
where
    W: AsyncWrite + Unpin + ?Sized,
{
    type Output = io::Result<()>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
