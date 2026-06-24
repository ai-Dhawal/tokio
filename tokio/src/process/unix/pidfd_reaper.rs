use crate::{
    io::{interest::Interest, PollEvented},
    process::{
        imp::{orphan::Wait, OrphanQueue},
        kill::Kill,
    },
};
use libc::{syscall, SYS_pidfd_open, ENOSYS, PIDFD_NONBLOCK};
use mio::{event::Source, unix::SourceFd};
use std::{
    fs::File, future::Future, io, marker::Unpin, ops::Deref,
    os::unix::io::{AsRawFd, FromRawFd, RawFd},
    pin::Pin, process::ExitStatus, sync::atomic::{AtomicBool, Ordering::Relaxed},
    task::{Context, Poll},
};
#[derive(Debug)]
struct Pidfd {
    fd: File,
}
impl Pidfd {
    fn open(pid: u32) -> Option<Pidfd> {
        panic!("STUB: not implemented");
    }
}
impl AsRawFd for Pidfd {
    fn as_raw_fd(&self) -> RawFd {
        panic!("STUB: not implemented");
    }
}
impl Source for Pidfd {
    fn register(
        &mut self,
        registry: &mio::Registry,
        token: mio::Token,
        interest: mio::Interest,
    ) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    fn reregister(
        &mut self,
        registry: &mio::Registry,
        token: mio::Token,
        interest: mio::Interest,
    ) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    fn deregister(&mut self, registry: &mio::Registry) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug)]
struct PidfdReaperInner<W>
where
    W: Unpin,
{
    inner: W,
    pidfd: PollEvented<Pidfd>,
}
impl<W> Future for PidfdReaperInner<W>
where
    W: Wait + Unpin,
{
    type Output = io::Result<ExitStatus>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug)]
pub(crate) struct PidfdReaper<W, Q>
where
    W: Wait + Unpin,
    Q: OrphanQueue<W> + Unpin,
{
    inner: Option<PidfdReaperInner<W>>,
    orphan_queue: Q,
}
impl<W, Q> Deref for PidfdReaper<W, Q>
where
    W: Wait + Unpin,
    Q: OrphanQueue<W> + Unpin,
{
    type Target = W;
    fn deref(&self) -> &Self::Target {
        panic!("STUB: not implemented");
    }
}
impl<W, Q> PidfdReaper<W, Q>
where
    W: Wait + Unpin,
    Q: OrphanQueue<W> + Unpin,
{
    pub(crate) fn new(
        inner: W,
        orphan_queue: Q,
    ) -> Result<Self, (Option<io::Error>, W)> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn inner_mut(&mut self) -> &mut W {
        panic!("STUB: not implemented");
    }
}
impl<W, Q> Future for PidfdReaper<W, Q>
where
    W: Wait + Unpin,
    Q: OrphanQueue<W> + Unpin,
{
    type Output = io::Result<ExitStatus>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
impl<W, Q> Kill for PidfdReaper<W, Q>
where
    W: Wait + Unpin + Kill,
    Q: OrphanQueue<W> + Unpin,
{
    fn kill(&mut self) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
}
impl<W, Q> Drop for PidfdReaper<W, Q>
where
    W: Wait + Unpin,
    Q: OrphanQueue<W> + Unpin,
{
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
#[cfg(all(test, not(loom), not(miri)))]
mod test {
    use super::*;
    use crate::{
        process::unix::orphan::test::MockQueue,
        runtime::{Builder as RuntimeBuilder, Runtime},
    };
    use std::process::{Command, Output};
    fn create_runtime() -> Runtime {
        RuntimeBuilder::new_current_thread().enable_io().build().unwrap()
    }
    fn run_test(fut: impl Future<Output = ()>) {
        create_runtime().block_on(fut)
    }
    fn is_pidfd_available() -> bool {
        let Output { stdout, status, .. } = Command::new("uname")
            .arg("-r")
            .output()
            .unwrap();
        assert!(status.success());
        let stdout = String::from_utf8_lossy(&stdout);
        let mut kernel_version_iter = match stdout.split_once('-') {
            Some((version, _)) => version,
            _ => &stdout,
        }
            .split('.');
        let major: u32 = kernel_version_iter.next().unwrap().parse().unwrap();
        let minor: u32 = kernel_version_iter.next().unwrap().trim().parse().unwrap();
        major >= 6 || (major == 5 && minor >= 10)
    }
    #[test]
    fn test_pidfd_reaper_poll() {
        if !is_pidfd_available() {
            eprintln!("pidfd is not available on this linux kernel, skip this test");
            return;
        }
        let queue = MockQueue::new();
        run_test(async {
            let child = Command::new("true").spawn().unwrap();
            let pidfd_reaper = PidfdReaper::new(child, &queue).unwrap();
            let exit_status = pidfd_reaper.await.unwrap();
            assert!(exit_status.success());
        });
        assert!(queue.all_enqueued.borrow().is_empty());
    }
    #[test]
    fn test_pidfd_reaper_kill() {
        if !is_pidfd_available() {
            eprintln!("pidfd is not available on this linux kernel, skip this test");
            return;
        }
        let queue = MockQueue::new();
        run_test(async {
            let child = Command::new("sleep").arg("1800").spawn().unwrap();
            let mut pidfd_reaper = PidfdReaper::new(child, &queue).unwrap();
            pidfd_reaper.kill().unwrap();
            let exit_status = pidfd_reaper.await.unwrap();
            assert!(! exit_status.success());
        });
        assert!(queue.all_enqueued.borrow().is_empty());
    }
    #[test]
    fn test_pidfd_reaper_drop() {
        if !is_pidfd_available() {
            eprintln!("pidfd is not available on this linux kernel, skip this test");
            return;
        }
        let queue = MockQueue::new();
        let mut child = Command::new("sleep").arg("1800").spawn().unwrap();
        run_test(async {
            let _pidfd_reaper = PidfdReaper::new(&mut child, &queue).unwrap();
        });
        assert_eq!(queue.all_enqueued.borrow().len(), 1);
        child.kill().unwrap();
        child.wait().unwrap();
    }
}
