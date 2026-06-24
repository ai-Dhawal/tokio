use std::io;
use std::sync::OnceLock;
use crate::signal::RxFuture;
use crate::sync::watch;
use windows_sys::core::BOOL;
use windows_sys::Win32::System::Console as console;
type EventInfo = watch::Sender<()>;
pub(super) fn ctrl_break() -> io::Result<RxFuture> {
    panic!("STUB: not implemented");
}
pub(super) fn ctrl_close() -> io::Result<RxFuture> {
    panic!("STUB: not implemented");
}
pub(super) fn ctrl_c() -> io::Result<RxFuture> {
    panic!("STUB: not implemented");
}
pub(super) fn ctrl_logoff() -> io::Result<RxFuture> {
    panic!("STUB: not implemented");
}
pub(super) fn ctrl_shutdown() -> io::Result<RxFuture> {
    panic!("STUB: not implemented");
}
fn new(event_info: &EventInfo) -> io::Result<RxFuture> {
    panic!("STUB: not implemented");
}
fn event_requires_infinite_sleep_in_handler(signum: u32) -> bool {
    panic!("STUB: not implemented");
}
#[derive(Debug, Default)]
struct Registry {
    ctrl_break: EventInfo,
    ctrl_close: EventInfo,
    ctrl_c: EventInfo,
    ctrl_logoff: EventInfo,
    ctrl_shutdown: EventInfo,
}
impl Registry {
    fn event_info(&self, signum: u32) -> Option<&EventInfo> {
        panic!("STUB: not implemented");
    }
}
fn registry() -> &'static Registry {
    panic!("STUB: not implemented");
}
fn global_init() -> io::Result<()> {
    panic!("STUB: not implemented");
}
unsafe extern "system" fn handler(ty: u32) -> BOOL {
    panic!("STUB: not implemented");
}
#[cfg(all(test, not(loom)))]
mod tests {
    use super::*;
    use crate::runtime::Runtime;
    use tokio_test::{assert_ok, assert_pending, assert_ready_ok, task};
    unsafe fn raise_event(signum: u32) {
        if event_requires_infinite_sleep_in_handler(signum) {
            std::thread::spawn(move || unsafe { super::handler(signum) });
        } else {
            unsafe { super::handler(signum) };
        }
    }
    #[test]
    fn ctrl_c() {
        let rt = rt();
        let _enter = rt.enter();
        let mut ctrl_c = task::spawn(crate::signal::ctrl_c());
        assert_pending!(ctrl_c.poll());
        unsafe {
            raise_event(console::CTRL_C_EVENT);
        }
        assert_ready_ok!(ctrl_c.poll());
    }
    #[test]
    fn ctrl_break() {
        let rt = rt();
        rt.block_on(async {
            let mut ctrl_break = assert_ok!(crate ::signal::windows::ctrl_break());
            unsafe {
                raise_event(console::CTRL_BREAK_EVENT);
            }
            ctrl_break.recv().await.unwrap();
        });
    }
    #[test]
    fn ctrl_close() {
        let rt = rt();
        rt.block_on(async {
            let mut ctrl_close = assert_ok!(crate ::signal::windows::ctrl_close());
            unsafe {
                raise_event(console::CTRL_CLOSE_EVENT);
            }
            ctrl_close.recv().await.unwrap();
        });
    }
    #[test]
    fn ctrl_shutdown() {
        let rt = rt();
        rt.block_on(async {
            let mut ctrl_shutdown = assert_ok!(crate ::signal::windows::ctrl_shutdown());
            unsafe {
                raise_event(console::CTRL_SHUTDOWN_EVENT);
            }
            ctrl_shutdown.recv().await.unwrap();
        });
    }
    #[test]
    fn ctrl_logoff() {
        let rt = rt();
        rt.block_on(async {
            let mut ctrl_logoff = assert_ok!(crate ::signal::windows::ctrl_logoff());
            unsafe {
                raise_event(console::CTRL_LOGOFF_EVENT);
            }
            ctrl_logoff.recv().await.unwrap();
        });
    }
    fn rt() -> Runtime {
        crate::runtime::Builder::new_current_thread().build().unwrap()
    }
}
