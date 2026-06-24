use crate::io::AsyncWrite;
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};
pin_project! {
    #[doc = " A future used to shutdown an I/O object."] #[doc = ""] #[doc =
    " Created by the [`AsyncWriteExt::shutdown`][shutdown] function."] #[doc =
    " [shutdown]: [`crate::io::AsyncWriteExt::shutdown`]"] #[must_use =
    "futures do nothing unless you `.await` or poll them"] #[derive(Debug)] pub struct
    Shutdown <'a, A : ? Sized > { a : &'a mut A, #[pin] _pin : PhantomPinned, }
}
/// Creates a future which will shutdown an I/O object.
pub(super) fn shutdown<A>(a: &mut A) -> Shutdown<'_, A>
where
    A: AsyncWrite + Unpin + ?Sized,
{
    panic!("STUB: not implemented");
}
impl<A> Future for Shutdown<'_, A>
where
    A: AsyncWrite + Unpin + ?Sized,
{
    type Output = io::Result<()>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
