//! Mock version of std::fs::File;
use mockall::mock;
use crate::sync::oneshot;
#[cfg(all(test, unix))]
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::{
    cell::RefCell, collections::VecDeque, fs::{Metadata, Permissions},
    future::Future, io::{self, Read, Seek, SeekFrom, Write},
    path::PathBuf, pin::Pin, task::{Context, Poll},
};
mock! {
    #[derive(Debug)] pub File { pub fn create(pb : PathBuf) -> io::Result < Self >; pub
    fn inner_flush(& self) -> io::Result < () >; pub fn inner_read(& self, dst : & mut
    [u8]) -> io::Result < usize >; pub fn inner_seek(& self, pos : SeekFrom) ->
    io::Result < u64 >; pub fn inner_write(& self, src : & [u8]) -> io::Result < usize >;
    pub fn metadata(& self) -> io::Result < Metadata >; pub fn open(pb : PathBuf) ->
    io::Result < Self >; pub fn set_len(& self, size : u64) -> io::Result < () >; pub fn
    set_permissions(& self, _perm : Permissions) -> io::Result < () >; pub fn
    set_max_buf_size(& self, max_buf_size : usize); pub fn sync_all(& self) -> io::Result
    < () >; pub fn sync_data(& self) -> io::Result < () >; pub fn try_clone(& self) ->
    io::Result < Self >; } #[cfg(windows)] impl std::os::windows::io::AsRawHandle for
    File { fn as_raw_handle(& self) -> std::os::windows::io::RawHandle; } #[cfg(windows)]
    impl std::os::windows::io::FromRawHandle for File { unsafe fn from_raw_handle(h :
    std::os::windows::io::RawHandle) -> Self; } #[cfg(unix)] impl
    std::os::unix::io::AsRawFd for File { fn as_raw_fd(& self) ->
    std::os::unix::io::RawFd; } #[cfg(unix)] impl std::os::unix::io::FromRawFd for File {
    unsafe fn from_raw_fd(h : std::os::unix::io::RawFd) -> Self; }
}
impl Read for MockFile {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
}
impl Read for &'_ MockFile {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
}
impl Seek for &'_ MockFile {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        panic!("STUB: not implemented");
    }
}
impl Write for &'_ MockFile {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
    fn flush(&mut self) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
}
#[cfg(all(test, unix))]
impl From<MockFile> for OwnedFd {
    #[inline]
    fn from(file: MockFile) -> OwnedFd {
        panic!("STUB: not implemented");
    }
}
#[cfg(all(test, unix))]
impl From<OwnedFd> for MockFile {
    #[inline]
    fn from(file: OwnedFd) -> MockFile {
        panic!("STUB: not implemented");
    }
}
tokio_thread_local! {
    static QUEUE : RefCell < VecDeque < Box < dyn FnOnce() + Send >>> =
    RefCell::new(VecDeque::new())
}
#[derive(Debug)]
pub(super) struct JoinHandle<T> {
    rx: oneshot::Receiver<T>,
}
pub(super) fn spawn_blocking<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    panic!("STUB: not implemented");
}
pub(super) fn spawn_mandatory_blocking<F, R>(f: F) -> Option<JoinHandle<R>>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    panic!("STUB: not implemented");
}
impl<T> Future for JoinHandle<T> {
    type Output = Result<T, io::Error>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
pub(super) mod pool {
    use super::*;
    pub(in super::super) fn len() -> usize {
        panic!("STUB: not implemented");
    }
    pub(in super::super) fn run_one() {
        panic!("STUB: not implemented");
    }
}
