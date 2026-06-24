//! Utility for helping miri understand our exposed pointers.
//!
//! During normal execution, this module is equivalent to pointer casts. However, when running
//! under miri, pointer casts are replaced with lookups in a hash map. This makes Tokio compatible
//! with strict provenance when running under miri (which comes with a performance cost).
use std::marker::PhantomData;
#[cfg(miri)]
use {crate::loom::sync::Mutex, std::collections::BTreeMap};
pub(crate) struct PtrExposeDomain<T> {
    #[cfg(miri)]
    map: Mutex<BTreeMap<usize, *const T>>,
    _phantom: PhantomData<T>,
}
unsafe impl<T> Sync for PtrExposeDomain<T> {}
impl<T> PtrExposeDomain<T> {
    pub(crate) const fn new() -> Self {
        Self {
            #[cfg(miri)]
            map: Mutex::const_new(BTreeMap::new()),
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub(crate) fn expose_provenance(&self, ptr: *const T) -> usize {
        panic!("STUB: not implemented");
    }
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_exposed_addr(&self, addr: usize) -> *const T {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn unexpose_provenance(&self, _ptr: *const T) {
        panic!("STUB: not implemented");
    }
}
