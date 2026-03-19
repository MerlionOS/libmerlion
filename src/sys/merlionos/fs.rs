//! Filesystem operations.
//! Equivalent to std::sys::pal::unix::fs.

use crate::syscall::*;
use super::fd::{FileDesc, RawFd};

pub struct File(FileDesc);

impl File {
    pub fn open(path: &str, write: bool) -> Result<File, i64> {
        let flags = if write { 1u64 } else { 0 };
        let fd = syscall3(SYS_OPEN, path.as_ptr() as u64, path.len() as u64, flags);
        if fd < 0 { Err(fd) } else { Ok(File(FileDesc::new(fd as RawFd))) }
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize, i64> { self.0.read(buf) }
    pub fn write(&self, buf: &[u8]) -> Result<usize, i64> { self.0.write(buf) }
    pub fn fd(&self) -> &FileDesc { &self.0 }
    pub fn raw_fd(&self) -> RawFd { self.0.raw() }
    pub fn set_nonblocking(&self, nb: bool) -> Result<(), i64> { self.0.set_nonblocking(nb) }
}

pub fn stat(path: &str) -> Result<FileStat, i64> {
    let r = syscall3(SYS_STAT, path.as_ptr() as u64, path.len() as u64, 0);
    if r < 0 { Err(r) } else { Ok(FileStat { size: r as u64, is_dir: false }) }
}

pub struct FileStat {
    pub size: u64,
    pub is_dir: bool,
}

pub fn mkdir(path: &str) -> Result<(), i64> {
    let r = syscall2(SYS_MKDIR, path.as_ptr() as u64, path.len() as u64);
    if r < 0 { Err(r) } else { Ok(()) }
}

pub fn unlink(path: &str) -> Result<(), i64> {
    let r = syscall2(106, path.as_ptr() as u64, path.len() as u64); // SYS_UNLINK
    if r < 0 { Err(r) } else { Ok(()) }
}

pub fn getcwd() -> Result<alloc::string::String, i64> {
    let mut buf = [0u8; 256];
    let r = syscall2(SYS_GETCWD, buf.as_ptr() as u64, 256);
    if r < 0 { Err(r) }
    else {
        let s = core::str::from_utf8(&buf[..r as usize]).unwrap_or("/");
        Ok(alloc::string::String::from(s))
    }
}

pub fn chdir(path: &str) -> Result<(), i64> {
    let r = syscall2(SYS_CHDIR, path.as_ptr() as u64, path.len() as u64);
    if r < 0 { Err(r) } else { Ok(()) }
}
