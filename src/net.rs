//! Networking (std::net equivalent).

use crate::syscall::*;
use crate::io::{Read, Write};

/// IPv4 address.
#[derive(Clone, Copy, Debug)]
pub struct Ipv4Addr(pub [u8; 4]);

impl Ipv4Addr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self([a, b, c, d])
    }
    pub const LOCALHOST: Self = Self([127, 0, 0, 1]);
    pub const UNSPECIFIED: Self = Self([0, 0, 0, 0]);

    fn to_packed(&self) -> u64 {
        ((self.0[0] as u64) << 24) | ((self.0[1] as u64) << 16) |
        ((self.0[2] as u64) << 8) | (self.0[3] as u64)
    }
}

/// Socket address (IP + port).
#[derive(Clone, Copy, Debug)]
pub struct SocketAddr {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl SocketAddr {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        Self { ip, port }
    }
}

/// TCP listener (server socket).
pub struct TcpListener {
    id: u32,
    port: u16,
}

impl TcpListener {
    /// Bind to an address and start listening.
    pub fn bind(addr: SocketAddr) -> Result<TcpListener, i64> {
        let id = syscall2(SYS_STD_TCP_LISTEN, addr.port as u64, 128);
        if id < 0 { Err(id) } else {
            Ok(TcpListener { id: id as u32, port: addr.port })
        }
    }

    /// Accept a new connection.
    pub fn accept(&self) -> Result<(TcpStream, SocketAddr), i64> {
        let stream_id = syscall1(SYS_STD_TCP_ACCEPT, self.id as u64);
        if stream_id < 0 { Err(stream_id) } else {
            Ok((
                TcpStream { id: stream_id as usize },
                SocketAddr::new(Ipv4Addr::UNSPECIFIED, 0),
            ))
        }
    }

    /// Get the local port.
    pub fn local_addr(&self) -> SocketAddr {
        SocketAddr::new(Ipv4Addr::UNSPECIFIED, self.port)
    }
}

/// TCP stream (connected socket).
pub struct TcpStream {
    id: usize,
}

impl TcpStream {
    /// Connect to a remote address.
    pub fn connect(addr: SocketAddr) -> Result<TcpStream, i64> {
        let id = syscall2(SYS_STD_TCP_CONNECT, addr.ip.to_packed(), addr.port as u64);
        if id < 0 { Err(id) } else {
            Ok(TcpStream { id: id as usize })
        }
    }

    /// Shutdown the connection.
    pub fn shutdown(&self) -> Result<(), i64> {
        let r = syscall1(SYS_STD_TCP_SHUTDOWN, self.id as u64);
        if r < 0 { Err(r) } else { Ok(()) }
    }
}

impl Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, i64> {
        let n = syscall3(SYS_STD_TCP_READ, self.id as u64, buf.len() as u64, buf.as_ptr() as u64);
        if n < 0 { Err(n) } else { Ok(n as usize) }
    }
}

impl Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize, i64> {
        let n = syscall3(SYS_STD_TCP_WRITE, self.id as u64, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(n) } else { Ok(n as usize) }
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        let _ = syscall1(SYS_STD_TCP_SHUTDOWN, self.id as u64);
    }
}
