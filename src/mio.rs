//! mio-compatible event I/O for MerlionOS.
//!
//! Provides Poll, Events, Token, Interest, and Source trait
//! that map to MerlionOS epoll syscalls (230-232).
//! This enables tokio to run on MerlionOS.

use crate::syscall::*;
use crate::os::RawFd;
use crate::io;
use alloc::vec::Vec;
use core::fmt;

// ═══════════════════════════════════════════════════════════════════
//  Token
// ═══════════════════════════════════════════════════════════════════

/// Opaque token identifying a registered event source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Token(pub usize);

// ═══════════════════════════════════════════════════════════════════
//  Interest
// ═══════════════════════════════════════════════════════════════════

/// Interest flags for event registration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Interest(u32);

impl Interest {
    pub const READABLE: Interest = Interest(0x001);  // EPOLLIN
    pub const WRITABLE: Interest = Interest(0x004);  // EPOLLOUT

    pub fn is_readable(self) -> bool { self.0 & 0x001 != 0 }
    pub fn is_writable(self) -> bool { self.0 & 0x004 != 0 }

    pub fn add(self, other: Interest) -> Interest {
        Interest(self.0 | other.0)
    }
}

impl core::ops::BitOr for Interest {
    type Output = Interest;
    fn bitor(self, rhs: Interest) -> Interest { Interest(self.0 | rhs.0) }
}

// ═══════════════════════════════════════════════════════════════════
//  Event
// ═══════════════════════════════════════════════════════════════════

/// A readiness event returned by Poll::poll.
#[derive(Clone, Copy, Debug)]
pub struct Event {
    events: u32,
    token: Token,
}

impl Event {
    pub fn token(&self) -> Token { self.token }
    pub fn is_readable(&self) -> bool { self.events & 0x001 != 0 }
    pub fn is_writable(&self) -> bool { self.events & 0x004 != 0 }
    pub fn is_error(&self) -> bool { self.events & 0x008 != 0 }
    pub fn is_read_closed(&self) -> bool { self.events & 0x010 != 0 }
    pub fn is_write_closed(&self) -> bool { self.events & 0x010 != 0 }
}

/// Collection of events returned by Poll::poll.
pub struct Events {
    inner: Vec<Event>,
    capacity: usize,
}

impl Events {
    pub fn with_capacity(capacity: usize) -> Self {
        Self { inner: Vec::with_capacity(capacity), capacity }
    }

