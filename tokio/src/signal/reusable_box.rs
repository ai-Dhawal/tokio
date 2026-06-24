use std::alloc::Layout;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;
use std::ptr::{self, NonNull};
use std::task::{Context, Poll};
use std::{fmt, panic};
/// A reusable `Pin<Box<dyn Future<Output = T> + Send>>`.
///
/// This type lets you replace the future stored in the box without
/// reallocating when the size and alignment permits this.
pub(crate) struct ReusableBoxFuture<T> {
    boxed: NonNull<dyn Future<Output = T> + Send>,
}
impl<T> ReusableBoxFuture<T> {
    /// Create a new `ReusableBoxFuture<T>` containing the provided future.
    pub(crate) fn new<F>(future: F) -> Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Replaces the future currently stored in this box.
    ///
    /// This reallocates if and only if the layout of the provided future is
    /// different from the layout of the currently stored future.
    pub(crate) fn set<F>(&mut self, future: F)
    where
        F: Future<Output = T> + Send + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Replaces the future currently stored in this box.
    ///
    /// This function never reallocates, but returns an error if the provided
    /// future has a different size or alignment from the currently stored
    /// future.
    pub(crate) fn try_set<F>(&mut self, future: F) -> Result<(), F>
    where
        F: Future<Output = T> + Send + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Sets the current future.
    ///
    /// # Safety
    ///
    /// This function requires that the layout of the provided future is the
    /// same as `self.layout`.
    unsafe fn set_same_layout<F>(&mut self, future: F)
    where
        F: Future<Output = T> + Send + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Gets a pinned reference to the underlying future.
    pub(crate) fn get_pin(&mut self) -> Pin<&mut (dyn Future<Output = T> + Send)> {
        panic!("STUB: not implemented");
    }
    /// Polls the future stored inside this box.
    pub(crate) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<T> {
        panic!("STUB: not implemented");
    }
}
impl<T> Future for ReusableBoxFuture<T> {
    type Output = T;
    /// Polls the future stored inside this box.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        panic!("STUB: not implemented");
    }
}
unsafe impl<T> Send for ReusableBoxFuture<T> {}
unsafe impl<T> Sync for ReusableBoxFuture<T> {}
impl<T> Unpin for ReusableBoxFuture<T> {}
impl<T> Drop for ReusableBoxFuture<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T> fmt::Debug for ReusableBoxFuture<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod test {
    use super::ReusableBoxFuture;
    use futures::future::FutureExt;
    use std::alloc::Layout;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    #[test]
    fn test_different_futures() {
        let fut = async move { 10 };
        assert_eq!(Layout::for_value(& fut).size(), 1);
        let mut b = ReusableBoxFuture::new(fut);
        assert_eq!(b.get_pin().now_or_never(), Some(10));
        b.try_set(async move { 20 }).unwrap_or_else(|_| panic!("incorrect size"));
        assert_eq!(b.get_pin().now_or_never(), Some(20));
        b.try_set(async move { 30 }).unwrap_or_else(|_| panic!("incorrect size"));
        assert_eq!(b.get_pin().now_or_never(), Some(30));
    }
    #[test]
    fn test_different_sizes() {
        let fut1 = async move { 10 };
        let val = [0u32; 1000];
        let fut2 = async move { val[0] };
        let fut3 = ZeroSizedFuture {};
        assert_eq!(Layout::for_value(& fut1).size(), 1);
        assert_eq!(Layout::for_value(& fut2).size(), 4004);
        assert_eq!(Layout::for_value(& fut3).size(), 0);
        let mut b = ReusableBoxFuture::new(fut1);
        assert_eq!(b.get_pin().now_or_never(), Some(10));
        b.set(fut2);
        assert_eq!(b.get_pin().now_or_never(), Some(0));
        b.set(fut3);
        assert_eq!(b.get_pin().now_or_never(), Some(5));
    }
    struct ZeroSizedFuture {}
    impl Future for ZeroSizedFuture {
        type Output = u32;
        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<u32> {
            Poll::Ready(5)
        }
    }
    #[test]
    fn test_zero_sized() {
        let fut = ZeroSizedFuture {};
        assert_eq!(Layout::for_value(& fut).size(), 0);
        let mut b = ReusableBoxFuture::new(fut);
        assert_eq!(b.get_pin().now_or_never(), Some(5));
        assert_eq!(b.get_pin().now_or_never(), Some(5));
        b.try_set(ZeroSizedFuture {}).unwrap_or_else(|_| panic!("incorrect size"));
        assert_eq!(b.get_pin().now_or_never(), Some(5));
        assert_eq!(b.get_pin().now_or_never(), Some(5));
    }
}
