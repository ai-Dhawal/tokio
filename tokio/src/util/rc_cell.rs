use crate::loom::cell::UnsafeCell;
use std::rc::Rc;
/// This is exactly like `Cell<Option<Rc<T>>>`, except that it provides a `get`
/// method even though `Rc` is not `Copy`.
pub(crate) struct RcCell<T> {
    inner: UnsafeCell<Option<Rc<T>>>,
}
impl<T> RcCell<T> {
    #[cfg(not(all(loom, test)))]
    pub(crate) const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }
    #[cfg(all(loom, test))]
    pub(crate) fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }
    /// Safety: This method may not be called recursively.
    #[inline]
    unsafe fn with_inner<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Option<Rc<T>>) -> R,
    {
        panic!("STUB: not implemented");
    }
    pub(crate) fn get(&self) -> Option<Rc<T>> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn replace(&self, val: Option<Rc<T>>) -> Option<Rc<T>> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn set(&self, val: Option<Rc<T>>) {
        panic!("STUB: not implemented");
    }
}
