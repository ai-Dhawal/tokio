#![allow(unsafe_op_in_unsafe_fn)]
use crate::future::Future;
use crate::runtime::task::core::{Core, Trailer};
use crate::runtime::task::{Cell, Harness, Header, Id, Schedule, State};
#[cfg(tokio_unstable)]
use std::panic::Location;
use std::ptr::NonNull;
use std::task::{Poll, Waker};
/// Raw task handle
#[derive(Clone)]
pub(crate) struct RawTask {
    ptr: NonNull<Header>,
}
pub(super) struct Vtable {
    /// Polls the future.
    pub(super) poll: unsafe fn(NonNull<Header>),
    /// Schedules the task for execution on the runtime.
    pub(super) schedule: unsafe fn(NonNull<Header>),
    /// Deallocates the memory.
    pub(super) dealloc: unsafe fn(NonNull<Header>),
    /// Reads the task output, if complete.
    pub(super) try_read_output: unsafe fn(NonNull<Header>, *mut (), &Waker),
    /// The join handle has been dropped.
    pub(super) drop_join_handle_slow: unsafe fn(NonNull<Header>),
    /// An abort handle has been dropped.
    pub(super) drop_abort_handle: unsafe fn(NonNull<Header>),
    /// Scheduler is being shutdown.
    pub(super) shutdown: unsafe fn(NonNull<Header>),
    /// The number of bytes that the `trailer` field is offset from the header.
    pub(super) trailer_offset: usize,
    /// The number of bytes that the `scheduler` field is offset from the header.
    pub(super) scheduler_offset: usize,
    /// The number of bytes that the `id` field is offset from the header.
    pub(super) id_offset: usize,
    /// The number of bytes that the `spawned_at` field is offset from the header.
    #[cfg(tokio_unstable)]
    pub(super) spawn_location_offset: usize,
}
/// Get the vtable for the requested `T` and `S` generics.
pub(super) fn vtable<T: Future, S: Schedule>() -> &'static Vtable {
    panic!("STUB: not implemented");
}
/// Calling `get_trailer_offset` directly in vtable doesn't work because it
/// prevents the vtable from being promoted to a static reference.
///
/// See this thread for more info:
/// <https://users.rust-lang.org/t/custom-vtables-with-integers/78508>
struct OffsetHelper<T, S>(T, S);
impl<T: Future, S: Schedule> OffsetHelper<T, S> {
    const TRAILER_OFFSET: usize = get_trailer_offset(
        std::mem::size_of::<Header>(),
        std::mem::size_of::<Core<T, S>>(),
        std::mem::align_of::<Core<T, S>>(),
        std::mem::align_of::<Trailer>(),
    );
    const SCHEDULER_OFFSET: usize = get_core_offset(
        std::mem::size_of::<Header>(),
        std::mem::align_of::<Core<T, S>>(),
    );
    const ID_OFFSET: usize = get_id_offset(
        std::mem::size_of::<Header>(),
        std::mem::align_of::<Core<T, S>>(),
        std::mem::size_of::<S>(),
        std::mem::align_of::<Id>(),
    );
    #[cfg(tokio_unstable)]
    const SPAWN_LOCATION_OFFSET: usize = get_spawn_location_offset(
        std::mem::size_of::<Header>(),
        std::mem::align_of::<Core<T, S>>(),
        std::mem::size_of::<S>(),
        std::mem::align_of::<Id>(),
        std::mem::size_of::<Id>(),
        std::mem::align_of::<&'static Location<'static>>(),
    );
}
/// Compute the offset of the `Trailer` field in `Cell<T, S>` using the
/// `#[repr(C)]` algorithm.
///
/// Pseudo-code for the `#[repr(C)]` algorithm can be found here:
/// <https://doc.rust-lang.org/reference/type-layout.html#reprc-structs>
const fn get_trailer_offset(
    header_size: usize,
    core_size: usize,
    core_align: usize,
    trailer_align: usize,
) -> usize {
    let mut offset = header_size;
    let core_misalign = offset % core_align;
    if core_misalign > 0 {
        offset += core_align - core_misalign;
    }
    offset += core_size;
    let trailer_misalign = offset % trailer_align;
    if trailer_misalign > 0 {
        offset += trailer_align - trailer_misalign;
    }
    offset
}
/// Compute the offset of the `Core<T, S>` field in `Cell<T, S>` using the
/// `#[repr(C)]` algorithm.
///
/// Pseudo-code for the `#[repr(C)]` algorithm can be found here:
/// <https://doc.rust-lang.org/reference/type-layout.html#reprc-structs>
const fn get_core_offset(header_size: usize, core_align: usize) -> usize {
    let mut offset = header_size;
    let core_misalign = offset % core_align;
    if core_misalign > 0 {
        offset += core_align - core_misalign;
    }
    offset
}
/// Compute the offset of the `Id` field in `Cell<T, S>` using the
/// `#[repr(C)]` algorithm.
///
/// Pseudo-code for the `#[repr(C)]` algorithm can be found here:
/// <https://doc.rust-lang.org/reference/type-layout.html#reprc-structs>
const fn get_id_offset(
    header_size: usize,
    core_align: usize,
    scheduler_size: usize,
    id_align: usize,
) -> usize {
    let mut offset = get_core_offset(header_size, core_align);
    offset += scheduler_size;
    let id_misalign = offset % id_align;
    if id_misalign > 0 {
        offset += id_align - id_misalign;
    }
    offset
}
/// Compute the offset of the `&'static Location<'static>` field in `Cell<T, S>`
/// using the `#[repr(C)]` algorithm.
///
/// Pseudo-code for the `#[repr(C)]` algorithm can be found here:
/// <https://doc.rust-lang.org/reference/type-layout.html#reprc-structs>
#[cfg(tokio_unstable)]
const fn get_spawn_location_offset(
    header_size: usize,
    core_align: usize,
    scheduler_size: usize,
    id_align: usize,
    id_size: usize,
    spawn_location_align: usize,
) -> usize {
    let mut offset = get_id_offset(header_size, core_align, scheduler_size, id_align);
    offset += id_size;
    let spawn_location_misalign = offset % spawn_location_align;
    if spawn_location_misalign > 0 {
        offset += spawn_location_align - spawn_location_misalign;
    }
    offset
}
impl RawTask {
    pub(super) fn new<T, S>(
        task: T,
        scheduler: S,
        id: Id,
        _spawned_at: super::SpawnLocation,
    ) -> RawTask
    where
        T: Future,
        S: Schedule,
    {
        panic!("STUB: not implemented");
    }
    /// # Safety
    ///
    /// `ptr` must be a valid pointer to a [`Header`].
    pub(super) unsafe fn from_raw(ptr: NonNull<Header>) -> RawTask {
        panic!("STUB: not implemented");
    }
    pub(super) fn header_ptr(&self) -> NonNull<Header> {
        panic!("STUB: not implemented");
    }
    cfg_taskdump! {
        pub (super) fn header_ptr_ref(& self) -> & NonNull < Header > { & self.ptr }
    }
    pub(super) fn trailer_ptr(&self) -> NonNull<Trailer> {
        panic!("STUB: not implemented");
    }
    /// Returns a reference to the task's header.
    pub(super) fn header(&self) -> &Header {
        panic!("STUB: not implemented");
    }
    /// Returns a reference to the task's trailer.
    pub(super) fn trailer(&self) -> &Trailer {
        panic!("STUB: not implemented");
    }
    /// Returns a reference to the task's state.
    pub(super) fn state(&self) -> &State {
        panic!("STUB: not implemented");
    }
    /// Safety: mutual exclusion is required to call this function.
    pub(crate) fn poll(self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn schedule(self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn dealloc(self) {
        panic!("STUB: not implemented");
    }
    /// Safety: `dst` must be a `*mut Poll<super::Result<T::Output>>` where `T`
    /// is the future stored by the task.
    pub(super) unsafe fn try_read_output<O>(
        self,
        dst: *mut Poll<super::Result<O>>,
        waker: &Waker,
    ) {
        panic!("STUB: not implemented");
    }
    pub(super) fn drop_join_handle_slow(self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn drop_abort_handle(self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn shutdown(self) {
        panic!("STUB: not implemented");
    }
    /// Increment the task's reference count.
    ///
    /// Currently, this is used only when creating an `AbortHandle`.
    pub(super) fn ref_inc(self) {
        panic!("STUB: not implemented");
    }
    /// Get the queue-next pointer
    ///
    /// This is for usage by the injection queue
    ///
    /// Safety: make sure only one queue uses this and access is synchronized.
    pub(crate) unsafe fn get_queue_next(self) -> Option<RawTask> {
        panic!("STUB: not implemented");
    }
    /// Sets the queue-next pointer
    ///
    /// This is for usage by the injection queue
    ///
    /// Safety: make sure only one queue uses this and access is synchronized.
    pub(crate) unsafe fn set_queue_next(self, val: Option<RawTask>) {
        panic!("STUB: not implemented");
    }
}
impl Copy for RawTask {}
unsafe fn poll<T: Future, S: Schedule>(ptr: NonNull<Header>) {
    panic!("STUB: not implemented");
}
unsafe fn schedule<S: Schedule>(ptr: NonNull<Header>) {
    panic!("STUB: not implemented");
}
unsafe fn dealloc<T: Future, S: Schedule>(ptr: NonNull<Header>) {
    panic!("STUB: not implemented");
}
unsafe fn try_read_output<T: Future, S: Schedule>(
    ptr: NonNull<Header>,
    dst: *mut (),
    waker: &Waker,
) {
    panic!("STUB: not implemented");
}
unsafe fn drop_join_handle_slow<T: Future, S: Schedule>(ptr: NonNull<Header>) {
    panic!("STUB: not implemented");
}
unsafe fn drop_abort_handle<T: Future, S: Schedule>(ptr: NonNull<Header>) {
    panic!("STUB: not implemented");
}
unsafe fn shutdown<T: Future, S: Schedule>(ptr: NonNull<Header>) {
    panic!("STUB: not implemented");
}
