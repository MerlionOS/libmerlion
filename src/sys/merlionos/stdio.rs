//! Standard I/O handles (stdin/stdout/stderr).

use crate::syscall::*;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub const fn new() -> Self { Self }
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, i64> {
        let n = syscall3(SYS_READ, 0, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(n) } else { Ok(n as usize) }
    }
}

impl Stdout {
    pub const fn new() -> Self { Self }
    pub fn write(&self, buf: &[u8]) -> Result<usize, i64> {
        let n = syscall2(SYS_WRITE, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(n) } else { Ok(n as usize) }
    }
    pub fn flush(&self) -> Result<(), i64> { Ok(()) }
}

impl Stderr {
    pub const fn new() -> Self { Self }
    pub fn write(&self, buf: &[u8]) -> Result<usize, i64> {
        let n = syscall2(SYS_WRITE, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(n) } else { Ok(n as usize) }
    }
    pub fn flush(&self) -> Result<(), i64> { Ok(()) }
}
