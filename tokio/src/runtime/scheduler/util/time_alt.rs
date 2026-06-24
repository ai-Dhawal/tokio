use crate::runtime::scheduler::driver;
use crate::runtime::time_alt::cancellation_queue::{Receiver, Sender};
use crate::runtime::time_alt::{EntryHandle, RegistrationQueue, WakeQueue, Wheel};
use std::time::Duration;
pub(crate) fn min_duration(
    a: Option<Duration>,
    b: Option<Duration>,
) -> Option<Duration> {
    panic!("STUB: not implemented");
}
pub(crate) fn process_registration_queue(
    registration_queue: &mut RegistrationQueue,
    wheel: &mut Wheel,
    tx: &Sender,
    wake_queue: &mut WakeQueue,
) {
    panic!("STUB: not implemented");
}
pub(crate) fn insert_inject_timers(
    wheel: &mut Wheel,
    tx: &Sender,
    inject: Vec<EntryHandle>,
    wake_queue: &mut WakeQueue,
) {
    panic!("STUB: not implemented");
}
pub(crate) fn remove_cancelled_timers(wheel: &mut Wheel, rx: &mut Receiver) {
    panic!("STUB: not implemented");
}
pub(crate) fn next_expiration_time(
    wheel: &Wheel,
    drv_hdl: &driver::Handle,
) -> Option<Duration> {
    panic!("STUB: not implemented");
}
#[cfg(feature = "test-util")]
pub(crate) fn pre_auto_advance(
    drv_hdl: &driver::Handle,
    duration: Option<Duration>,
) -> bool {
    drv_hdl
        .with_time(|maybe_time_hdl| {
            if maybe_time_hdl.is_none() {
                return false;
            }
            if duration.is_some() {
                let clock = drv_hdl.clock();
                if clock.can_auto_advance() {
                    return true;
                }
                false
            } else {
                false
            }
        })
}
pub(crate) fn process_expired_timers(
    wheel: &mut Wheel,
    drv_hdl: &driver::Handle,
    wake_queue: &mut WakeQueue,
) {
    panic!("STUB: not implemented");
}
pub(crate) fn shutdown_local_timers(
    wheel: &mut Wheel,
    rx: &mut Receiver,
    inject: Vec<EntryHandle>,
    drv_hdl: &driver::Handle,
) {
    panic!("STUB: not implemented");
}
#[cfg(feature = "test-util")]
pub(crate) fn post_auto_advance(drv_hdl: &driver::Handle, duration: Option<Duration>) {
    drv_hdl
        .with_time(|maybe_time_hdl| {
            let Some(time_hdl) = maybe_time_hdl else {
                return;
            };
            if let Some(park_duration) = duration {
                let clock = drv_hdl.clock();
                if clock.can_auto_advance() && !time_hdl.did_wake() {
                    if let Err(msg) = clock.advance(park_duration) {
                        panic!("{msg}");
                    }
                }
            }
        })
}
#[cfg(not(feature = "test-util"))]
pub(crate) fn pre_auto_advance(
    _drv_hdl: &driver::Handle,
    _duration: Option<Duration>,
) -> bool {
    false
}
#[cfg(not(feature = "test-util"))]
pub(crate) fn post_auto_advance(_drv_hdl: &driver::Handle, _duration: Option<Duration>) {}
