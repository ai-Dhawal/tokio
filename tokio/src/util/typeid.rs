use std::{any::TypeId, marker::PhantomData, mem::{self, ManuallyDrop}};
pub(super) unsafe fn try_transmute<Src, Target: 'static>(x: Src) -> Result<Target, Src> {
    panic!("STUB: not implemented");
}
#[inline(always)]
fn nonstatic_typeid<T>() -> TypeId
where
    T: ?Sized,
{
    panic!("STUB: not implemented");
}
