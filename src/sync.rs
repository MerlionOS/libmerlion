//! Synchronization primitives (std::sync equivalent).

use crate::syscall::*;

/// Mutex (kernel-backed via pthread module).
pub struct Mutex {
    id: u32,
}

impl Mutex {
    pub fn new() -> Self {
        let id = syscall0(SYS_MUTEX_CREATE) as u32;
        Self { id }
    }

    pub fn lock(&self) -> MutexGuard {
        syscall1(SYS_MUTEX_LOCK, self.id as u64);
        MutexGuard { mutex_id: self.id }
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        syscall1(SYS_MUTEX_DESTROY, self.id as u64);
    }
}

/// RAII mutex guard — unlocks on drop.
pub struct MutexGuard {
    mutex_id: u32,
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        syscall1(SYS_MUTEX_UNLOCK, self.mutex_id as u64);
    }
}

/// Condition variable.
pub struct Condvar {
    id: u32,
}

impl Condvar {
    pub fn new() -> Self {
        let id = syscall0(SYS_CONDVAR_CREATE) as u32;
        Self { id }
    }

    pub fn wait(&self, mutex: &Mutex) {
        syscall2(SYS_CONDVAR_WAIT, self.id as u64, mutex.id as u64);
    }

    pub fn notify_one(&self) {
        syscall1(SYS_CONDVAR_SIGNAL, self.id as u64);
    }

    pub fn notify_all(&self) {
        syscall1(239, self.id as u64); // SYS_CONDVAR_BROADCAST
    }
}
