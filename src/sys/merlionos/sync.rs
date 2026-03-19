//! Synchronization primitives via MerlionOS futex/mutex syscalls.

use crate::syscall::*;
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

/// Futex-based mutex (matches std::sys::pal::unix::sync::Mutex).
pub struct Mutex {
    /// 0 = unlocked, 1 = locked
    state: AtomicU32,
}

impl Mutex {
    pub const fn new() -> Self { Self { state: AtomicU32::new(0) } }

    pub fn lock(&self) {
        while self.state.compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // Futex wait: sleep until state != 1
            syscall2(SYS_FUTEX_WAIT, &self.state as *const _ as u64, 1);
        }
    }

    pub fn try_lock(&self) -> bool {
        self.state.compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed).is_ok()
    }

    pub fn unlock(&self) {
        self.state.store(0, Ordering::Release);
        // Futex wake: wake one waiter
        syscall2(SYS_FUTEX_WAKE, &self.state as *const _ as u64, 1);
    }
}

/// Futex-based condition variable.
pub struct Condvar {
    seq: AtomicU32,
}

impl Condvar {
    pub const fn new() -> Self { Self { seq: AtomicU32::new(0) } }

    pub fn wait(&self, mutex: &Mutex) {
        let seq = self.seq.load(Ordering::Relaxed);
        mutex.unlock();
        syscall2(SYS_FUTEX_WAIT, &self.seq as *const _ as u64, seq as u64);
        mutex.lock();
    }

    pub fn notify_one(&self) {
        self.seq.fetch_add(1, Ordering::Release);
        syscall2(SYS_FUTEX_WAKE, &self.seq as *const _ as u64, 1);
    }

    pub fn notify_all(&self) {
        self.seq.fetch_add(1, Ordering::Release);
        syscall2(SYS_FUTEX_WAKE, &self.seq as *const _ as u64, u32::MAX as u64);
    }
}

/// Read-write lock.
pub struct RwLock {
    /// 0 = free, >0 = N readers, u32::MAX = writer
    state: AtomicU32,
}

impl RwLock {
    pub const fn new() -> Self { Self { state: AtomicU32::new(0) } }

    pub fn read(&self) {
        loop {
            let s = self.state.load(Ordering::Acquire);
            if s != u32::MAX {
                if self.state.compare_exchange(s, s + 1, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                    return;
                }
            }
            core::hint::spin_loop();
        }
    }

    pub fn write(&self) {
        while self.state.compare_exchange(0, u32::MAX, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }
    }

    pub fn read_unlock(&self) { self.state.fetch_sub(1, Ordering::Release); }
    pub fn write_unlock(&self) { self.state.store(0, Ordering::Release); }
}

/// Once — run initialization exactly once (like pthread_once).
pub struct Once {
    done: AtomicBool,
}

impl Once {
    pub const fn new() -> Self { Self { done: AtomicBool::new(false) } }

    pub fn call_once<F: FnOnce()>(&self, f: F) {
        if !self.done.swap(true, Ordering::SeqCst) { f(); }
    }

    pub fn is_completed(&self) -> bool { self.done.load(Ordering::SeqCst) }
}
