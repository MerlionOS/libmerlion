//! Threading (std::thread equivalent).

use crate::syscall::*;
use crate::time::Duration;

/// Sleep for the given duration.
pub fn sleep(dur: Duration) {
    syscall1(SYS_STD_THREAD_SLEEP, dur.as_millis());
}

/// Sleep for milliseconds.
pub fn sleep_ms(ms: u64) {
    syscall1(SYS_STD_THREAD_SLEEP, ms);
}

/// Yield the current thread.
pub fn yield_now() {
    syscall0(SYS_YIELD);
}

/// Get current thread ID.
pub fn current_id() -> u32 {
    syscall0(SYS_GETPID) as u32
}

/// Spawn a new thread (simplified — no closure capture across threads yet).
pub fn spawn_fn(f: fn()) -> JoinHandle {
    let tid = syscall2(SYS_CLONE, 0, 0);
    JoinHandle { tid: tid as u32 }
}

/// Thread join handle.
pub struct JoinHandle {
    pub tid: u32,
}

impl JoinHandle {
    /// Wait for the thread to finish (simplified).
    pub fn join(self) -> Result<(), ()> {
        // Wait for thread to exit
        let r = syscall1(SYS_WAITPID, self.tid as u64);
        if r < 0 { Err(()) } else { Ok(()) }
    }
}

/// Thread builder (for naming threads).
pub struct Builder {
    name: Option<&'static str>,
}

impl Builder {
    pub fn new() -> Self { Self { name: None } }
    pub fn name(mut self, name: &'static str) -> Self { self.name = Some(name); self }
    pub fn spawn(self, f: fn()) -> crate::io::Result<JoinHandle> {
        Ok(spawn_fn(f))
    }
}
