use crate::signal::unix::{OsExtraData, OsStorage};
use crate::sync::watch;
use std::ops;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
pub(crate) type EventId = usize;
/// State for a specific event, whether a notification is pending delivery,
/// and what listeners are registered.
#[derive(Debug)]
pub(crate) struct EventInfo {
    pending: AtomicBool,
    tx: watch::Sender<()>,
}
impl Default for EventInfo {
    fn default() -> Self {
        panic!("STUB: not implemented");
    }
}
/// An interface for retrieving the `EventInfo` for a particular `eventId`.
pub(crate) trait Storage {
    /// Gets the `EventInfo` for `id` if it exists.
    fn event_info(&self, id: EventId) -> Option<&EventInfo>;
    /// Invokes `f` once for each defined `EventInfo` in this storage.
    fn for_each<'a, F>(&'a self, f: F)
    where
        F: FnMut(&'a EventInfo);
}
impl Storage for Vec<EventInfo> {
    fn event_info(&self, id: EventId) -> Option<&EventInfo> {
        panic!("STUB: not implemented");
    }
    fn for_each<'a, F>(&'a self, f: F)
    where
        F: FnMut(&'a EventInfo),
    {
        panic!("STUB: not implemented");
    }
}
/// Manages and distributes event notifications to any registered listeners.
///
/// Generic over the underlying storage to allow for domain specific
/// optimizations (e.g. `eventIds` may or may not be contiguous).
#[derive(Debug)]
pub(crate) struct Registry<S> {
    storage: S,
}
impl<S> Registry<S> {
    fn new(storage: S) -> Self {
        panic!("STUB: not implemented");
    }
}
impl<S: Storage> Registry<S> {
    /// Registers a new listener for `event_id`.
    fn register_listener(&self, event_id: EventId) -> watch::Receiver<()> {
        panic!("STUB: not implemented");
    }
    /// Marks `event_id` as having been delivered, without broadcasting it to
    /// any listeners.
    fn record_event(&self, event_id: EventId) {
        panic!("STUB: not implemented");
    }
    /// Broadcasts all previously recorded events to their respective listeners.
    ///
    /// Returns `true` if an event was delivered to at least one listener.
    fn broadcast(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
pub(crate) struct Globals {
    extra: OsExtraData,
    registry: Registry<OsStorage>,
}
impl ops::Deref for Globals {
    type Target = OsExtraData;
    fn deref(&self) -> &Self::Target {
        panic!("STUB: not implemented");
    }
}
impl Globals {
    /// Registers a new listener for `event_id`.
    pub(crate) fn register_listener(&self, event_id: EventId) -> watch::Receiver<()> {
        panic!("STUB: not implemented");
    }
    /// Marks `event_id` as having been delivered, without broadcasting it to
    /// any listeners.
    pub(crate) fn record_event(&self, event_id: EventId) {
        panic!("STUB: not implemented");
    }
    /// Broadcasts all previously recorded events to their respective listeners.
    ///
    /// Returns `true` if an event was delivered to at least one listener.
    pub(crate) fn broadcast(&self) -> bool {
        panic!("STUB: not implemented");
    }
    #[cfg(unix)]
    pub(crate) fn storage(&self) -> &OsStorage {
        panic!("STUB: not implemented");
    }
}
fn globals_init() -> Globals
where
    OsExtraData: 'static + Send + Sync + Default,
    OsStorage: 'static + Send + Sync + Default,
{
    panic!("STUB: not implemented");
}
pub(crate) fn globals() -> &'static Globals
where
    OsExtraData: 'static + Send + Sync + Default,
    OsStorage: 'static + Send + Sync + Default,
{
    panic!("STUB: not implemented");
}
#[cfg(all(test, not(loom)))]
mod tests {
    use super::*;
    use crate::runtime::{self, Runtime};
    use crate::sync::{oneshot, watch};
    use futures::future;
    #[test]
    fn smoke() {
        let rt = rt();
        rt.block_on(async move {
            let registry = Registry::new(
                vec![EventInfo::default(), EventInfo::default(), EventInfo::default(),],
            );
            let first = registry.register_listener(0);
            let second = registry.register_listener(1);
            let third = registry.register_listener(2);
            let (fire, wait) = oneshot::channel();
            crate::spawn(async {
                wait.await.expect("wait failed");
                registry.record_event(0);
                registry.record_event(0);
                registry.record_event(1);
                registry.record_event(1);
                registry.broadcast();
                for _ in 0..100 {
                    crate::task::yield_now().await;
                }
                registry.record_event(0);
                registry.broadcast();
                drop(registry);
            });
            let _ = fire.send(());
            let all = future::join3(collect(first), collect(second), collect(third));
            let (first_results, second_results, third_results) = all.await;
            assert_eq!(2, first_results.len());
            assert_eq!(1, second_results.len());
            assert_eq!(0, third_results.len());
        });
    }
    #[test]
    #[should_panic = "invalid event_id: 1"]
    fn register_panics_on_invalid_input() {
        let registry = Registry::new(vec![EventInfo::default()]);
        registry.register_listener(1);
    }
    #[test]
    fn record_invalid_event_does_nothing() {
        let registry = Registry::new(vec![EventInfo::default()]);
        registry.record_event(1302);
    }
    #[test]
    fn broadcast_returns_if_at_least_one_event_fired() {
        let registry = Registry::new(vec![EventInfo::default(), EventInfo::default()]);
        registry.record_event(0);
        assert!(! registry.broadcast());
        let first = registry.register_listener(0);
        let second = registry.register_listener(1);
        registry.record_event(0);
        assert!(registry.broadcast());
        drop(first);
        registry.record_event(0);
        assert!(! registry.broadcast());
        drop(second);
    }
    fn rt() -> Runtime {
        runtime::Builder::new_current_thread().enable_time().build().unwrap()
    }
    async fn collect(mut rx: watch::Receiver<()>) -> Vec<()> {
        let mut ret = vec![];
        while let Ok(v) = rx.changed().await {
            ret.push(v);
        }
        ret
    }
}
