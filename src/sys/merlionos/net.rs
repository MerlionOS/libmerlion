//! Networking primitives.
//! Equivalent to std::sys::pal::unix::net.

use crate::syscall::*;
use super::fd::{FileDesc, RawFd};

pub struct Socket(FileDesc);

impl Socket {
    pub fn new(domain: i32, sock_type: i32, protocol: i32) -> Result<Socket, i64> {
        let fd = syscall3(SYS_SOCKET, domain as u64, sock_type as u64, protocol as u64);
        if fd < 0 { Err(fd) } else { Ok(Socket(FileDesc::new(fd as RawFd))) }
    }

    pub fn connect(&self, ip: [u8; 4], port: u16) -> Result<(), i64> {
        // Build sockaddr_in on stack
        let mut addr = [0u8; 8];
        addr[0] = 0; addr[1] = 2; // AF_INET
        addr[2] = (port >> 8) as u8; addr[3] = (port & 0xFF) as u8; // port (big-endian)
        addr[4] = ip[0]; addr[5] = ip[1]; addr[6] = ip[2]; addr[7] = ip[3]; // IP
        let r = syscall3(SYS_CONNECT, self.0.raw() as u64, addr.as_ptr() as u64, 8);
        if r < 0 { Err(r) } else { Ok(()) }
    }

    pub fn bind(&self, ip: [u8; 4], port: u16) -> Result<(), i64> {
        let mut addr = [0u8; 8];
        addr[0] = 0; addr[1] = 2;
        addr[2] = (port >> 8) as u8; addr[3] = (port & 0xFF) as u8;
        addr[4] = ip[0]; addr[5] = ip[1]; addr[6] = ip[2]; addr[7] = ip[3];
        let r = syscall3(SYS_BIND, self.0.raw() as u64, addr.as_ptr() as u64, 8);
        if r < 0 { Err(r) } else { Ok(()) }
    }

    pub fn listen(&self, backlog: i32) -> Result<(), i64> {
        let r = syscall2(SYS_LISTEN, self.0.raw() as u64, backlog as u64);
        if r < 0 { Err(r) } else { Ok(()) }
    }

    pub fn accept(&self) -> Result<Socket, i64> {
        let fd = syscall1(SYS_ACCEPT, self.0.raw() as u64);
        if fd < 0 { Err(fd) } else { Ok(Socket(FileDesc::new(fd as RawFd))) }
    }

    pub fn send(&self, data: &[u8]) -> Result<usize, i64> {
        let n = syscall3(SYS_SENDTO, self.0.raw() as u64, data.as_ptr() as u64, data.len() as u64);
        if n < 0 { Err(n) } else { Ok(n as usize) }
    }

    pub fn recv(&self, buf: &mut [u8]) -> Result<usize, i64> {
        let n = syscall3(SYS_RECVFROM, self.0.raw() as u64, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(n) } else { Ok(n as usize) }
    }

    pub fn shutdown(&self, how: i32) -> Result<(), i64> {
        let r = syscall2(268, self.0.raw() as u64, how as u64); // SYS_SHUTDOWN
        if r < 0 { Err(r) } else { Ok(()) }
    }

    pub fn set_nodelay(&self, nodelay: bool) -> Result<(), i64> {
        let val = if nodelay { 1u32 } else { 0 };
        // setsockopt(fd, IPPROTO_TCP<<16|TCP_NODELAY, val)
        let level_opt = (6u64 << 16) | 1; // IPPROTO_TCP=6, TCP_NODELAY=1
        let r = syscall3(244, self.0.raw() as u64, level_opt, val as u64);
        if r < 0 { Err(r) } else { Ok(()) }
    }

    pub fn set_nonblocking(&self, nb: bool) -> Result<(), i64> { self.0.set_nonblocking(nb) }
    pub fn fd(&self) -> &FileDesc { &self.0 }
    pub fn raw_fd(&self) -> RawFd { self.0.raw() }

    pub fn setsockopt(&self, level: i32, name: i32, val: i32) -> Result<(), i64> {
        let level_opt = ((level as u64) << 16) | (name as u64 & 0xFFFF);
        let r = syscall3(244, self.0.raw() as u64, level_opt, val as u64);
        if r < 0 { Err(r) } else { Ok(()) }
    }
}
