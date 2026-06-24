use crate::io::{AsyncRead, ReadBuf};
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::marker::PhantomPinned;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
/// A future which can be used to easily read exactly enough bytes to fill
/// a buffer.
///
/// Created by the [`AsyncReadExt::read_exact`][read_exact].
/// [`read_exact`]: [`crate::io::AsyncReadExt::read_exact`]
pub(crate) fn read_exact<'a, A>(reader: &'a mut A, buf: &'a mut [u8]) -> ReadExact<'a, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    panic!("STUB: not implemented");
}
pin_project! {
    #[doc = " Creates a future which will read exactly enough bytes to fill `buf`,"]
    #[doc = " returning an error if EOF is hit sooner."] #[doc = ""] #[doc =
    " On success the number of bytes is returned"] #[derive(Debug)] #[must_use =
    "futures do nothing unless you `.await` or poll them"] pub struct ReadExact <'a, A :
    ? Sized > { reader : &'a mut A, buf : ReadBuf <'a >, #[pin] _pin : PhantomPinned, }
}
fn eof() -> io::Error {
    panic!("STUB: not implemented");
}
impl<A> Future for ReadExact<'_, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    type Output = io::Result<usize>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
}
