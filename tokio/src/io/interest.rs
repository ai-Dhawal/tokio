#![cfg_attr(not(feature = "net"), allow(dead_code, unreachable_pub))]
use crate::io::ready::Ready;
use std::fmt;
use std::ops;
const READABLE: usize = 0b0001;
const WRITABLE: usize = 0b0010;
#[cfg(target_os = "freebsd")]
const AIO: usize = 0b0100;
#[cfg(target_os = "freebsd")]
const LIO: usize = 0b1000;
#[cfg(any(target_os = "linux", target_os = "android"))]
const PRIORITY: usize = 0b0001_0000;
const ERROR: usize = 0b0010_0000;
/// Readiness event interest.
///
/// Specifies the readiness events the caller is interested in when awaiting on
/// I/O resource readiness states.
#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Interest(usize);
impl Interest {
    cfg_aio! {
        #[doc = " Interest for POSIX AIO."] #[cfg(target_os = "freebsd")] pub const AIO :
        Interest = Interest(AIO); #[doc = " Interest for POSIX AIO."] #[cfg(not(target_os
        = "freebsd"))] pub const AIO : Interest = Interest(READABLE); #[doc =
        " Interest for POSIX AIO `lio_listio` events."] #[cfg(target_os = "freebsd")] pub
        const LIO : Interest = Interest(LIO); #[doc =
        " Interest for POSIX AIO `lio_listio` events."] #[cfg(not(target_os =
        "freebsd"))] pub const LIO : Interest = Interest(READABLE);
    }
    /// Interest in all readable events.
    ///
    /// Readable interest includes read-closed events.
    pub const READABLE: Interest = Interest(READABLE);
    /// Interest in all writable events.
    ///
    /// Writable interest includes write-closed events.
    pub const WRITABLE: Interest = Interest(WRITABLE);
    /// Interest in error events.
    ///
    /// Passes error interest to the underlying OS selector.
    /// Behavior is platform-specific, read your platform's documentation.
    pub const ERROR: Interest = Interest(ERROR);
    /// Returns a `Interest` set representing priority completion interests.
    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "android"))))]
    pub const PRIORITY: Interest = Interest(PRIORITY);
    /// Returns true if the value includes readable interest.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::io::Interest;
    ///
    /// assert!(Interest::READABLE.is_readable());
    /// assert!(!Interest::WRITABLE.is_readable());
    ///
    /// let both = Interest::READABLE | Interest::WRITABLE;
    /// assert!(both.is_readable());
    /// ```
    pub const fn is_readable(self) -> bool {
        self.0 & READABLE != 0
    }
    /// Returns true if the value includes writable interest.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::io::Interest;
    ///
    /// assert!(!Interest::READABLE.is_writable());
    /// assert!(Interest::WRITABLE.is_writable());
    ///
    /// let both = Interest::READABLE | Interest::WRITABLE;
    /// assert!(both.is_writable());
    /// ```
    pub const fn is_writable(self) -> bool {
        self.0 & WRITABLE != 0
    }
    /// Returns true if the value includes error interest.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::io::Interest;
    ///
    /// assert!(Interest::ERROR.is_error());
    /// assert!(!Interest::WRITABLE.is_error());
    ///
    /// let combined = Interest::READABLE | Interest::ERROR;
    /// assert!(combined.is_error());
    /// ```
    pub const fn is_error(self) -> bool {
        self.0 & ERROR != 0
    }
    #[cfg(target_os = "freebsd")]
    const fn is_aio(self) -> bool {
        self.0 & AIO != 0
    }
    #[cfg(target_os = "freebsd")]
    const fn is_lio(self) -> bool {
        self.0 & LIO != 0
    }
    /// Returns true if the value includes priority interest.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::io::Interest;
    ///
    /// assert!(!Interest::READABLE.is_priority());
    /// assert!(Interest::PRIORITY.is_priority());
    ///
    /// let both = Interest::READABLE | Interest::PRIORITY;
    /// assert!(both.is_priority());
    /// ```
    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "android"))))]
    pub const fn is_priority(self) -> bool {
        self.0 & PRIORITY != 0
    }
    /// Add together two `Interest` values.
    ///
    /// This function works from a `const` context.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::io::Interest;
    ///
    /// const BOTH: Interest = Interest::READABLE.add(Interest::WRITABLE);
    ///
    /// assert!(BOTH.is_readable());
    /// assert!(BOTH.is_writable());
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn add(self, other: Interest) -> Interest {
        Self(self.0 | other.0)
    }
    /// Remove `Interest` from `self`.
    ///
    /// Interests present in `other` but *not* in `self` are ignored.
    ///
    /// Returns `None` if the set would be empty after removing `Interest`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::io::Interest;
    ///
    /// const RW_INTEREST: Interest = Interest::READABLE.add(Interest::WRITABLE);
    ///
    /// let w_interest = RW_INTEREST.remove(Interest::READABLE).unwrap();
    /// assert!(!w_interest.is_readable());
    /// assert!(w_interest.is_writable());
    ///
    /// // Removing all interests from the set returns `None`.
    /// assert_eq!(w_interest.remove(Interest::WRITABLE), None);
    ///
    /// // Remove all interests at once.
    /// assert_eq!(RW_INTEREST.remove(RW_INTEREST), None);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn remove(self, other: Interest) -> Option<Interest> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn to_mio(self) -> mio::Interest {
        panic!("STUB: not implemented");
    }
    pub(crate) fn mask(self) -> Ready {
        panic!("STUB: not implemented");
    }
}
impl ops::BitOr for Interest {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        panic!("STUB: not implemented");
    }
}
impl ops::BitOrAssign for Interest {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for Interest {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
