use crate::io::AsyncBufRead;
use crate::util::memchr;
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::marker::PhantomPinned;
use std::mem;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
pin_project! {
    #[doc =
    " Future for the [`read_until`](crate::io::AsyncBufReadExt::read_until) method."]
    #[doc = " The delimiter is included in the resulting vector."] #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"] pub struct
    ReadUntil <'a, R : ? Sized > { reader : &'a mut R, delimiter : u8, buf : &'a mut Vec
    < u8 >, read : usize, #[pin] _pin : PhantomPinned, }
}
pub(crate) fn read_until<'a, R>(
    reader: &'a mut R,
    delimiter: u8,
    buf: &'a mut Vec<u8>,
) -> ReadUntil<'a, R>
where
    R: AsyncBufRead + ?Sized + Unpin,
{
    panic!("STUB: not implemented");
}
pub(super) fn read_until_internal<R: AsyncBufRead + ?Sized>(
    mut reader: Pin<&mut R>,
    cx: &mut Context<'_>,
    delimiter: u8,
    buf: &mut Vec<u8>,
    read: &mut usize,
) -> Poll<io::Result<usize>> {
    panic!("STUB: not implemented");
}
impl<R: AsyncBufRead + ?Sized + Unpin> Future for ReadUntil<'_, R> {
    type Output = io::Result<usize>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
