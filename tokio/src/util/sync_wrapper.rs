//! This module contains a type that can make `Send + !Sync` types `Sync` by
//! disallowing all immutable access to the value.
//!
//! A similar primitive is provided in the `sync_wrapper` crate.
use std::any::Any;
pub(crate) struct SyncWrapper<T> {
    value: T,
}
unsafe impl<T: Send> Send for SyncWrapper<T> {}
unsafe impl<T> Sync for SyncWrapper<T> {}
impl<T> SyncWrapper<T> {
    pub(crate) fn new(value: T) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn into_inner(self) -> T {
        panic!("STUB: not implemented");
    }
}
impl SyncWrapper<Box<dyn Any + Send>> {
    /// Attempt to downcast using `Any::downcast_ref()` to a type that is known to be `Sync`.
    pub(crate) fn downcast_ref_sync<T: Any + Sync>(&self) -> Option<&T> {
        panic!("STUB: not implemented");
    }
}
