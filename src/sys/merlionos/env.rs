//! Environment variables.

use alloc::string::String;
use crate::syscall::*;

pub fn getenv(name: &str) -> Option<String> {
    let mut buf = [0u8; 256];
    let n = syscall3(SYS_GETENV, name.as_ptr() as u64, name.len() as u64, buf.as_ptr() as u64);
    if n <= 0 { None }
    else { core::str::from_utf8(&buf[..n as usize]).ok().map(String::from) }
}

pub fn setenv(name: &str, value: &str) {
    // Set via kernel env module (no direct syscall yet — would need SYS_SETENV)
    let _ = (name, value);
}
