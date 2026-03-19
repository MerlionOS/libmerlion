//! Threading (std::thread equivalent).

use crate::syscall::*;

/// Sleep for the given duration in milliseconds.
pub fn sleep_ms(ms: u64) {
    syscall1(SYS_STD_THREAD_SLEEP, ms);
}

/// Yield the current thread to the scheduler.
pub fn yield_now() {
    syscall0(SYS_YIELD);
}

/// Get the current thread/process ID.
pub fn current_id() -> u32 {
    syscall0(SYS_GETPID) as u32
}

/// Spawn a new thread (via SYS_CLONE).
/// Returns the thread ID.
pub fn spawn() -> Result<u32, i64> {
    let tid = syscall2(SYS_CLONE, 0, 0);
    if tid < 0 { Err(tid) } else { Ok(tid as u32) }
}
