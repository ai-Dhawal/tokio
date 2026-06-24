//! This module defines an `IdleNotifiedSet`, which is a collection of elements.
//! Each element is intended to correspond to a task, and the collection will
//! keep track of which tasks have had their waker notified, and which have not.
//!
//! Each entry in the set holds some user-specified value. The value's type is
//! specified using the `T` parameter. It will usually be a `JoinHandle` or
//! similar.
use std::marker::PhantomPinned;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;
use std::task::{Context, Waker};
use crate::loom::cell::UnsafeCell;
use crate::loom::sync::{Arc, Mutex};
use crate::util::linked_list::{self, Link, LinkedList};
use crate::util::{waker_ref, Wake};
/// This is the main handle to the collection.
pub(crate) struct IdleNotifiedSet<T> {
    lists: Arc<Lists<T>>,
    length: usize,
}
/// A handle to an entry that is guaranteed to be stored in the idle or notified
/// list of its `IdleNotifiedSet`. This value borrows the `IdleNotifiedSet`
/// mutably to prevent the entry from being moved to the `Neither` list, which
/// only the `IdleNotifiedSet` may do.
///
/// The main consequence of being stored in one of the lists is that the `value`
/// field has not yet been consumed.
///
/// Note: This entry can be moved from the idle to the notified list while this
/// object exists by waking its waker.
pub(crate) struct EntryInOneOfTheLists<'a, T> {
    entry: Arc<ListEntry<T>>,
    set: &'a mut IdleNotifiedSet<T>,
}
type Lists<T> = Mutex<ListsInner<T>>;
/// The linked lists hold strong references to the `ListEntry` items, and the
/// `ListEntry` items also hold a strong reference back to the Lists object, but
/// the destructor of the `IdleNotifiedSet` will clear the two lists, so once
/// that object is destroyed, no ref-cycles will remain.
struct ListsInner<T> {
    notified: LinkedList<ListEntry<T>>,
    idle: LinkedList<ListEntry<T>>,
    /// Whenever an element in the `notified` list is woken, this waker will be
    /// notified and consumed, if it exists.
    waker: Option<Waker>,
}
/// Which of the two lists in the shared Lists object is this entry stored in?
///
/// If the value is `Idle`, then an entry's waker may move it to the notified
/// list. Otherwise, only the `IdleNotifiedSet` may move it.
///
/// If the value is `Neither`, then it is still possible that the entry is in
/// some third external list (this happens in `drain`).
#[derive(Copy, Clone, Eq, PartialEq)]
enum List {
    Notified,
    Idle,
    Neither,
}
/// An entry in the list.
///
/// # Safety
///
/// The `my_list` field must only be accessed while holding the mutex in
/// `parent`. It is an invariant that the value of `my_list` corresponds to
/// which linked list in the `parent` holds this entry. Once this field takes
/// the value `Neither`, then it may never be modified again.
///
/// If the value of `my_list` is `Notified` or `Idle`, then the `pointers` field
/// must only be accessed while holding the mutex. If the value of `my_list` is
/// `Neither`, then the `pointers` field may be accessed by the
/// `IdleNotifiedSet` (this happens inside `drain`).
///
/// The `value` field is owned by the `IdleNotifiedSet` and may only be accessed
/// by the `IdleNotifiedSet`. The operation that sets the value of `my_list` to
/// `Neither` assumes ownership of the `value`, and it must either drop it or
/// move it out from this entry to prevent it from getting leaked. (Since the
/// two linked lists are emptied in the destructor of `IdleNotifiedSet`, the
/// value should not be leaked.)
struct ListEntry<T> {
    /// The linked list pointers of the list this entry is in.
    pointers: linked_list::Pointers<ListEntry<T>>,
    /// Pointer to the shared `Lists` struct.
    parent: Arc<Lists<T>>,
    /// The value stored in this entry.
    value: UnsafeCell<ManuallyDrop<T>>,
    /// Used to remember which list this entry is in.
    my_list: UnsafeCell<List>,
    /// Required by the `linked_list::Pointers` field.
    _pin: PhantomPinned,
}
generate_addr_of_methods! {
    impl < T > ListEntry < T > { unsafe fn addr_of_pointers(self : NonNull < Self >) ->
    NonNull < linked_list::Pointers < ListEntry < T >>> { & self.pointers } }
}
unsafe impl<T: Send> Send for IdleNotifiedSet<T> {}
unsafe impl<T: Sync> Sync for IdleNotifiedSet<T> {}
unsafe impl<T> Send for ListEntry<T> {}
unsafe impl<T> Sync for ListEntry<T> {}
impl<T> IdleNotifiedSet<T> {
    /// Create a new `IdleNotifiedSet`.
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Insert the given value into the `idle` list.
    pub(crate) fn insert_idle(&mut self, value: T) -> EntryInOneOfTheLists<'_, T> {
        panic!("STUB: not implemented");
    }
    /// Pop an entry from the notified list to poll it. The entry is moved to
    /// the idle list atomically.
    pub(crate) fn pop_notified(
        &mut self,
        waker: &Waker,
    ) -> Option<EntryInOneOfTheLists<'_, T>> {
        panic!("STUB: not implemented");
    }
    /// Tries to pop an entry from the notified list to poll it. The entry is moved to
    /// the idle list atomically.
    pub(crate) fn try_pop_notified(&mut self) -> Option<EntryInOneOfTheLists<'_, T>> {
        panic!("STUB: not implemented");
    }
    /// Call a function on every element in this list.
    pub(crate) fn for_each<F: FnMut(&mut T)>(&mut self, mut func: F) {
        panic!("STUB: not implemented");
    }
    /// Remove all entries in both lists, applying some function to each element.
    ///
    /// The closure is called on all elements even if it panics. Having it panic
    /// twice is a double-panic, and will abort the application.
    pub(crate) fn drain<F: FnMut(T)>(&mut self, func: F) {
        panic!("STUB: not implemented");
    }
}
/// # Safety
///
/// The mutex for the entries must be held, and the target list must be such
/// that setting `my_list` to `Neither` is ok.
unsafe fn move_to_new_list<T>(
    from: &mut LinkedList<ListEntry<T>>,
    to: &mut LinkedList<ListEntry<T>>,
) {
    panic!("STUB: not implemented");
}
impl<'a, T> EntryInOneOfTheLists<'a, T> {
    /// Remove this entry from the list it is in, returning the value associated
    /// with the entry.
    ///
    /// This consumes the value, since it is no longer guaranteed to be in a
    /// list.
    pub(crate) fn remove(self) -> T {
        panic!("STUB: not implemented");
    }
    /// Access the value in this entry together with a context for its waker.
    pub(crate) fn with_value_and_context<F, U>(&mut self, func: F) -> U
    where
        F: FnOnce(&mut T, &mut Context<'_>) -> U,
        T: 'static,
    {
        panic!("STUB: not implemented");
    }
}
impl<T> Drop for IdleNotifiedSet<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T: 'static> Wake for ListEntry<T> {
    fn wake_by_ref(me: &Arc<Self>) {
        panic!("STUB: not implemented");
    }
    fn wake(me: Arc<Self>) {
        panic!("STUB: not implemented");
    }
}
/// # Safety
///
/// `ListEntry` is forced to be !Unpin.
unsafe impl<T> linked_list::Link for ListEntry<T> {
    type Handle = Arc<ListEntry<T>>;
    type Target = ListEntry<T>;
    fn as_raw(handle: &Self::Handle) -> NonNull<ListEntry<T>> {
        panic!("STUB: not implemented");
    }
    unsafe fn from_raw(ptr: NonNull<ListEntry<T>>) -> Arc<ListEntry<T>> {
        panic!("STUB: not implemented");
    }
    unsafe fn pointers(
        target: NonNull<ListEntry<T>>,
    ) -> NonNull<linked_list::Pointers<ListEntry<T>>> {
        panic!("STUB: not implemented");
    }
}
#[cfg(all(test, not(loom)))]
mod tests {
    use crate::runtime::Builder;
    use crate::task::JoinSet;
    #[test]
    fn join_set_test() {
        let rt = Builder::new_current_thread().build().unwrap();
        let mut set = JoinSet::new();
        set.spawn_on(futures::future::ready(()), rt.handle());
        rt.block_on(set.join_next()).unwrap().unwrap();
    }
}
