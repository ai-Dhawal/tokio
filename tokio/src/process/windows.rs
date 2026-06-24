//! Windows asynchronous process handling.
//!
//! Like with Unix we don't actually have a way of registering a process with an
//! IOCP object. As a result we similarly need another mechanism for getting a
//! signal when a process has exited. For now this is implemented with the
//! `RegisterWaitForSingleObject` function in the kernel32.dll.
//!
//! This strategy is the same that libuv takes and essentially just queues up a
//! wait for the process in a kernel32-specific thread pool. Once the object is
//! notified (e.g. the process exits) then we have a callback that basically
//! just completes a `Oneshot`.
//!
//! The `poll_exit` implementation will attempt to wait for the process in a
//! nonblocking fashion, but failing that it'll fire off a
//! `RegisterWaitForSingleObject` and then wait on the other end of the oneshot
//! from then on out.
use crate::io::{blocking::Blocking, AsyncRead, AsyncWrite, ReadBuf};
use crate::process::kill::Kill;
use crate::process::SpawnedChild;
use crate::sync::oneshot;
use std::fmt;
use std::fs::File as StdFile;
use std::future::Future;
use std::io;
use std::os::windows::prelude::{AsRawHandle, IntoRawHandle, OwnedHandle, RawHandle};
use std::pin::Pin;
use std::process::Stdio;
use std::process::{Child as StdChild, ExitStatus};
use std::ptr::null_mut;
use std::sync::Arc;
use std::task::{Context, Poll};
use windows_sys::{
    Win32::Foundation::{
        DuplicateHandle, DUPLICATE_SAME_ACCESS, HANDLE, INVALID_HANDLE_VALUE,
    },
    Win32::System::Threading::{
        GetCurrentProcess, RegisterWaitForSingleObject, UnregisterWaitEx, INFINITE,
        WT_EXECUTEINWAITTHREAD, WT_EXECUTEONLYONCE,
    },
};
#[must_use = "futures do nothing unless polled"]
pub(crate) struct Child {
    child: StdChild,
    waiting: Option<Waiting>,
}
impl fmt::Debug for Child {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
struct Waiting {
    rx: oneshot::Receiver<()>,
    wait_object: HANDLE,
    tx: *mut Option<oneshot::Sender<()>>,
}
unsafe impl Sync for Waiting {}
unsafe impl Send for Waiting {}
pub(crate) fn build_child(mut child: StdChild) -> io::Result<SpawnedChild> {
    panic!("STUB: not implemented");
}
impl Child {
    pub(crate) fn id(&self) -> u32 {
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
impl AsRawHandle for Child {
    fn as_raw_handle(&self) -> RawHandle {
        panic!("STUB: not implemented");
    }
}
impl Drop for Waiting {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
unsafe extern "system" fn callback(ptr: *mut std::ffi::c_void, _timer_fired: bool) {
    panic!("STUB: not implemented");
}
#[derive(Debug)]
struct ArcFile(Arc<StdFile>);
impl io::Read for ArcFile {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
}
impl io::Write for ArcFile {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
    fn flush(&mut self) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug)]
pub(crate) struct ChildStdio {
    raw: Arc<StdFile>,
    io: Blocking<ArcFile>,
}
impl ChildStdio {
    pub(super) fn into_owned_handle(self) -> io::Result<OwnedHandle> {
        panic!("STUB: not implemented");
    }
}
impl AsRawHandle for ChildStdio {
    fn as_raw_handle(&self) -> RawHandle {
        panic!("STUB: not implemented");
    }
}
impl AsyncRead for ChildStdio {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
impl AsyncWrite for ChildStdio {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
pub(super) fn stdio<T>(io: T) -> io::Result<ChildStdio>
where
    T: IntoRawHandle,
{
    panic!("STUB: not implemented");
}
fn convert_to_file(child_stdio: ChildStdio) -> io::Result<StdFile> {
    panic!("STUB: not implemented");
}
pub(crate) fn convert_to_stdio(child_stdio: ChildStdio) -> io::Result<Stdio> {
    panic!("STUB: not implemented");
}
fn duplicate_handle<T: AsRawHandle>(io: &T) -> io::Result<StdFile> {
    panic!("STUB: not implemented");
}
