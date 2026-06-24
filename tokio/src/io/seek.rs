use crate::io::AsyncSeek;
use pin_project_lite::pin_project;
use std::future::Future;
use std::io::{self, SeekFrom};
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
pin_project! {
    #[doc = " Future for the [`seek`](crate::io::AsyncSeekExt::seek) method."]
    #[derive(Debug)] #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct Seek <'a, S : ? Sized > { seek : &'a mut S, pos : Option < SeekFrom >,
    #[pin] _pin : PhantomPinned, }
}
pub(crate) fn seek<S>(seek: &mut S, pos: SeekFrom) -> Seek<'_, S>
where
    S: AsyncSeek + ?Sized + Unpin,
{
    panic!("STUB: not implemented");
}
impl<S> Future for Seek<'_, S>
where
    S: AsyncSeek + ?Sized + Unpin,
{
    type Output = io::Result<u64>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
