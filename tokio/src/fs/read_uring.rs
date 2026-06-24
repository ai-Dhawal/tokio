use crate::fs::OpenOptions;
use crate::runtime::driver::op::Op;
use std::io;
use std::io::ErrorKind;
use std::os::fd::OwnedFd;
use std::path::Path;
const PROBE_SIZE: usize = 32;
const PROBE_SIZE_U32: u32 = PROBE_SIZE as u32;
const MAX_READ_SIZE: usize = 64 * 1024 * 1024;
pub(crate) async fn read_uring(path: &Path) -> io::Result<Vec<u8>> {
    panic!("STUB: not implemented");
}
async fn read_to_end_uring(mut fd: OwnedFd, mut buf: Vec<u8>) -> io::Result<Vec<u8>> {
    panic!("STUB: not implemented");
}
async fn small_probe_read(
    fd: OwnedFd,
    mut buf: Vec<u8>,
    offset: &mut u64,
) -> io::Result<(OwnedFd, Vec<u8>, bool)> {
    panic!("STUB: not implemented");
}
async fn op_read(
    mut fd: OwnedFd,
    mut buf: Vec<u8>,
    offset: &mut u64,
    read_len: u32,
) -> io::Result<(OwnedFd, Vec<u8>, bool)> {
    panic!("STUB: not implemented");
}
