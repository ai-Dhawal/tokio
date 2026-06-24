use crate::net::unix;
/// Credentials of a process.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct UCred {
    /// PID (process ID) of the process.
    pid: Option<unix::pid_t>,
    /// UID (user ID) of the process.
    uid: unix::uid_t,
    /// GID (group ID) of the process.
    gid: unix::gid_t,
}
impl UCred {
    /// Gets UID (user ID) of the process.
    pub fn uid(&self) -> unix::uid_t {
        panic!("STUB: not implemented");
    }
    /// Gets GID (group ID) of the process.
    pub fn gid(&self) -> unix::gid_t {
        panic!("STUB: not implemented");
    }
    /// Gets PID (process ID) of the process.
    ///
    /// This is implemented under Linux, Android, OpenBSD, FreeBSD (since
    /// FreeBSD 13), NetBSD, NTO, iOS, macOS, tvOS, watchOS, visionOS,
    /// Solaris, Illumos, Cygwin, Haiku, and Redox. On other platforms this
    /// will always return `None`.
    pub fn pid(&self) -> Option<unix::pid_t> {
        panic!("STUB: not implemented");
    }
}
#[cfg(
    any(
        target_os = "linux",
        target_os = "redox",
        target_os = "android",
        target_os = "openbsd",
        target_os = "haiku",
        target_os = "cygwin"
    )
)]
pub(crate) use self::impl_linux::get_peer_cred;
#[cfg(any(target_os = "netbsd", target_os = "nto"))]
pub(crate) use self::impl_netbsd::get_peer_cred;
#[cfg(target_os = "dragonfly")]
pub(crate) use self::impl_dragonfly::get_peer_cred;
#[cfg(target_os = "freebsd")]
pub(crate) use self::impl_freebsd::get_peer_cred;
#[cfg(
    any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "visionos"
    )
)]
pub(crate) use self::impl_macos::get_peer_cred;
#[cfg(any(target_os = "solaris", target_os = "illumos"))]
pub(crate) use self::impl_solaris::get_peer_cred;
#[cfg(target_os = "aix")]
pub(crate) use self::impl_aix::get_peer_cred;
#[cfg(any(target_os = "espidf", target_os = "vita", target_os = "hurd"))]
pub(crate) use self::impl_noproc::get_peer_cred;
#[cfg(
    any(
        target_os = "linux",
        target_os = "redox",
        target_os = "android",
        target_os = "openbsd",
        target_os = "haiku",
        target_os = "cygwin"
    )
)]
pub(crate) mod impl_linux {
    use crate::net::unix::{self, UnixStream};
    use libc::{c_void, getsockopt, socklen_t, SOL_SOCKET, SO_PEERCRED};
    use std::{io, mem};
    #[cfg(target_os = "openbsd")]
    use libc::sockpeercred as ucred;
    #[cfg(
        any(
            target_os = "linux",
            target_os = "redox",
            target_os = "android",
            target_os = "haiku",
            target_os = "cygwin"
        )
    )]
    use libc::ucred;
    pub(crate) fn get_peer_cred(sock: &UnixStream) -> io::Result<super::UCred> {
        panic!("STUB: not implemented");
    }
}
#[cfg(any(target_os = "netbsd", target_os = "nto"))]
pub(crate) mod impl_netbsd {
    use crate::net::unix::{self, UnixStream};
    use libc::{c_void, getsockopt, socklen_t, unpcbid, LOCAL_PEEREID, SOL_SOCKET};
    use std::io;
    use std::mem::size_of;
    use std::os::unix::io::AsRawFd;
    pub(crate) fn get_peer_cred(sock: &UnixStream) -> io::Result<super::UCred> {
        panic!("STUB: not implemented");
    }
}
#[cfg(target_os = "dragonfly")]
pub(crate) mod impl_dragonfly {
    use crate::net::unix::{self, UnixStream};
    use libc::getpeereid;
    use std::io;
    use std::mem::MaybeUninit;
    use std::os::unix::io::AsRawFd;
    pub(crate) fn get_peer_cred(sock: &UnixStream) -> io::Result<super::UCred> {
        panic!("STUB: not implemented");
    }
}
#[cfg(target_os = "freebsd")]
pub(crate) mod impl_freebsd {
    use crate::net::unix::{self, UnixStream};
    use libc::{c_void, getsockopt, socklen_t, xucred, LOCAL_PEERCRED, XUCRED_VERSION};
    use std::io;
    use std::mem::{size_of, MaybeUninit};
    use std::os::unix::io::AsRawFd;
    pub(crate) fn get_peer_cred(sock: &UnixStream) -> io::Result<super::UCred> {
        panic!("STUB: not implemented");
    }
}
#[cfg(
    any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "visionos"
    )
)]
pub(crate) mod impl_macos {
    use crate::net::unix::{self, UnixStream};
    use libc::{c_void, getpeereid, getsockopt, pid_t, LOCAL_PEEREPID, SOL_LOCAL};
    use std::io;
    use std::mem::size_of;
    use std::mem::MaybeUninit;
    use std::os::unix::io::AsRawFd;
    pub(crate) fn get_peer_cred(sock: &UnixStream) -> io::Result<super::UCred> {
        panic!("STUB: not implemented");
    }
}
#[cfg(any(target_os = "solaris", target_os = "illumos"))]
pub(crate) mod impl_solaris {
    use crate::net::unix::{self, UnixStream};
    use std::io;
    use std::os::unix::io::AsRawFd;
    use std::ptr;
    pub(crate) fn get_peer_cred(sock: &UnixStream) -> io::Result<super::UCred> {
        panic!("STUB: not implemented");
    }
}
#[cfg(target_os = "aix")]
pub(crate) mod impl_aix {
    use crate::net::unix::UnixStream;
    use std::io;
    use std::os::unix::io::AsRawFd;
    pub(crate) fn get_peer_cred(sock: &UnixStream) -> io::Result<super::UCred> {
        panic!("STUB: not implemented");
    }
}
#[cfg(any(target_os = "espidf", target_os = "vita", target_os = "hurd"))]
pub(crate) mod impl_noproc {
    use crate::net::unix::UnixStream;
    use std::io;
    pub(crate) fn get_peer_cred(_sock: &UnixStream) -> io::Result<super::UCred> {
        panic!("STUB: not implemented");
    }
}
