//! Unix handling of child processes.
//!
//! Right now the only "fancy" thing about this is how we implement the
//! `Future` implementation on `Child` to get the exit status. Unix offers
//! no way to register a child with epoll, and the only real way to get a
//! notification when a process exits is the SIGCHLD signal.
//!
//! Signal handling in general is *super* hairy and complicated, and it's even
//! more complicated here with the fact that signals are coalesced, so we may
//! not get a SIGCHLD-per-child.
//!
//! Our best approximation here is to check *all spawned processes* for all
//! SIGCHLD signals received. To do that we create a `Signal`, implemented in
//! the `tokio-net` crate, which is a stream over signals being received.
//!
//! Later when we poll the process's exit status we simply check to see if a
//! SIGCHLD has happened since we last checked, and while that returns "yes" we
//! keep trying.
//!
//! Note that this means that this isn't really scalable, but then again
//! processes in general aren't scalable (e.g. millions) so it shouldn't be that
//! bad in theory...
pub(crate) mod orphan;
use orphan::{OrphanQueue, OrphanQueueImpl, Wait};
mod reap;
use reap::Reaper;
#[cfg(all(target_os = "linux", feature = "rt"))]
mod pidfd_reaper;
use crate::io::{AsyncRead, AsyncWrite, PollEvented, ReadBuf};
use crate::process::kill::Kill;
use crate::process::SpawnedChild;
use crate::runtime::signal::Handle as SignalHandle;
use crate::signal::unix::{signal, Signal, SignalKind};
use mio::event::Source;
use mio::unix::SourceFd;
use std::fmt;
use std::fs::File;
use std::future::Future;
use std::io;
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use std::pin::Pin;
use std::process::{Child as StdChild, ExitStatus, Stdio};
use std::task::Context;
use std::task::Poll;
impl Wait for StdChild {
    fn id(&self) -> u32 {
        panic!("STUB: not implemented");
    }
    fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        panic!("STUB: not implemented");
    }
}
impl Kill for StdChild {
    fn kill(&mut self) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
}
cfg_not_has_const_mutex_new! {
    fn get_orphan_queue() -> &'static OrphanQueueImpl < StdChild > { use
    std::sync::OnceLock; static ORPHAN_QUEUE : OnceLock < OrphanQueueImpl < StdChild >> =
    OnceLock::new(); ORPHAN_QUEUE.get_or_init(OrphanQueueImpl::new) }
}
cfg_has_const_mutex_new! {
    fn get_orphan_queue() -> &'static OrphanQueueImpl < StdChild > { static ORPHAN_QUEUE
    : OrphanQueueImpl < StdChild > = OrphanQueueImpl::new(); & ORPHAN_QUEUE }
}
pub(crate) struct GlobalOrphanQueue;
impl fmt::Debug for GlobalOrphanQueue {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl GlobalOrphanQueue {
    pub(crate) fn reap_orphans(handle: &SignalHandle) {
        panic!("STUB: not implemented");
    }
}
impl OrphanQueue<StdChild> for GlobalOrphanQueue {
    fn push_orphan(&self, orphan: StdChild) {
        panic!("STUB: not implemented");
    }
}
#[must_use = "futures do nothing unless polled"]
pub(crate) enum Child {
    SignalReaper(Reaper<StdChild, GlobalOrphanQueue, Signal>),
    #[cfg(all(target_os = "linux", feature = "rt"))]
    PidfdReaper(pidfd_reaper::PidfdReaper<StdChild, GlobalOrphanQueue>),
}
impl fmt::Debug for Child {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
pub(crate) fn build_child(mut child: StdChild) -> io::Result<SpawnedChild> {
    panic!("STUB: not implemented");
}
impl Child {
    pub(crate) fn id(&self) -> u32 {
        panic!("STUB: not implemented");
    }
    fn std_child(&mut self) -> &mut StdChild {
        panic!("STUB: not implemented");
    }
    pub(crate) fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        panic!("STUB: not implemented");
    }
}
impl Kill for Child {
    fn kill(&mut self) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
}
impl Future for Child {
    type Output = io::Result<ExitStatus>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug)]
pub(crate) struct Pipe {
    fd: File,
}
impl<T: IntoRawFd> From<T> for Pipe {
    fn from(fd: T) -> Self {
        panic!("STUB: not implemented");
    }
}
impl io::Read for &Pipe {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
}
impl io::Write for &Pipe {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
    fn flush(&mut self) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
}
impl AsRawFd for Pipe {
    fn as_raw_fd(&self) -> RawFd {
        panic!("STUB: not implemented");
    }
}
impl AsFd for Pipe {
    fn as_fd(&self) -> BorrowedFd<'_> {
        panic!("STUB: not implemented");
    }
}
fn convert_to_blocking_file(io: ChildStdio) -> io::Result<File> {
    panic!("STUB: not implemented");
}
pub(crate) fn convert_to_stdio(io: ChildStdio) -> io::Result<Stdio> {
    panic!("STUB: not implemented");
}
impl Source for Pipe {
    fn register(
        &mut self,
        registry: &mio::Registry,
        token: mio::Token,
        interest: mio::Interest,
    ) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    fn reregister(
        &mut self,
        registry: &mio::Registry,
        token: mio::Token,
        interest: mio::Interest,
    ) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    fn deregister(&mut self, registry: &mio::Registry) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
}
pub(crate) struct ChildStdio {
    inner: PollEvented<Pipe>,
}
impl ChildStdio {
    pub(super) fn into_owned_fd(self) -> io::Result<OwnedFd> {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for ChildStdio {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl AsRawFd for ChildStdio {
    fn as_raw_fd(&self) -> RawFd {
        panic!("STUB: not implemented");
    }
}
impl AsFd for ChildStdio {
    fn as_fd(&self) -> BorrowedFd<'_> {
        panic!("STUB: not implemented");
    }
}
impl AsyncWrite for ChildStdio {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<Result<usize, io::Error>> {
        panic!("STUB: not implemented");
    }
    fn is_write_vectored(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
impl AsyncRead for ChildStdio {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
fn set_nonblocking<T: AsRawFd>(fd: &mut T, nonblocking: bool) -> io::Result<()> {
    panic!("STUB: not implemented");
}
pub(super) fn stdio<T>(io: T) -> io::Result<ChildStdio>
where
    T: IntoRawFd,
{
    panic!("STUB: not implemented");
}
