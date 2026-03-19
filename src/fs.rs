//! Filesystem operations (std::fs equivalent).

use alloc::string::String;
use alloc::vec::Vec;
use crate::syscall::*;
use crate::io::{self, Read, Write, Error, Result};

/// A file handle.
pub struct File {
    fd: usize,
}

impl File {
    /// Open a file for reading.
    pub fn open(path: &str) -> Result<File> {
        let fd = syscall3(SYS_OPEN, path.as_ptr() as u64, path.len() as u64, 0);
        if fd < 0 { Err(Error::new(fd, "open failed")) }
        else { Ok(File { fd: fd as usize }) }
    }

    /// Create/open a file for writing.
    pub fn create(path: &str) -> Result<File> {
        let fd = syscall3(SYS_OPEN, path.as_ptr() as u64, path.len() as u64, 1);
        if fd < 0 { Err(Error::new(fd, "create failed")) }
        else { Ok(File { fd: fd as usize }) }
    }

    /// Get raw fd.
    pub fn as_raw_fd(&self) -> usize { self.fd }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = syscall3(SYS_READ, self.fd as u64, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(Error::new(n, "read failed")) } else { Ok(n as usize) }
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = syscall3(SYS_FWRITE, self.fd as u64, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(Error::new(n, "write failed")) } else { Ok(n as usize) }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        syscall1(SYS_CLOSE, self.fd as u64);
    }
}

/// Read entire file to String.
pub fn read_to_string(path: &str) -> Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

/// Write string to file.
pub fn write(path: &str, contents: &str) -> Result<()> {
    let mut f = File::create(path)?;
    f.write_all(contents.as_bytes())
}

/// Create a directory.
pub fn create_dir(path: &str) -> Result<()> {
    let r = syscall2(SYS_MKDIR, path.as_ptr() as u64, path.len() as u64);
    if r < 0 { Err(Error::new(r, "mkdir failed")) } else { Ok(()) }
}

/// Create a directory and all parents.
pub fn create_dir_all(path: &str) -> Result<()> {
    create_dir(path) // simplified
}

/// Check if a path exists.
pub fn exists(path: &str) -> bool {
    let r = syscall3(SYS_STAT, path.as_ptr() as u64, path.len() as u64, 0);
    r >= 0
}

/// File metadata.
pub struct Metadata {
    pub size: u64,
    pub is_dir: bool,
}

/// Get file metadata.
pub fn metadata(path: &str) -> Result<Metadata> {
    let r = syscall3(SYS_STAT, path.as_ptr() as u64, path.len() as u64, 0);
    if r < 0 {
        Err(Error::new(r, "stat failed"))
    } else {
        Ok(Metadata { size: r as u64, is_dir: false })
    }
}
