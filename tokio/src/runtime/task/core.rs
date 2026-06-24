//! Core task module.
//!
//! # Safety
//!
//! The functions in this module are private to the `task` module. All of them
//! should be considered `unsafe` to use, but are not marked as such since it
//! would be too noisy.
//!
//! Make sure to consult the relevant safety section of each function before
//! use.
#![allow(unsafe_op_in_unsafe_fn)]
use crate::future::Future;
use crate::loom::cell::UnsafeCell;
use crate::runtime::context;
use crate::runtime::metrics::ScheduleLatencyInstant;
use crate::runtime::task::raw::{self, Vtable};
use crate::runtime::task::state::State;
use crate::runtime::task::{Id, Schedule, TaskHarnessScheduleHooks};
use crate::util::linked_list;
use std::num::NonZeroU64;
#[cfg(tokio_unstable)]
use std::panic::Location;
use std::pin::Pin;
use std::ptr::NonNull;
use std::task::{Context, Poll, Waker};
/// The task cell. Contains the components of the task.
///
/// It is critical for `Header` to be the first field as the task structure will
/// be referenced by both *mut Cell and *mut Header.
///
/// Any changes to the layout of this struct _must_ also be reflected in the
/// `const` fns in raw.rs.
///
#[cfg_attr(
    any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "powerpc64"),
    repr(align(128))
)]
#[cfg_attr(
    any(
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "sparc",
        target_arch = "hexagon",
    ),
    repr(align(32))
)]
#[cfg_attr(target_arch = "m68k", repr(align(16)))]
#[cfg_attr(target_arch = "s390x", repr(align(256)))]
#[cfg_attr(
    not(
        any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "powerpc64",
            target_arch = "arm",
            target_arch = "mips",
            target_arch = "mips64",
            target_arch = "sparc",
            target_arch = "hexagon",
            target_arch = "m68k",
            target_arch = "s390x",
        )
    ),
    repr(align(64))
)]
#[repr(C)]
pub(super) struct Cell<T: Future, S> {
    /// Hot task state data
    pub(super) header: Header,
    /// Either the future or output, depending on the execution stage.
    pub(super) core: Core<T, S>,
    /// Cold data
    pub(super) trailer: Trailer,
}
pub(super) struct CoreStage<T: Future> {
    stage: UnsafeCell<Stage<T>>,
}
/// The core of the task.
///
/// Holds the future or output, depending on the stage of execution.
///
/// Any changes to the layout of this struct _must_ also be reflected in the
/// `const` fns in raw.rs.
#[repr(C)]
pub(super) struct Core<T: Future, S> {
    /// Scheduler used to drive this future.
    pub(super) scheduler: S,
    /// The task's ID, used for populating `JoinError`s.
    pub(super) task_id: Id,
    /// The source code location where the task was spawned.
    ///
    /// This is used for populating the `TaskMeta` passed to the task runtime
    /// hooks.
    #[cfg(tokio_unstable)]
    pub(super) spawned_at: &'static Location<'static>,
    /// Either the future or the output.
    pub(super) stage: CoreStage<T>,
}
/// Crate public as this is also needed by the pool.
#[repr(C)]
pub(crate) struct Header {
    /// Task state.
    pub(super) state: State,
    /// Pointer to next task, used with the injection queue.
    pub(super) queue_next: UnsafeCell<Option<NonNull<Header>>>,
    /// Table of function pointers for executing actions on the task.
    pub(super) vtable: &'static Vtable,
    /// This integer contains the id of the `OwnedTasks` or `LocalOwnedTasks`
    /// that this task is stored in. If the task is not in any list, should be
    /// the id of the list that it was previously in, or `None` if it has never
    /// been in any list.
    ///
    /// Once a task has been bound to a list, it can never be bound to another
    /// list, even if removed from the first list.
    ///
    /// The id is not unset when removed from a list because we want to be able
    /// to read the id without synchronization, even if it is concurrently being
    /// removed from the list.
    pub(super) owner_id: UnsafeCell<Option<NonZeroU64>>,
    /// The tracing ID for this instrumented task.
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    pub(super) tracing_id: Option<tracing::Id>,
    /// The last time this task was scheduled. Used to measure schedule latency.
    pub(super) scheduled_at: UnsafeCell<ScheduleLatencyInstant>,
}
unsafe impl Send for Header {}
unsafe impl Sync for Header {}
/// Cold data is stored after the future. Data is considered cold if it is only
/// used during creation or shutdown of the task.
pub(super) struct Trailer {
    /// Pointers for the linked list in the `OwnedTasks` that owns this task.
    pub(super) owned: linked_list::Pointers<Header>,
    /// Consumer task waiting on completion of this task.
    pub(super) waker: UnsafeCell<Option<Waker>>,
    /// Optional hooks needed in the harness.
    #[cfg_attr(not(tokio_unstable), allow(dead_code))]
    pub(super) hooks: TaskHarnessScheduleHooks,
}
generate_addr_of_methods! {
    impl <> Trailer { pub (super) unsafe fn addr_of_owned(self : NonNull < Self >) ->
    NonNull < linked_list::Pointers < Header >> { & self.owned } }
}
/// Either the future or the output.
#[repr(C)]
pub(super) enum Stage<T: Future> {
    Running(T),
    Finished(super::Result<T::Output>),
    Consumed,
}
impl<T: Future, S: Schedule> Cell<T, S> {
    /// Allocates a new task cell, containing the header, trailer, and core
    /// structures.
    pub(super) fn new(
        future: T,
        scheduler: S,
        state: State,
        task_id: Id,
        #[cfg(tokio_unstable)]
        spawned_at: &'static Location<'static>,
    ) -> Box<Cell<T, S>> {
        panic!("STUB: not implemented");
    }
}
impl<T: Future> CoreStage<T> {
    pub(super) fn with_mut<R>(&self, f: impl FnOnce(*mut Stage<T>) -> R) -> R {
        panic!("STUB: not implemented");
    }
}
/// Set and clear the task id in the context when the future is executed or
/// dropped, or when the output produced by the future is dropped.
pub(crate) struct TaskIdGuard {
    parent_task_id: Option<Id>,
}
impl TaskIdGuard {
    fn enter(id: Id) -> Self {
        panic!("STUB: not implemented");
    }
}
impl Drop for TaskIdGuard {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T: Future, S: Schedule> Core<T, S> {
    /// Polls the future.
    ///
    /// # Safety
    ///
    /// The caller must ensure it is safe to mutate the `state` field. This
    /// requires ensuring mutual exclusion between any concurrent thread that
    /// might modify the future or output field.
    ///
    /// The mutual exclusion is implemented by `Harness` and the `Lifecycle`
    /// component of the task state.
    ///
    /// `self` must also be pinned. This is handled by storing the task on the
    /// heap.
    pub(super) fn poll(&self, mut cx: Context<'_>) -> Poll<T::Output> {
        panic!("STUB: not implemented");
    }
    /// Drops the future.
    ///
    /// # Safety
    ///
    /// The caller must ensure it is safe to mutate the `stage` field.
    pub(super) fn drop_future_or_output(&self) {
        panic!("STUB: not implemented");
    }
    /// Stores the task output.
    ///
    /// # Safety
    ///
    /// The caller must ensure it is safe to mutate the `stage` field.
    pub(super) fn store_output(&self, output: super::Result<T::Output>) {
        panic!("STUB: not implemented");
    }
    /// Takes the task output.
    ///
    /// # Safety
    ///
    /// The caller must ensure it is safe to mutate the `stage` field.
    pub(super) fn take_output(&self) -> super::Result<T::Output> {
        panic!("STUB: not implemented");
    }
    unsafe fn set_stage(&self, stage: Stage<T>) {
        panic!("STUB: not implemented");
    }
}
impl Header {
    pub(super) unsafe fn set_next(&self, next: Option<NonNull<Header>>) {
        panic!("STUB: not implemented");
    }
    pub(super) unsafe fn set_owner_id(&self, owner: NonZeroU64) {
        panic!("STUB: not implemented");
    }
    pub(super) fn get_owner_id(&self) -> Option<NonZeroU64> {
        panic!("STUB: not implemented");
    }
    /// Gets a pointer to the `Trailer` of the task containing this `Header`.
    ///
    /// # Safety
    ///
    /// The provided raw pointer must point at the header of a task.
    pub(super) unsafe fn get_trailer(me: NonNull<Header>) -> NonNull<Trailer> {
        panic!("STUB: not implemented");
    }
    /// Gets a pointer to the scheduler of the task containing this `Header`.
    ///
    /// # Safety
    ///
    /// The provided raw pointer must point at the header of a task.
    ///
    /// The generic type S must be set to the correct scheduler type for this
    /// task.
    pub(super) unsafe fn get_scheduler<S>(me: NonNull<Header>) -> NonNull<S> {
        panic!("STUB: not implemented");
    }
    /// Gets a pointer to the id of the task containing this `Header`.
    ///
    /// # Safety
    ///
    /// The provided raw pointer must point at the header of a task.
    pub(super) unsafe fn get_id_ptr(me: NonNull<Header>) -> NonNull<Id> {
        panic!("STUB: not implemented");
    }
    /// Gets the id of the task containing this `Header`.
    ///
    /// # Safety
    ///
    /// The provided raw pointer must point at the header of a task.
    pub(super) unsafe fn get_id(me: NonNull<Header>) -> Id {
        panic!("STUB: not implemented");
    }
    /// Gets a pointer to the source code location where the task containing
    /// this `Header` was spawned.
    ///
    /// # Safety
    ///
    /// The provided raw pointer must point at the header of a task.
    #[cfg(tokio_unstable)]
    pub(super) unsafe fn get_spawn_location_ptr(
        me: NonNull<Header>,
    ) -> NonNull<&'static Location<'static>> {
        panic!("STUB: not implemented");
    }
    /// Gets the source code location where the task containing
    /// this `Header` was spawned
    ///
    /// # Safety
    ///
    /// The provided raw pointer must point at the header of a task.
    #[cfg(tokio_unstable)]
    pub(super) unsafe fn get_spawn_location(
        me: NonNull<Header>,
    ) -> &'static Location<'static> {
        panic!("STUB: not implemented");
    }
    /// Gets the tracing id of the task containing this `Header`.
    ///
    /// # Safety
    ///
    /// The provided raw pointer must point at the header of a task.
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    pub(super) unsafe fn get_tracing_id(me: &NonNull<Header>) -> Option<&tracing::Id> {
        panic!("STUB: not implemented");
    }
    /// Updates the last time this task was scheduled. Used to calculate
    /// the time elapsed between task scheduling and polling.
    ///
    /// # Safety
    ///
    /// The caller must guarantee exclusive access to this field.
    pub(super) unsafe fn set_scheduled_at(&self, scheduled_at: ScheduleLatencyInstant) {
        panic!("STUB: not implemented");
    }
    /// Gets the last time this task was scheduled.
    pub(super) fn get_scheduled_at(&self) -> ScheduleLatencyInstant {
        panic!("STUB: not implemented");
    }
}
impl Trailer {
    fn new(hooks: TaskHarnessScheduleHooks) -> Self {
        panic!("STUB: not implemented");
    }
    pub(super) unsafe fn set_waker(&self, waker: Option<Waker>) {
        panic!("STUB: not implemented");
    }
    pub(super) unsafe fn will_wake(&self, waker: &Waker) -> bool {
        panic!("STUB: not implemented");
    }
    pub(super) fn wake_join(&self) {
        panic!("STUB: not implemented");
    }
}
#[test]
#[cfg(not(loom))]
fn header_lte_cache_line() {
    assert!(std::mem::size_of::< Header > () <= 8 * std::mem::size_of::<* const () > ());
}
