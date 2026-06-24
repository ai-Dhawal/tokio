use super::{Context, CONTEXT};
use crate::runtime::{scheduler, TryCurrentError};
use crate::util::markers::SyncNotSend;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
#[derive(Debug)]
#[must_use]
pub(crate) struct SetCurrentGuard {
    prev: Option<scheduler::Handle>,
    depth: usize,
    _p: PhantomData<SyncNotSend>,
}
pub(super) struct HandleCell {
    /// Current handle
    handle: RefCell<Option<scheduler::Handle>>,
    /// Tracks the number of nested calls to `try_set_current`.
    depth: Cell<usize>,
}
/// Sets this [`Handle`] as the current active [`Handle`].
///
/// [`Handle`]: crate::runtime::scheduler::Handle
pub(crate) fn try_set_current(handle: &scheduler::Handle) -> Option<SetCurrentGuard> {
    panic!("STUB: not implemented");
}
pub(crate) fn with_current<F, R>(f: F) -> Result<R, TryCurrentError>
where
    F: FnOnce(&scheduler::Handle) -> R,
{
    panic!("STUB: not implemented");
}
impl Context {
    pub(super) fn set_current(&self, handle: &scheduler::Handle) -> SetCurrentGuard {
        panic!("STUB: not implemented");
    }
}
impl HandleCell {
    pub(super) const fn new() -> HandleCell {
        HandleCell {
            handle: RefCell::new(None),
            depth: Cell::new(0),
        }
    }
}
impl Drop for SetCurrentGuard {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
