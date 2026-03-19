//! Time (std::time equivalent).

use crate::syscall::*;

/// Monotonic clock instant.
#[derive(Clone, Copy, Debug)]
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
        // 100 Hz PIT → each tick = 10ms
        Duration { ms: diff * 10 }
    }
}

/// Duration in milliseconds.
#[derive(Clone, Copy, Debug)]
pub struct Duration {
    pub ms: u64,
}

impl Duration {
    pub const fn from_millis(ms: u64) -> Self { Self { ms } }
    pub const fn from_secs(s: u64) -> Self { Self { ms: s * 1000 } }
    pub fn as_millis(&self) -> u64 { self.ms }
    pub fn as_secs(&self) -> u64 { self.ms / 1000 }
}

/// System time (wall clock).
pub struct SystemTime;

impl SystemTime {
    pub fn now() -> (u64, u64) {
        let secs = syscall0(SYS_STD_SYSTEM_TIME) as u64;
        (secs, 0)
    }
}
