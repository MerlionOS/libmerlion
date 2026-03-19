//! Low-level I/O operations.

use crate::syscall::*;

pub fn read(fd: i32, buf: &mut [u8]) -> Result<usize, i64> {
    let n = syscall3(SYS_READ, fd as u64, buf.as_ptr() as u64, buf.len() as u64);
    if n < 0 { Err(n) } else { Ok(n as usize) }
}

pub fn write(fd: i32, buf: &[u8]) -> Result<usize, i64> {
    let n = syscall3(SYS_FWRITE, fd as u64, buf.as_ptr() as u64, buf.len() as u64);
    if n < 0 { Err(n) } else { Ok(n as usize) }
}

/// Vectored I/O (simplified — concatenate and write).
pub fn writev(fd: i32, bufs: &[&[u8]]) -> Result<usize, i64> {
    let mut total = 0;
    for buf in bufs {
        let n = write(fd, buf)?;
        total += n;
    }
    Ok(total)
}
