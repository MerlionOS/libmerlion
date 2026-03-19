//! MerlionOS platform abstraction layer.
//!
//! Implements the sys::pal interface for MerlionOS, mapping Rust std
//! operations to MerlionOS kernel syscalls via `int 0x80`.
//!
//! Module structure matches `library/std/src/sys/pal/unix/` from rustc.

pub mod alloc;
pub mod args;
pub mod env;
pub mod fd;
pub mod fs;
pub mod io;
pub mod net;
pub mod os;
pub mod pipe;
pub mod process;
pub mod stdio;
pub mod sync;
pub mod thread;
pub mod thread_local;
pub mod time;

/// Syscall interface — all platform operations go through here.
pub mod syscall {
    pub use crate::syscall::*;
}

/// Convert a syscall return value to Result, using negative values as errors.
/// Equivalent to Unix cvt() in std::sys::pal::unix.
#[inline]
pub fn cvt(result: i64) -> Result<i64, i64> {
    if result < 0 { Err(result) } else { Ok(result) }
}

/// Convert with retry on EINTR (not applicable for MerlionOS but keeps API compat).
#[inline]
pub fn cvt_r<F: FnMut() -> i64>(mut f: F) -> Result<i64, i64> {
    cvt(f())
}

/// Platform initialization — called before main().
pub fn init() {
    // Nothing needed for MerlionOS — kernel handles setup
}

/// Platform cleanup — called after main() returns.
pub fn cleanup() {
    // Run atexit handlers if needed
}

/// Abort the process.
pub fn abort_internal() -> ! {
    crate::syscall::syscall1(crate::syscall::SYS_EXIT, 134);
    loop { core::hint::spin_loop(); }
}
