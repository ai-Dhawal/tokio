use std::cell::RefCell;
use std::task::Waker;
pub(crate) struct Defer {
    deferred: RefCell<Vec<Waker>>,
}
impl Defer {
    pub(crate) fn new() -> Defer {
        panic!("STUB: not implemented");
    }
    pub(crate) fn defer(&self, waker: &Waker) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn wake(&self) {
        panic!("STUB: not implemented");
    }
    #[cfg(feature = "taskdump")]
    pub(crate) fn take_deferred(&self) -> Vec<Waker> {
        panic!("STUB: not implemented");
    }
}
