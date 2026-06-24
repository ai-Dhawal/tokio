use crate::io::util::vec_with_initialized::{
    into_read_buf_parts, VecU8, VecWithInitialized,
};
use crate::io::{AsyncRead, ReadBuf};
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::marker::PhantomPinned;
use std::mem::{self, MaybeUninit};
use std::pin::Pin;
use std::task::{ready, Context, Poll};
pin_project! {
    #[derive(Debug)] #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadToEnd <'a, R : ? Sized > { reader : &'a mut R, buf :
    VecWithInitialized <&'a mut Vec < u8 >>, read : usize, #[pin] _pin : PhantomPinned, }
}
pub(crate) fn read_to_end<'a, R>(
    reader: &'a mut R,
    buffer: &'a mut Vec<u8>,
) -> ReadToEnd<'a, R>
where
    R: AsyncRead + Unpin + ?Sized,
{
    panic!("STUB: not implemented");
}
pub(super) fn read_to_end_internal<V: VecU8, R: AsyncRead + ?Sized>(
    buf: &mut VecWithInitialized<V>,
    mut reader: Pin<&mut R>,
    num_read: &mut usize,
    cx: &mut Context<'_>,
) -> Poll<io::Result<usize>> {
    panic!("STUB: not implemented");
}
/// Tries to read from the provided [`AsyncRead`].
///
/// The length of the buffer is increased by the number of bytes read.
fn poll_read_to_end<V: VecU8, R: AsyncRead + ?Sized>(
    buf: &mut VecWithInitialized<V>,
    read: Pin<&mut R>,
    cx: &mut Context<'_>,
) -> Poll<io::Result<usize>> {
    panic!("STUB: not implemented");
}
impl<A> Future for ReadToEnd<'_, A>
where
    A: AsyncRead + ?Sized + Unpin,
{
    type Output = io::Result<usize>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
