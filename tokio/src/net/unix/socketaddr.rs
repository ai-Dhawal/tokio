use std::fmt;
use std::path::Path;
/// An address associated with a Tokio Unix socket.
///
/// This type is a thin wrapper around [`std::os::unix::net::SocketAddr`]. You
/// can convert to and from the standard library `SocketAddr` type using the
/// [`From`] trait.
#[derive(Clone)]
pub struct SocketAddr(pub(super) std::os::unix::net::SocketAddr);
impl SocketAddr {
    /// Returns `true` if the address is unnamed.
    ///
    /// Documentation reflected in [`SocketAddr`].
    ///
    /// [`SocketAddr`]: std::os::unix::net::SocketAddr
    pub fn is_unnamed(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns the contents of this address if it is a `pathname` address.
    ///
    /// Documentation reflected in [`SocketAddr`].
    ///
    /// [`SocketAddr`]: std::os::unix::net::SocketAddr
    pub fn as_pathname(&self) -> Option<&Path> {
        panic!("STUB: not implemented");
    }
    /// Returns the contents of this address if it is in the abstract namespace.
    ///
    /// Documentation reflected in [`SocketAddrExt`].
    /// The abstract namespace is a Linux-specific feature.
    ///
    ///
    /// [`SocketAddrExt`]: std::os::linux::net::SocketAddrExt
    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "android"))))]
    pub fn as_abstract_name(&self) -> Option<&[u8]> {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for SocketAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl From<std::os::unix::net::SocketAddr> for SocketAddr {
    fn from(value: std::os::unix::net::SocketAddr) -> Self {
        panic!("STUB: not implemented");
    }
}
impl From<SocketAddr> for std::os::unix::net::SocketAddr {
    fn from(value: SocketAddr) -> Self {
        panic!("STUB: not implemented");
    }
}
