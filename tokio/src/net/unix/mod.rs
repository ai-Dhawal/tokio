//! Unix specific network types.
#[doc(hidden)]
pub mod datagram;
pub(crate) mod listener;
pub(crate) mod socket;
mod split;
pub use split::{ReadHalf, WriteHalf};
mod split_owned;
pub use split_owned::{OwnedReadHalf, OwnedWriteHalf, ReuniteError};
mod socketaddr;
pub use socketaddr::SocketAddr;
pub(crate) mod stream;
pub(crate) use stream::UnixStream;
mod ucred;
pub use ucred::UCred;
pub mod pipe;
/// A type representing user ID.
#[allow(non_camel_case_types)]
pub type uid_t = u32;
/// A type representing group ID.
#[allow(non_camel_case_types)]
pub type gid_t = u32;
/// A type representing process and process group IDs.
#[allow(non_camel_case_types)]
pub type pid_t = i32;
