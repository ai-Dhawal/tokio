use crate::future::maybe_done::{maybe_done, MaybeDone};
use pin_project_lite::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
pub(crate) fn try_join3<T1, F1, T2, F2, T3, F3, E>(
    future1: F1,
    future2: F2,
    future3: F3,
) -> TryJoin3<F1, F2, F3>
where
    F1: Future<Output = Result<T1, E>>,
    F2: Future<Output = Result<T2, E>>,
    F3: Future<Output = Result<T3, E>>,
{
    panic!("STUB: not implemented");
}
pin_project! {
    pub (crate) struct TryJoin3 < F1, F2, F3 > where F1 : Future, F2 : Future, F3 :
    Future, { #[pin] future1 : MaybeDone < F1 >, #[pin] future2 : MaybeDone < F2 >,
    #[pin] future3 : MaybeDone < F3 >, }
}
impl<T1, F1, T2, F2, T3, F3, E> Future for TryJoin3<F1, F2, F3>
where
    F1: Future<Output = Result<T1, E>>,
    F2: Future<Output = Result<T2, E>>,
    F3: Future<Output = Result<T3, E>>,
{
    type Output = Result<(T1, T2, T3), E>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
