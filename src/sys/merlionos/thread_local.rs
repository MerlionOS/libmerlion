//! Thread-local storage.
//!
//! Uses kernel TLS syscalls (257-259) for per-thread key-value storage.

use crate::syscall::*;
use core::sync::atomic::{AtomicU32, Ordering};

/// TLS key type.
pub type Key = u32;

/// Create a TLS key.
pub fn create() -> Key {
    syscall0(257) as Key // SYS_TLS_KEY_CREATE
}

/// Set TLS value for current thread.
pub fn set(key: Key, value: u64) {
    syscall2(258, key as u64, value); // SYS_TLS_SET
}

/// Get TLS value for current thread.
pub fn get(key: Key) -> u64 {
    syscall1(259, key as u64) as u64 // SYS_TLS_GET
}

/// Destroy a TLS key (no-op — keys persist).
pub fn destroy(_key: Key) {}
