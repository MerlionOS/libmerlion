//! Process management (std::process equivalent).

use crate::syscall::*;

/// Exit the current process.
pub fn exit(code: i32) -> ! {
    syscall1(SYS_EXIT, code as u64);
    loop { core::hint::spin_loop(); }
}

/// Get current process ID.
pub fn id() -> u32 {
    syscall0(SYS_GETPID) as u32
}

/// Fork the current process.
pub fn fork() -> i64 {
    syscall0(SYS_FORK)
}
