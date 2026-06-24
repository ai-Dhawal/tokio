use crate::io::AsyncRead;
use bytes::BufMut;
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
pub(crate) fn read_buf<'a, R, B>(reader: &'a mut R, buf: &'a mut B) -> ReadBuf<'a, R, B>
where
    R: AsyncRead + Unpin + ?Sized,
    B: BufMut + ?Sized,
{
    panic!("STUB: not implemented");
}
pin_project! {
    #[doc = " Future returned by [`read_buf`](crate::io::AsyncReadExt::read_buf)."]
    #[derive(Debug)] #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadBuf <'a, R : ? Sized, B : ? Sized > { reader : &'a mut R, buf : &'a
    mut B, #[pin] _pin : PhantomPinned, }
}
impl<R, B> Future for ReadBuf<'_, R, B>
where
    R: AsyncRead + Unpin + ?Sized,
    B: BufMut + ?Sized,
{
    type Output = io::Result<usize>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
}
