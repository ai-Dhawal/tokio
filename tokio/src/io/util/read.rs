use crate::io::{AsyncRead, ReadBuf};
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::marker::PhantomPinned;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
/// Tries to read some bytes directly into the given `buf` in asynchronous
/// manner, returning a future type.
///
/// The returned future will resolve to both the I/O stream and the buffer
/// as well as the number of bytes read once the read operation is completed.
pub(crate) fn read<'a, R>(reader: &'a mut R, buf: &'a mut [u8]) -> Read<'a, R>
where
    R: AsyncRead + Unpin + ?Sized,
{
    panic!("STUB: not implemented");
}
pin_project! {
    #[doc =
    " A future which can be used to easily read available number of bytes to fill"] #[doc
    = " a buffer."] #[doc = ""] #[doc = " Created by the [`read`] function."]
    #[derive(Debug)] #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct Read <'a, R : ? Sized > { reader : &'a mut R, buf : &'a mut [u8], #[pin]
    _pin : PhantomPinned, }
}
impl<R> Future for Read<'_, R>
where
    R: AsyncRead + Unpin + ?Sized,
{
    type Output = io::Result<usize>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
}
