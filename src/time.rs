//! Time (std::time equivalent).

use crate::syscall::*;
use core::fmt;

/// Duration (like std::time::Duration).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    ms: u64,
}

impl Duration {
    pub const ZERO: Duration = Duration { ms: 0 };
    pub const fn from_millis(ms: u64) -> Self { Self { ms } }
    pub const fn from_secs(s: u64) -> Self { Self { ms: s * 1000 } }
    pub const fn from_micros(us: u64) -> Self { Self { ms: us / 1000 } }
    pub const fn from_nanos(ns: u64) -> Self { Self { ms: ns / 1_000_000 } }
    pub fn as_millis(&self) -> u64 { self.ms }
    pub fn as_secs(&self) -> u64 { self.ms / 1000 }
    pub fn as_micros(&self) -> u128 { self.ms as u128 * 1000 }
    pub fn as_nanos(&self) -> u128 { self.ms as u128 * 1_000_000 }
    pub fn as_secs_f64(&self) -> f64 { self.ms as f64 / 1000.0 }
    pub fn checked_add(self, rhs: Duration) -> Option<Duration> {
        Some(Duration { ms: self.ms.checked_add(rhs.ms)? })
    }
    pub fn checked_sub(self, rhs: Duration) -> Option<Duration> {
        Some(Duration { ms: self.ms.checked_sub(rhs.ms)? })
    }
}

impl fmt::Debug for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}ms", self.ms)
    }
}

/// Monotonic instant (like std::time::Instant).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    ticks: u64,
}

impl Instant {
    pub fn now() -> Self {
        Self { ticks: syscall0(SYS_STD_INSTANT_NOW) as u64 }
    }
    pub fn elapsed(&self) -> Duration {
        let now = Self::now();
        let diff = now.ticks.saturating_sub(self.ticks);
        Duration::from_millis(diff * 10) // 100 Hz PIT
    }
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        let diff = self.ticks.saturating_sub(earlier.ticks);
        Duration::from_millis(diff * 10)
    }
    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        if self.ticks >= earlier.ticks {
            Some(self.duration_since(earlier))
        } else {
            None
        }
    }
}

impl fmt::Debug for Instant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instant({})", self.ticks)
    }
}

/// System time (like std::time::SystemTime).
#[derive(Clone, Copy)]
pub struct SystemTime {
    secs: u64,
}

impl SystemTime {
    pub const UNIX_EPOCH: SystemTime = SystemTime { secs: 0 };

    pub fn now() -> Self {
        Self { secs: syscall0(SYS_STD_SYSTEM_TIME) as u64 }
    }

    pub fn duration_since(&self, earlier: SystemTime) -> Result<Duration, ()> {
        if self.secs >= earlier.secs {
            Ok(Duration::from_secs(self.secs - earlier.secs))
        } else {
            Err(())
        }
    }

    pub fn elapsed(&self) -> Result<Duration, ()> {
        Self::now().duration_since(*self)
    }
}
