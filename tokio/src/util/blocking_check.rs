#[cfg(unix)]
use std::os::fd::AsFd;
#[cfg(unix)]
#[allow(unused_variables)]
#[track_caller]
pub(crate) fn check_socket_for_blocking<S: AsFd>(s: &S) -> crate::io::Result<()> {
    panic!("STUB: not implemented");
}
#[cfg(not(unix))]
#[allow(unused_variables)]
pub(crate) fn check_socket_for_blocking<S>(s: &S) -> crate::io::Result<()> {
    panic!("STUB: not implemented");
}
