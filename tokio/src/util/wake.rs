use crate::loom::sync::Arc;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::task::{RawWaker, RawWakerVTable, Waker};
/// Simplified waking interface based on Arcs.
pub(crate) trait Wake: Send + Sync + Sized + 'static {
    /// Wake by value.
    fn wake(arc_self: Arc<Self>);
    /// Wake by reference.
    fn wake_by_ref(arc_self: &Arc<Self>);
}
/// A `Waker` that is only valid for a given lifetime.
#[derive(Debug)]
pub(crate) struct WakerRef<'a> {
    waker: ManuallyDrop<Waker>,
    _p: PhantomData<&'a ()>,
}
impl Deref for WakerRef<'_> {
    type Target = Waker;
    fn deref(&self) -> &Waker {
        panic!("STUB: not implemented");
    }
}
/// Creates a reference to a `Waker` from a reference to `Arc<impl Wake>`.
pub(crate) fn waker_ref<W: Wake>(wake: &Arc<W>) -> WakerRef<'_> {
    panic!("STUB: not implemented");
}
fn waker_vtable<W: Wake>() -> &'static RawWakerVTable {
    panic!("STUB: not implemented");
}
unsafe fn clone_arc_raw<T: Wake>(data: *const ()) -> RawWaker {
    panic!("STUB: not implemented");
}
unsafe fn wake_arc_raw<T: Wake>(data: *const ()) {
    panic!("STUB: not implemented");
}
unsafe fn wake_by_ref_arc_raw<T: Wake>(data: *const ()) {
    panic!("STUB: not implemented");
}
unsafe fn drop_arc_raw<T: Wake>(data: *const ()) {
    panic!("STUB: not implemented");
}
