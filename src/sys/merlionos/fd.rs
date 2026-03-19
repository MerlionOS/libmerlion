//! File descriptor operations.
//! Equivalent to std::sys::pal::unix::fd.

use crate::syscall::*;

/// Raw file descriptor.
pub type RawFd = i32;

/// Owned file descriptor with Drop.
pub struct FileDesc {
    fd: RawFd,
}

impl FileDesc {
    pub fn new(fd: RawFd) -> Self { Self { fd } }
    pub fn raw(&self) -> RawFd { self.fd }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize, i64> {
        let n = syscall3(SYS_READ, self.fd as u64, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(n) } else { Ok(n as usize) }
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize, i64> {
        let n = syscall3(SYS_FWRITE, self.fd as u64, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(n) } else { Ok(n as usize) }
    }

    pub fn close(&self) -> Result<(), i64> {
        let r = syscall1(SYS_CLOSE, self.fd as u64);
        if r < 0 { Err(r) } else { Ok(()) }
    }

    pub fn duplicate(&self) -> Result<FileDesc, i64> {
        let new_fd = syscall2(SYS_DUP2, self.fd as u64, 0xFF); // dup to next available
        if new_fd < 0 { Err(new_fd) } else { Ok(FileDesc::new(new_fd as RawFd)) }
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> Result<(), i64> {
        let flags = if nonblocking { 0x800 } else { 0 }; // O_NONBLOCK
        let r = syscall3(243, self.fd as u64, 4, flags); // SYS_FCNTL, F_SETFL
        if r < 0 { Err(r) } else { Ok(()) }
    }

    pub fn set_cloexec(&self) -> Result<(), i64> {
        let r = syscall3(243, self.fd as u64, 2, 1); // SYS_FCNTL, F_SETFD, FD_CLOEXEC
        if r < 0 { Err(r) } else { Ok(()) }
    }
}

impl Drop for FileDesc {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
