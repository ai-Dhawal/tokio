#[cfg(tokio_internal_mt_counters)]
mod imp {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering::Relaxed;
    static NUM_MAINTENANCE: AtomicUsize = AtomicUsize::new(0);
    static NUM_NOTIFY_LOCAL: AtomicUsize = AtomicUsize::new(0);
    static NUM_UNPARKS_LOCAL: AtomicUsize = AtomicUsize::new(0);
    static NUM_LIFO_SCHEDULES: AtomicUsize = AtomicUsize::new(0);
    static NUM_LIFO_CAPPED: AtomicUsize = AtomicUsize::new(0);
    impl Drop for super::Counters {
        fn drop(&mut self) {
            panic!("STUB: not implemented");
        }
    }
    pub(crate) fn inc_num_inc_notify_local() {
        panic!("STUB: not implemented");
    }
    pub(crate) fn inc_num_unparks_local() {
        panic!("STUB: not implemented");
    }
    pub(crate) fn inc_num_maintenance() {
        panic!("STUB: not implemented");
    }
    pub(crate) fn inc_lifo_schedules() {
        panic!("STUB: not implemented");
    }
    pub(crate) fn inc_lifo_capped() {
        panic!("STUB: not implemented");
    }
}
#[cfg(not(tokio_internal_mt_counters))]
mod imp {
    pub(crate) fn inc_num_inc_notify_local() {
        panic!("STUB: not implemented");
    }
    pub(crate) fn inc_num_unparks_local() {
        panic!("STUB: not implemented");
    }
    pub(crate) fn inc_num_maintenance() {
        panic!("STUB: not implemented");
    }
    pub(crate) fn inc_lifo_schedules() {
        panic!("STUB: not implemented");
    }
    pub(crate) fn inc_lifo_capped() {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug)]
pub(crate) struct Counters;
pub(super) use imp::*;
