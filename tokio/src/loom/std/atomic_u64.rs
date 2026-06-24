//! Implementation of an atomic `u64` cell. On 64 bit platforms, this is a
//! re-export of `AtomicU64`. On 32 bit platforms, this is implemented using a
//! `Mutex`.
cfg_has_atomic_u64! {
    #[path = "atomic_u64_native.rs"] mod imp;
}
cfg_not_has_atomic_u64! {
    #[path = "atomic_u64_as_mutex.rs"] mod imp;
}
pub(crate) use imp::AtomicU64;