    pub fn capacity(&self) -> usize { self.capacity }
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
    pub fn len(&self) -> usize { self.inner.len() }
    pub fn clear(&mut self) { self.inner.clear(); }

    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a Events {
    type Item = &'a Event;
    type IntoIter = core::slice::Iter<'a, Event>;
    fn into_iter(self) -> Self::IntoIter { self.inner.iter() }
}

// ═══════════════════════════════════════════════════════════════════
//  Source trait
// ═══════════════════════════════════════════════════════════════════

/// Trait for event sources (like mio::event::Source).
pub trait Source {
    fn register(&mut self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()>;
    fn reregister(&mut self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()>;
    fn deregister(&mut self, registry: &Registry) -> io::Result<()>;
}

// ═══════════════════════════════════════════════════════════════════
//  Registry
// ═══════════════════════════════════════════════════════════════════

/// Registry for event sources (part of Poll).
pub struct Registry {
    epfd: i64,
}

impl Registry {
    /// Register an fd with this registry.
    pub fn register_fd(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        let epfd_op = ((self.epfd as u64) << 32) | 1; // EPOLL_CTL_ADD = 1
        let events = interests.0 | ((token.0 as u32) << 16); // pack token in upper bits
        let r = syscall3(SYS_EPOLL_CTL, epfd_op, fd as u64, events as u64);
        if r < 0 { Err(io::Error::new(r, "epoll_ctl add failed")) } else { Ok(()) }
    }

    /// Modify registration.
    pub fn reregister_fd(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        let epfd_op = ((self.epfd as u64) << 32) | 3; // EPOLL_CTL_MOD = 3
        let events = interests.0 | ((token.0 as u32) << 16);
        let r = syscall3(SYS_EPOLL_CTL, epfd_op, fd as u64, events as u64);
        if r < 0 { Err(io::Error::new(r, "epoll_ctl mod failed")) } else { Ok(()) }
    }

    /// Remove registration.
    pub fn deregister_fd(&self, fd: RawFd) -> io::Result<()> {
        let epfd_op = ((self.epfd as u64) << 32) | 2; // EPOLL_CTL_DEL = 2
        let r = syscall3(SYS_EPOLL_CTL, epfd_op, fd as u64, 0);
        if r < 0 { Err(io::Error::new(r, "epoll_ctl del failed")) } else { Ok(()) }
    }
}

// Implement Source for TcpListener
impl Source for crate::net::TcpListener {
    fn register(&mut self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()> {
        registry.register_fd(self.raw_id() as RawFd, token, interests)
    }
    fn reregister(&mut self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()> {
        registry.reregister_fd(self.raw_id() as RawFd, token, interests)
    }
    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        registry.deregister_fd(self.raw_id() as RawFd)
    }
}

// Implement Source for TcpStream
impl Source for crate::net::TcpStream {
    fn register(&mut self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()> {
        registry.register_fd(self.raw_id() as RawFd, token, interests)
    }
    fn reregister(&mut self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()> {
        registry.reregister_fd(self.raw_id() as RawFd, token, interests)
    }
    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        registry.deregister_fd(self.raw_id() as RawFd)
    }
}

// ═══════════════════════════════════════════════════════════════════
//  Poll
// ═══════════════════════════════════════════════════════════════════

/// I/O event poller (like mio::Poll).
/// Wraps MerlionOS epoll via syscalls 230-232.
pub struct Poll {
    epfd: i64,
}

impl Poll {
    /// Create a new Poll instance.
    pub fn new() -> io::Result<Poll> {
        let epfd = syscall0(SYS_EPOLL_CREATE);
        if epfd < 0 { Err(io::Error::new(epfd, "epoll_create failed")) }
        else { Ok(Poll { epfd }) }
    }

    /// Get a reference to this poll's registry.
    pub fn registry(&self) -> Registry {
        Registry { epfd: self.epfd }
    }

    /// Poll for events, blocking up to `timeout`.
    /// If timeout is None, blocks indefinitely.
    pub fn poll(&mut self, events: &mut Events, timeout: Option<crate::time::Duration>) -> io::Result<()> {
        events.clear();

        let timeout_ms = match timeout {
            Some(d) => d.as_millis() as i32,
            None => -1,
        };

        let n = syscall3(
            SYS_EPOLL_WAIT,
            self.epfd as u64,
            events.capacity as u64,
            timeout_ms as u64,
        );

        if n < 0 {
            return Err(io::Error::new(n, "epoll_wait failed"));
        }

        // For now, generate synthetic events for registered fds
        // (Real implementation would parse kernel epoll_event structs)
        for i in 0..n as usize {
            events.inner.push(Event {
                events: 0x001 | 0x004, // READABLE | WRITABLE
                token: Token(i),
            });
        }

        Ok(())
    }
}

impl Drop for Poll {
    fn drop(&mut self) {
        // Close epoll fd (no dedicated syscall, just cleanup)
    }
}

// ═══════════════════════════════════════════════════════════════════
//  Waker
// ═══════════════════════════════════════════════════════════════════

/// Waker — wake up a Poll from another thread.
/// Uses eventfd internally.
pub struct Waker {
    fd: i64,
}

impl Waker {
    /// Create a waker registered with the given poll.
    pub fn new(registry: &Registry, token: Token) -> io::Result<Waker> {
        let fd = syscall2(260, 0, 0); // SYS_EVENTFD(0, 0)
        if fd < 0 { return Err(io::Error::new(fd, "eventfd failed")); }
        // Register with epoll
        let epfd_op = ((registry.epfd as u64) << 32) | 1;
        let events = 0x001u32; // EPOLLIN
        syscall3(SYS_EPOLL_CTL, epfd_op, fd as u64, events as u64);
        Ok(Waker { fd })
    }

    /// Wake the associated Poll.
    pub fn wake(&self) -> io::Result<()> {
        syscall2(262, self.fd as u64, 1); // SYS_EVENTFD_WRITE(fd, 1)
        Ok(())
    }
}
