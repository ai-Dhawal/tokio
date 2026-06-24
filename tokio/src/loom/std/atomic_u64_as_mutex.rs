use crate::loom::sync::Mutex;
use std::sync::atomic::Ordering;
cfg_has_const_mutex_new! {
    #[path = "atomic_u64_static_const_new.rs"] mod static_macro;
}
cfg_not_has_const_mutex_new! {
    #[path = "atomic_u64_static_once_cell.rs"] mod static_macro;
}
pub(crate) use static_macro::StaticAtomicU64;
#[derive(Debug)]
pub(crate) struct AtomicU64 {
    inner: Mutex<u64>,
}
impl AtomicU64 {
    pub(crate) fn load(&self, _: Ordering) -> u64 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn store(&self, val: u64, _: Ordering) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn fetch_add(&self, val: u64, _: Ordering) -> u64 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn fetch_or(&self, val: u64, _: Ordering) -> u64 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn compare_exchange(
        &self,
        current: u64,
        new: u64,
        _success: Ordering,
        _failure: Ordering,
    ) -> Result<u64, u64> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn compare_exchange_weak(
        &self,
        current: u64,
        new: u64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<u64, u64> {
        panic!("STUB: not implemented");
    }
}
impl Default for AtomicU64 {
    fn default() -> AtomicU64 {
        panic!("STUB: not implemented");
    }
}
