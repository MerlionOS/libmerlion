//! Filesystem operations (std::fs equivalent).

use crate::syscall::*;
use crate::io::{Read, Write};

/// A file handle.
pub struct File {
    fd: usize,
}

impl File {
    /// Open a file for reading.
    pub fn open(path: &str) -> Result<File, i64> {
        let fd = syscall3(SYS_OPEN, path.as_ptr() as u64, path.len() as u64, 0);
        if fd < 0 { Err(fd) } else { Ok(File { fd: fd as usize }) }
    }

    /// Create/open a file for writing.
    pub fn create(path: &str) -> Result<File, i64> {
        let fd = syscall3(SYS_OPEN, path.as_ptr() as u64, path.len() as u64, 1);
        if fd < 0 { Err(fd) } else { Ok(File { fd: fd as usize }) }
    }

    /// Get the raw fd.
    pub fn as_raw_fd(&self) -> usize { self.fd }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, i64> {
        let n = syscall3(SYS_READ, self.fd as u64, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(n) } else { Ok(n as usize) }
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize, i64> {
        let n = syscall3(SYS_FWRITE, self.fd as u64, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(n) } else { Ok(n as usize) }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        syscall1(SYS_CLOSE, self.fd as u64);
    }
}

/// Read entire file to bytes.
pub fn read(path: &str) -> Result<&'static [u8], i64> {
    let mut f = File::open(path)?;
    // Simplified: read up to 4096 bytes
    // In full implementation, would allocate and read in loop
    let _ = f;
    Err(-1) // TODO: needs allocator
}

/// Create directory.
pub fn create_dir(path: &str) -> Result<(), i64> {
    let r = syscall2(SYS_MKDIR, path.as_ptr() as u64, path.len() as u64);
    if r < 0 { Err(r) } else { Ok(()) }
}
