use std::fmt;
use std::mem::MaybeUninit;
/// A wrapper around a byte buffer that is incrementally filled and initialized.
///
/// This type is a sort of "double cursor". It tracks three regions in the
/// buffer: a region at the beginning of the buffer that has been logically
/// filled with data, a region that has been initialized at some point but not
/// yet logically filled, and a region at the end that may be uninitialized.
/// The filled region is guaranteed to be a subset of the initialized region.
///
/// In summary, the contents of the buffer can be visualized as:
///
/// ```not_rust
/// [             capacity              ]
/// [ filled |         unfilled         ]
/// [    initialized    | uninitialized ]
/// ```
///
/// It is undefined behavior to de-initialize any bytes from the uninitialized
/// region, since it is merely unknown whether this region is uninitialized or
/// not, and if part of it turns out to be initialized, it must stay initialized.
pub struct ReadBuf<'a> {
    buf: &'a mut [MaybeUninit<u8>],
    filled: usize,
    initialized: usize,
}
impl<'a> ReadBuf<'a> {
    /// Creates a new `ReadBuf` from a fully initialized buffer.
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> ReadBuf<'a> {
        panic!("STUB: not implemented");
    }
    /// Creates a new `ReadBuf` from a buffer that may be uninitialized.
    ///
    /// The internal cursor will mark the entire buffer as uninitialized. If
    /// the buffer is known to be partially initialized, then use `assume_init`
    /// to move the internal cursor.
    #[inline]
    pub fn uninit(buf: &'a mut [MaybeUninit<u8>]) -> ReadBuf<'a> {
        panic!("STUB: not implemented");
    }
    /// Returns the total capacity of the buffer.
    #[inline]
    pub fn capacity(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Returns a shared reference to the filled portion of the buffer.
    #[inline]
    pub fn filled(&self) -> &[u8] {
        panic!("STUB: not implemented");
    }
    /// Returns a mutable reference to the filled portion of the buffer.
    #[inline]
    pub fn filled_mut(&mut self) -> &mut [u8] {
        panic!("STUB: not implemented");
    }
    /// Returns a new `ReadBuf` comprised of the unfilled section up to `n`.
    #[inline]
    pub fn take(&mut self, n: usize) -> ReadBuf<'_> {
        panic!("STUB: not implemented");
    }
    /// Returns a shared reference to the initialized portion of the buffer.
    ///
    /// This includes the filled portion.
    #[inline]
    pub fn initialized(&self) -> &[u8] {
        panic!("STUB: not implemented");
    }
    /// Returns a mutable reference to the initialized portion of the buffer.
    ///
    /// This includes the filled portion.
    #[inline]
    pub fn initialized_mut(&mut self) -> &mut [u8] {
        panic!("STUB: not implemented");
    }
    /// Returns a mutable reference to the entire buffer, without ensuring that it has been fully
    /// initialized.
    ///
    /// The elements between 0 and `self.filled().len()` are filled, and those between 0 and
    /// `self.initialized().len()` are initialized (and so can be converted to a `&mut [u8]`).
    ///
    /// The caller of this method must ensure that these invariants are upheld. For example, if the
    /// caller initializes some of the uninitialized section of the buffer, it must call
    /// [`assume_init`](Self::assume_init) with the number of bytes initialized.
    ///
    /// # Safety
    ///
    /// The caller must not de-initialize portions of the buffer that have already been initialized.
    /// This includes any bytes in the region marked as uninitialized by `ReadBuf`.
    #[inline]
    pub unsafe fn inner_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        panic!("STUB: not implemented");
    }
    /// Returns a mutable reference to the unfilled part of the buffer without ensuring that it has been fully
    /// initialized.
    ///
    /// # Safety
    ///
    /// The caller must not de-initialize portions of the buffer that have already been initialized.
    /// This includes any bytes in the region marked as uninitialized by `ReadBuf`.
    #[inline]
    pub unsafe fn unfilled_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        panic!("STUB: not implemented");
    }
    /// Returns a mutable reference to the unfilled part of the buffer, ensuring it is fully initialized.
    ///
    /// Since `ReadBuf` tracks the region of the buffer that has been initialized, this is effectively "free" after
    /// the first use.
    #[inline]
    pub fn initialize_unfilled(&mut self) -> &mut [u8] {
        panic!("STUB: not implemented");
    }
    /// Returns a mutable reference to the first `n` bytes of the unfilled part of the buffer, ensuring it is
    /// fully initialized.
    ///
    /// # Panics
    ///
    /// Panics if `self.remaining()` is less than `n`.
    #[inline]
    #[track_caller]
    pub fn initialize_unfilled_to(&mut self, n: usize) -> &mut [u8] {
        panic!("STUB: not implemented");
    }
    /// Returns the number of bytes at the end of the slice that have not yet been filled.
    #[inline]
    pub fn remaining(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Clears the buffer, resetting the filled region to empty.
    ///
    /// The number of initialized bytes is not changed, and the contents of the buffer are not modified.
    #[inline]
    pub fn clear(&mut self) {
        panic!("STUB: not implemented");
    }
    /// Advances the size of the filled region of the buffer.
    ///
    /// The number of initialized bytes is not changed.
    ///
    /// # Panics
    ///
    /// Panics if the filled region of the buffer would become larger than the initialized region.
    #[inline]
    #[track_caller]
    pub fn advance(&mut self, n: usize) {
        panic!("STUB: not implemented");
    }
    /// Sets the size of the filled region of the buffer.
    ///
    /// The number of initialized bytes is not changed.
    ///
    /// Note that this can be used to *shrink* the filled region of the buffer in addition to growing it (for
    /// example, by a `AsyncRead` implementation that compresses data in-place).
    ///
    /// # Panics
    ///
    /// Panics if the filled region of the buffer would become larger than the initialized region.
    #[inline]
    #[track_caller]
    pub fn set_filled(&mut self, n: usize) {
        panic!("STUB: not implemented");
    }
    /// Asserts that the first `n` unfilled bytes of the buffer are initialized.
    ///
    /// `ReadBuf` assumes that bytes are never de-initialized, so this method does nothing when called with fewer
    /// bytes than are already known to be initialized.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `n` unfilled bytes of the buffer have already been initialized.
    #[inline]
    pub unsafe fn assume_init(&mut self, n: usize) {
        panic!("STUB: not implemented");
    }
    /// Appends data to the buffer, advancing the written position and possibly also the initialized position.
    ///
    /// # Panics
    ///
    /// Panics if `self.remaining()` is less than `buf.len()`.
    #[inline]
    #[track_caller]
    pub fn put_slice(&mut self, buf: &[u8]) {
        panic!("STUB: not implemented");
    }
}
#[cfg(feature = "io-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
unsafe impl<'a> bytes::BufMut for ReadBuf<'a> {
    fn remaining_mut(&self) -> usize {
        panic!("STUB: not implemented");
    }
    unsafe fn advance_mut(&mut self, cnt: usize) {
        panic!("STUB: not implemented");
    }
    fn chunk_mut(&mut self) -> &mut bytes::buf::UninitSlice {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for ReadBuf<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
/// # Safety
///
/// The caller must ensure that `slice` is fully initialized
/// and never writes uninitialized bytes to the returned slice.
unsafe fn slice_to_uninit_mut(slice: &mut [u8]) -> &mut [MaybeUninit<u8>] {
    panic!("STUB: not implemented");
}
/// # Safety
///
/// The caller must ensure that `slice` is fully initialized.
unsafe fn slice_assume_init(slice: &[MaybeUninit<u8>]) -> &[u8] {
    panic!("STUB: not implemented");
}
/// # Safety
///
/// The caller must ensure that `slice` is fully initialized.
unsafe fn slice_assume_init_mut(slice: &mut [MaybeUninit<u8>]) -> &mut [u8] {
    panic!("STUB: not implemented");
}
