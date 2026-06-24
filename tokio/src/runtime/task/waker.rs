use crate::runtime::task::{Header, RawTask, Schedule};
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops;
use std::ptr::NonNull;
use std::task::{RawWaker, RawWakerVTable, Waker};
pub(crate) struct WakerRef<'a, S: 'static> {
    waker: ManuallyDrop<Waker>,
    _p: PhantomData<(&'a Header, S)>,
}
/// Returns a `WakerRef` which avoids having to preemptively increase the
/// refcount if there is no need to do so.
pub(super) fn waker_ref<S>(header: &NonNull<Header>) -> WakerRef<'_, S>
where
    S: Schedule,
{
    panic!("STUB: not implemented");
}
impl<S> ops::Deref for WakerRef<'_, S> {
    type Target = Waker;
    fn deref(&self) -> &Waker {
        panic!("STUB: not implemented");
    }
}
cfg_trace! {
    #[doc = " # Safety"] #[doc = ""] #[doc =
    " `$header` must be a valid pointer to a [`Header`]."] macro_rules! trace { ($header
    : expr, $op : expr) => { if let Some(id) = Header::get_tracing_id(&$header) {
    tracing::trace!(target : "tokio::task::waker", op = $op, task.id = id.into_u64(),); }
    } }
}
cfg_not_trace! {
    macro_rules! trace { ($header : expr, $op : expr) => { let _ = &$header; } }
}
unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
    panic!("STUB: not implemented");
}
unsafe fn drop_waker(ptr: *const ()) {
    panic!("STUB: not implemented");
}
unsafe fn wake_by_val(ptr: *const ()) {
    panic!("STUB: not implemented");
}
unsafe fn wake_by_ref(ptr: *const ()) {
    panic!("STUB: not implemented");
}
static WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    clone_waker,
    wake_by_val,
    wake_by_ref,
    drop_waker,
);
fn raw_waker(header: NonNull<Header>) -> RawWaker {
    panic!("STUB: not implemented");
}
