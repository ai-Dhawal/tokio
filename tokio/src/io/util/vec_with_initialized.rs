use crate::io::ReadBuf;
use std::mem::MaybeUninit;
/// Something that looks like a `Vec<u8>`.
///
/// # Safety
///
/// The implementor must guarantee that the vector returned by the
/// `as_mut` and `as_mut` methods do not change from one call to
/// another.
pub(crate) unsafe trait VecU8: AsRef<Vec<u8>> + AsMut<Vec<u8>> {}
unsafe impl VecU8 for Vec<u8> {}
unsafe impl VecU8 for &mut Vec<u8> {}
/// This struct wraps a `Vec<u8>` or `&mut Vec<u8>`, combining it with a
/// `num_initialized`, which keeps track of the number of initialized bytes
/// in the unused capacity.
///
/// The purpose of this struct is to remember how many bytes were initialized
/// through a `ReadBuf` from call to call.
///
/// This struct has the safety invariant that the first `num_initialized` of the
/// vector's allocation must be initialized at any time.
#[derive(Debug)]
pub(crate) struct VecWithInitialized<V> {
    vec: V,
    num_initialized: usize,
    starting_capacity: usize,
}
impl VecWithInitialized<Vec<u8>> {
    #[cfg(feature = "io-util")]
    pub(crate) fn take(&mut self) -> Vec<u8> {
        panic!("STUB: not implemented");
    }
}
impl<V> VecWithInitialized<V>
where
    V: VecU8,
{
    pub(crate) fn new(mut vec: V) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn reserve(&mut self, num_bytes: usize) {
        panic!("STUB: not implemented");
    }
    #[cfg(feature = "io-util")]
    pub(crate) fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn get_read_buf<'a>(&'a mut self) -> ReadBuf<'a> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn apply_read_buf(&mut self, parts: ReadBufParts) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn try_small_read_first(&self, num_bytes: usize) -> bool {
        panic!("STUB: not implemented");
    }
}
pub(crate) struct ReadBufParts {
    ptr: *const u8,
    len: usize,
    initialized: usize,
}
pub(crate) fn into_read_buf_parts(rb: ReadBuf<'_>) -> ReadBufParts {
    panic!("STUB: not implemented");
}
