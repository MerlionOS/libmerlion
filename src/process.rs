//! Process management (std::process equivalent).

use crate::syscall::*;

/// Exit the current process with code.
pub fn exit(code: i32) -> ! {
    syscall1(SYS_EXIT, code as u64);
    loop { core::hint::spin_loop(); }
}

/// Abort the process.
pub fn abort() -> ! {
    exit(134) // SIGABRT
}

/// Get current process ID.
pub fn id() -> u32 {
    syscall0(SYS_GETPID) as u32
}

/// Exit code wrapper.
pub struct ExitCode(pub i32);

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(0);
    pub const FAILURE: ExitCode = ExitCode(1);
}
