use crate::loom::sync::atomic::AtomicPtr;
use std::ptr;
use std::sync::atomic::Ordering::AcqRel;
pub(crate) struct AtomicCell<T> {
    data: AtomicPtr<T>,
}
unsafe impl<T: Send> Send for AtomicCell<T> {}
unsafe impl<T: Send> Sync for AtomicCell<T> {}
impl<T> AtomicCell<T> {
    pub(crate) fn new(data: Option<Box<T>>) -> AtomicCell<T> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn swap(&self, val: Option<Box<T>>) -> Option<Box<T>> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn set(&self, val: Box<T>) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn take(&self) -> Option<Box<T>> {
        panic!("STUB: not implemented");
    }
}
fn to_raw<T>(data: Option<Box<T>>) -> *mut T {
    panic!("STUB: not implemented");
}
fn from_raw<T>(val: *mut T) -> Option<Box<T>> {
    panic!("STUB: not implemented");
}
impl<T> Drop for AtomicCell<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
