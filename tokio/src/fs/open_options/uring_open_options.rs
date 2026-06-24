use std::{io, os::unix::fs::OpenOptionsExt};
#[cfg(test)]
use super::mock_open_options::MockOpenOptions as StdOpenOptions;
#[cfg(not(test))]
use std::fs::OpenOptions as StdOpenOptions;
#[derive(Debug, Clone)]
pub(crate) struct UringOpenOptions {
    pub(crate) read: bool,
    pub(crate) write: bool,
    pub(crate) append: bool,
    pub(crate) truncate: bool,
    pub(crate) create: bool,
    pub(crate) create_new: bool,
    pub(crate) mode: libc::mode_t,
    pub(crate) custom_flags: libc::c_int,
}
impl UringOpenOptions {
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn append(&mut self, append: bool) -> &mut Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn create(&mut self, create: bool) -> &mut Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn create_new(&mut self, create_new: bool) -> &mut Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn read(&mut self, read: bool) -> &mut Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn write(&mut self, write: bool) -> &mut Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn truncate(&mut self, truncate: bool) -> &mut Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn mode(&mut self, mode: u32) -> &mut Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn custom_flags(&mut self, flags: i32) -> &mut Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn access_mode(&self) -> io::Result<libc::c_int> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn creation_mode(&self) -> io::Result<libc::c_int> {
        panic!("STUB: not implemented");
    }
}
impl From<UringOpenOptions> for StdOpenOptions {
    fn from(value: UringOpenOptions) -> Self {
        panic!("STUB: not implemented");
    }
}
