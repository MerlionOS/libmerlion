//! Networking (std::net equivalent).

use alloc::string::String;
use alloc::format;
use crate::syscall::*;
use crate::io::{self, Read, Write, Error, Result};
use core::fmt;

/// IPv4 address.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Addr(pub [u8; 4]);

impl Ipv4Addr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self { Self([a, b, c, d]) }
    pub const LOCALHOST: Self = Self([127, 0, 0, 1]);
    pub const UNSPECIFIED: Self = Self([0, 0, 0, 0]);
    pub fn octets(&self) -> [u8; 4] { self.0 }

    fn to_packed(&self) -> u64 {
        ((self.0[0] as u64) << 24) | ((self.0[1] as u64) << 16) |
        ((self.0[2] as u64) << 8) | (self.0[3] as u64)
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl fmt::Debug for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(self, f) }
}

/// IP address (v4 only for now).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IpAddr {
    V4(Ipv4Addr),
}

impl IpAddr {
    pub const fn new_v4(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self::V4(Ipv4Addr::new(a, b, c, d))
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self { IpAddr::V4(ip) => ip.fmt(f) }
    }
}

/// Socket address.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SocketAddr {
    ip: IpAddr,
    port: u16,
}

impl SocketAddr {
    pub fn new(ip: IpAddr, port: u16) -> Self { Self { ip, port } }
    pub fn ip(&self) -> &IpAddr { &self.ip }
    pub fn port(&self) -> u16 { self.port }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

/// Parse "ip:port" into SocketAddr.
impl SocketAddr {
    pub fn parse(s: &str) -> Option<Self> {
        let parts: alloc::vec::Vec<&str> = s.rsplitn(2, ':').collect();
        if parts.len() != 2 { return None; }
        let port: u16 = parts[0].parse().ok()?;
        let ip_parts: alloc::vec::Vec<&str> = parts[1].split('.').collect();
        if ip_parts.len() != 4 { return None; }
        let a: u8 = ip_parts[0].parse().ok()?;
        let b: u8 = ip_parts[1].parse().ok()?;
        let c: u8 = ip_parts[2].parse().ok()?;
        let d: u8 = ip_parts[3].parse().ok()?;
        Some(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(a, b, c, d)), port))
    }
}

/// Convenience: create SocketAddr from (Ipv4Addr, port).
pub fn socket_addr_v4(ip: Ipv4Addr, port: u16) -> SocketAddr {
    SocketAddr::new(IpAddr::V4(ip), port)
}

// ═══════════════════════════════════════════════════════════════════
//  TcpListener
// ═══════════════════════════════════════════════════════════════════

/// TCP listener (std::net::TcpListener equivalent).
pub struct TcpListener {
    id: i64,
    addr: SocketAddr,
}

impl TcpListener {
    /// Bind to an address.
    pub fn bind<A: ToSocketAddr>(addr: A) -> Result<TcpListener> {
        let sa = addr.to_socket_addr();
        let id = syscall2(SYS_STD_TCP_LISTEN, sa.port as u64, 128);
        if id < 0 { Err(Error::new(id, "bind failed")) }
        else { Ok(TcpListener { id, addr: sa }) }
    }

    /// Accept a connection.
    pub fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        let stream_id = syscall1(SYS_STD_TCP_ACCEPT, self.id as u64);
        if stream_id < 0 { Err(Error::new(stream_id, "accept failed")) }
        else {
            Ok((
                TcpStream { id: stream_id },
                SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
            ))
        }
    }

    /// Get local address.
    pub fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.addr)
    }

    /// Get raw internal ID (for AsRawFd).
    pub fn raw_id(&self) -> i64 { self.id }
}

/// Trait for things convertible to SocketAddr (like std::net::ToSocketAddrs).
pub trait ToSocketAddr {
    fn to_socket_addr(&self) -> SocketAddr;
}

impl ToSocketAddr for SocketAddr {
    fn to_socket_addr(&self) -> SocketAddr { *self }
}

impl ToSocketAddr for (&str, u16) {
    fn to_socket_addr(&self) -> SocketAddr {
        let ip = if self.0 == "0.0.0.0" || self.0 == "" {
            Ipv4Addr::UNSPECIFIED
        } else if self.0 == "127.0.0.1" || self.0 == "localhost" {
            Ipv4Addr::LOCALHOST
        } else {
            Ipv4Addr::UNSPECIFIED
        };
        SocketAddr::new(IpAddr::V4(ip), self.1)
    }
}

impl ToSocketAddr for &str {
    fn to_socket_addr(&self) -> SocketAddr {
        SocketAddr::parse(self).unwrap_or(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0))
    }
}

// ═══════════════════════════════════════════════════════════════════
//  TcpStream
// ═══════════════════════════════════════════════════════════════════

/// TCP stream (std::net::TcpStream equivalent).
pub struct TcpStream {
    id: i64,
}

impl TcpStream {
    /// Connect to a remote address.
    pub fn connect<A: ToSocketAddr>(addr: A) -> Result<TcpStream> {
        let sa = addr.to_socket_addr();
        let ip = match sa.ip {
            IpAddr::V4(v4) => v4,
        };
        let id = syscall2(SYS_STD_TCP_CONNECT, ip.to_packed(), sa.port as u64);
        if id < 0 { Err(Error::new(id, "connect failed")) }
        else { Ok(TcpStream { id }) }
    }

    /// Shutdown the connection.
    pub fn shutdown(&self, _how: Shutdown) -> Result<()> {
        let r = syscall1(SYS_STD_TCP_SHUTDOWN, self.id as u64);
        if r < 0 { Err(Error::new(r, "shutdown failed")) } else { Ok(()) }
    }

    /// Try to clone (creates a new reference to same connection).
    pub fn try_clone(&self) -> Result<TcpStream> {
        Ok(TcpStream { id: self.id })
    }

    /// Set TCP_NODELAY.
    pub fn set_nodelay(&self, _nodelay: bool) -> Result<()> { Ok(()) }

    /// Set read timeout (no-op in our implementation).
    pub fn set_read_timeout(&self, _dur: Option<crate::time::Duration>) -> Result<()> { Ok(()) }

    /// Get raw internal ID (for AsRawFd).
    pub fn raw_id(&self) -> i64 { self.id }
}

/// Shutdown direction.
pub enum Shutdown {
    Read,
    Write,
    Both,
}

impl Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = syscall3(SYS_STD_TCP_READ, self.id as u64, buf.len() as u64, buf.as_ptr() as u64);
        if n < 0 { Err(Error::new(n, "read failed")) } else { Ok(n as usize) }
    }
}

impl Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = syscall3(SYS_STD_TCP_WRITE, self.id as u64, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(Error::new(n, "write failed")) } else { Ok(n as usize) }
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        let _ = syscall1(SYS_STD_TCP_SHUTDOWN, self.id as u64);
    }
}
