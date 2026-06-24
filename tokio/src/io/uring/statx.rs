#![cfg(
    all(
        tokio_unstable,
        feature = "io-uring",
        feature = "rt",
        feature = "fs",
        any(target_env = "gnu", target_os = "android")
    )
)]
use crate::fs::File;
use crate::io::uring::utils::cstr;
use crate::runtime::driver::op::{CancelData, Cancellable, Completable, CqeResult, Op};
use io_uring::{opcode, types};
use libc::statx;
use std::ffi::{CStr, CString};
use std::fmt::{Debug, Formatter};
use std::io;
use std::mem::MaybeUninit;
use std::os::fd::AsRawFd;
use std::path::Path;
pub(crate) struct Metadata(statx);
impl Metadata {
    /// Returns the size of the file, in bytes, this metadata is for.
    pub(crate) fn len(&self) -> u64 {
        panic!("STUB: not implemented");
    }
}
impl Debug for Metadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug)]
pub(crate) struct Statx {
    /// This field will be read by the kernel during the operation, so we
    /// need to ensure it is valid for the entire duration of the operation.
    _path: CString,
    buffer: Box<MaybeUninit<statx>>,
}
impl Completable for Statx {
    type Output = io::Result<Metadata>;
    fn complete(self, cqe: CqeResult) -> Self::Output {
        panic!("STUB: not implemented");
    }
    fn complete_with_error(self, error: io::Error) -> Self::Output {
        panic!("STUB: not implemented");
    }
}
impl Cancellable for Statx {
    fn cancel(self) -> CancelData {
        panic!("STUB: not implemented");
    }
}
impl Op<Statx> {
    /// Submit a request to retrieve a file's status.
    #[inline]
    fn statx(path: &Path, flags: i32) -> io::Result<Op<Statx>> {
        panic!("STUB: not implemented");
    }
    /// Retrieves the metadata information of the given path, following symlinks
    /// if the path provided points to a symlink location.
    #[inline]
    pub(crate) fn metadata(path: &Path) -> io::Result<Op<Statx>> {
        panic!("STUB: not implemented");
    }
    /// Retrieves the metadata information of the given file
    pub(crate) fn file_metadata(file: &File) -> io::Result<Op<Statx>> {
        panic!("STUB: not implemented");
    }
    /// Retrieves the metadata information of the given path without following symlinks.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn symlink_metadata(path: &Path) -> io::Result<Op<Statx>> {
        panic!("STUB: not implemented");
    }
}
