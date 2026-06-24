use crate::io::AsyncBufRead;
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};
pin_project! {
    #[doc = " Future for the [`fill_buf`](crate::io::AsyncBufReadExt::fill_buf) method."]
    #[derive(Debug)] #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct FillBuf <'a, R : ? Sized > { reader : Option <&'a mut R >, #[pin] _pin :
    PhantomPinned, }
}
pub(crate) fn fill_buf<R>(reader: &mut R) -> FillBuf<'_, R>
where
    R: AsyncBufRead + ?Sized + Unpin,
{
    panic!("STUB: not implemented");
}
impl<'a, R: AsyncBufRead + ?Sized + Unpin> Future for FillBuf<'a, R> {
    type Output = io::Result<&'a [u8]>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
