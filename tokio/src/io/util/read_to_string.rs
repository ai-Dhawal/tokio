use crate::io::util::read_line::finish_string_read;
use crate::io::util::read_to_end::read_to_end_internal;
use crate::io::util::vec_with_initialized::VecWithInitialized;
use crate::io::AsyncRead;
use pin_project_lite::pin_project;
use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use std::{io, mem};
pin_project! {
    #[doc =
    " Future for the [`read_to_string`](super::AsyncReadExt::read_to_string) method."]
    #[derive(Debug)] #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadToString <'a, R : ? Sized > { reader : &'a mut R, output : &'a mut
    String, buf : VecWithInitialized < Vec < u8 >>, read : usize, #[pin] _pin :
    PhantomPinned, }
}
pub(crate) fn read_to_string<'a, R>(
    reader: &'a mut R,
    string: &'a mut String,
) -> ReadToString<'a, R>
where
    R: AsyncRead + ?Sized + Unpin,
{
    panic!("STUB: not implemented");
}
fn read_to_string_internal<R: AsyncRead + ?Sized>(
    reader: Pin<&mut R>,
    output: &mut String,
    buf: &mut VecWithInitialized<Vec<u8>>,
    read: &mut usize,
    cx: &mut Context<'_>,
) -> Poll<io::Result<usize>> {
    panic!("STUB: not implemented");
}
impl<A> Future for ReadToString<'_, A>
where
    A: AsyncRead + ?Sized + Unpin,
{
    type Output = io::Result<usize>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
