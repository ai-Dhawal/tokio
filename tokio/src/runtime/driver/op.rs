use crate::io::blocking::Buf;
use crate::io::uring::open::Open;
use crate::io::uring::read::Read;
use crate::io::uring::utils::ArcFd;
use crate::io::uring::write::Write;
use crate::runtime::Handle;
#[cfg(any(target_env = "gnu", target_os = "android"))]
use crate::io::uring::statx::Statx;
use io_uring::cqueue;
use io_uring::squeue::Entry;
use std::future::Future;
use std::io::{self, Error};
use std::mem;
use std::os::fd::OwnedFd;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum CancelData {
    Open(Open),
    Write(Write),
    ReadVec(Read<Vec<u8>, OwnedFd>),
    ReadBuf(Read<Buf, ArcFd>),
    #[cfg(any(target_env = "gnu", target_os = "android"))]
    Statx(Statx),
}
#[derive(Debug)]
pub(crate) enum Lifecycle {
    /// The operation has been submitted to uring and is currently in-flight
    Submitted,
    /// The submitter is waiting for the completion of the operation
    Waiting(Waker),
    /// The submitter no longer has interest in the operation result. The state
    /// must be passed to the driver and held until the operation completes.
    Cancelled(#[allow(dead_code)] CancelData),
    /// The operation has completed with a single cqe result
    Completed(io_uring::cqueue::Entry),
}
pub(crate) enum State {
    Initialize(Option<Entry>),
    Polled(usize),
    Complete,
}
pub(crate) struct Op<T: Cancellable> {
    handle: Handle,
    state: State,
    data: Option<T>,
}
impl<T: Cancellable> Op<T> {
    /// # Safety
    ///
    /// Callers must ensure that parameters of the entry (such as buffer) are valid and will
    /// be valid for the entire duration of the operation, otherwise it may cause memory problems.
    pub(crate) unsafe fn new(entry: Entry, data: T) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn take_data(&mut self) -> Option<T> {
        panic!("STUB: not implemented");
    }
}
impl<T: Cancellable> Drop for Op<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
/// A single CQE result
pub(crate) struct CqeResult {
    pub(crate) result: io::Result<u32>,
}
impl From<cqueue::Entry> for CqeResult {
    fn from(cqe: cqueue::Entry) -> Self {
        panic!("STUB: not implemented");
    }
}
/// A trait that converts a CQE result into a usable value for each operation.
pub(crate) trait Completable {
    type Output;
    fn complete(self, cqe: CqeResult) -> Self::Output;
    fn complete_with_error(self, error: Error) -> Self::Output;
}
/// Extracts the `CancelData` needed to safely cancel an in-flight io_uring operation.
pub(crate) trait Cancellable {
    fn cancel(self) -> CancelData;
}
impl<T: Cancellable> Unpin for Op<T> {}
impl<T: Cancellable + Completable + Send> Future for Op<T> {
    type Output = T::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
