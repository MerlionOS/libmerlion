//! Time operations.

use crate::syscall::*;

pub struct Timespec {
    pub secs: u64,
    pub nanos: u64,
}

pub fn monotonic_now() -> Timespec {
    let mut buf = [0u64; 2];
    syscall2(255, buf.as_ptr() as u64, 16); // SYS_CLOCK_MONOTONIC
    Timespec { secs: buf[0], nanos: buf[1] }
}

pub fn realtime_now() -> Timespec {
    let mut buf = [0u64; 2];
    syscall2(254, buf.as_ptr() as u64, 16); // SYS_GETTIMEOFDAY
    Timespec { secs: buf[0], nanos: buf[1] * 1000 } // usec → nsec
}

pub fn sleep(ms: u64) {
    syscall1(141, ms); // SYS_NANOSLEEP
}
