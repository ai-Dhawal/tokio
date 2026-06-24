use super::AtomicU64;
use crate::loom::sync::{atomic::Ordering, Mutex};
use std::sync::OnceLock;
pub(crate) struct StaticAtomicU64 {
    init: u64,
    cell: OnceLock<Mutex<u64>>,
}
impl AtomicU64 {
    pub(crate) fn new(val: u64) -> Self {
        panic!("STUB: not implemented");
    }
}
impl StaticAtomicU64 {
    pub(crate) const fn new(val: u64) -> StaticAtomicU64 {
        StaticAtomicU64 {
            init: val,
            cell: OnceLock::new(),
        }
    }
    pub(crate) fn load(&self, order: Ordering) -> u64 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn fetch_add(&self, val: u64, order: Ordering) -> u64 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn compare_exchange_weak(
        &self,
        current: u64,
        new: u64,
        _success: Ordering,
        _failure: Ordering,
    ) -> Result<u64, u64> {
        panic!("STUB: not implemented");
    }
    fn inner(&self) -> &Mutex<u64> {
        panic!("STUB: not implemented");
    }
}
