use super::Synced;
use crate::runtime::task;
use std::marker::PhantomData;
pub(crate) struct Pop<'a, T: 'static> {
    len: usize,
    synced: &'a mut Synced,
    _p: PhantomData<T>,
}
impl<'a, T: 'static> Pop<'a, T> {
    pub(super) fn new(len: usize, synced: &'a mut Synced) -> Pop<'a, T> {
        panic!("STUB: not implemented");
    }
}
impl<'a, T: 'static> Iterator for Pop<'a, T> {
    type Item = task::Notified<T>;
    fn next(&mut self) -> Option<Self::Item> {
        panic!("STUB: not implemented");
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        panic!("STUB: not implemented");
    }
}
impl<'a, T: 'static> ExactSizeIterator for Pop<'a, T> {
    fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
}
impl<'a, T: 'static> Drop for Pop<'a, T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
