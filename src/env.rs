//! Environment (std::env equivalent).

use alloc::string::String;
use alloc::vec::Vec;
use crate::syscall::*;

/// Get an environment variable.
pub fn var(name: &str) -> Result<String, VarError> {
    let mut buf = [0u8; 256];
    let n = syscall3(SYS_GETENV,
        name.as_ptr() as u64,
        name.len() as u64,
        buf.as_ptr() as u64,
    );
    if n < 0 { Err(VarError::NotPresent) }
    else {
        let s = core::str::from_utf8(&buf[..n as usize]).unwrap_or("");
        Ok(String::from(s))
    }
}

/// Error from var().
#[derive(Debug)]
pub enum VarError {
    NotPresent,
    NotUnicode,
}

/// Get current working directory.
pub fn current_dir() -> crate::io::Result<String> {
    let mut buf = [0u8; 256];
    let n = syscall2(SYS_STD_CURRENT_DIR, buf.as_ptr() as u64, 256);
    if n < 0 { Err(crate::io::Error::new(n, "getcwd failed")) }
    else {
        let s = core::str::from_utf8(&buf[..n as usize]).unwrap_or("/");
        Ok(String::from(s))
    }
}

/// Get command-line arguments.
pub fn args() -> Args {
    Args { pos: 0 }
}

/// Iterator over command-line arguments.
pub struct Args {
    pos: usize,
}

impl Iterator for Args {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.pos == 0 {
            self.pos = 1;
            Some(String::from("merlionos-program"))
        } else {
            None
        }
    }
}
