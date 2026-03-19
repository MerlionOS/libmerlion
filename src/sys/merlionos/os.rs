//! OS-level operations (getcwd, chdir, temp_dir, etc.).

use alloc::string::String;
use crate::syscall::*;

pub fn getcwd() -> Result<String, i64> { super::fs::getcwd() }
pub fn chdir(path: &str) -> Result<(), i64> { super::fs::chdir(path) }
pub fn temp_dir() -> String { String::from("/tmp") }
pub fn home_dir() -> Option<String> { super::env::getenv("HOME") }
pub fn current_exe() -> Result<String, i64> { Ok(String::from("/bin/merlionos-program")) }

pub fn errno() -> i32 { 0 } // simplified — no per-thread errno yet
pub fn set_errno(_e: i32) {}

/// Page size.
pub const PAGE_SIZE: usize = 4096;
