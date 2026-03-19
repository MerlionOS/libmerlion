//! OS-specific types (std::os equivalent).
//!
//! Provides RawFd, AsRawFd, FromRawFd, IntoRawFd for MerlionOS.
//! These are needed by mio/tokio for fd-based I/O.

/// Raw file descriptor (like std::os::unix::io::RawFd).
pub type RawFd = i32;

/// Trait for types that expose a raw file descriptor.
pub trait AsRawFd {
    fn as_raw_fd(&self) -> RawFd;
}

/// Trait for constructing from a raw fd.
pub trait FromRawFd {
    unsafe fn from_raw_fd(fd: RawFd) -> Self;
}

/// Trait for consuming into a raw fd.
pub trait IntoRawFd {
    fn into_raw_fd(self) -> RawFd;
}

// Implement for our types
impl AsRawFd for crate::fs::File {
    fn as_raw_fd(&self) -> RawFd { self.as_raw_fd() as RawFd }
}

impl AsRawFd for crate::net::TcpListener {
    fn as_raw_fd(&self) -> RawFd { self.raw_id() as RawFd }
}

impl AsRawFd for crate::net::TcpStream {
    fn as_raw_fd(&self) -> RawFd { self.raw_id() as RawFd }
}

/// Owned fd that closes on drop.
pub struct OwnedFd {
    fd: RawFd,
}

impl OwnedFd {
    pub fn new(fd: RawFd) -> Self { Self { fd } }
}

impl AsRawFd for OwnedFd {
    fn as_raw_fd(&self) -> RawFd { self.fd }
}

impl Drop for OwnedFd {
    fn drop(&mut self) {
        crate::syscall::syscall1(crate::syscall::SYS_CLOSE, self.fd as u64);
    }
}
