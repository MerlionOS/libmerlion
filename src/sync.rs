//! Synchronization primitives (std::sync equivalent).

use crate::syscall::*;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

/// Mutex with RAII guard (std::sync::Mutex equivalent).
pub struct Mutex<T> {
    id: u32,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Create a new mutex wrapping the given data.
    pub fn new(data: T) -> Self {
        let id = syscall0(SYS_MUTEX_CREATE) as u32;
        Self { id, data: UnsafeCell::new(data) }
    }

    /// Lock the mutex, returning a guard that unlocks on drop.
    pub fn lock(&self) -> Result<MutexGuard<T>, ()> {
        syscall1(SYS_MUTEX_LOCK, self.id as u64);
        Ok(MutexGuard { mutex: self })
    }
}

impl<T> Drop for Mutex<T> {
    fn drop(&mut self) {
        syscall1(SYS_MUTEX_DESTROY, self.id as u64);
    }
}

/// RAII mutex guard — provides &T / &mut T access, unlocks on drop.
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        syscall1(SYS_MUTEX_UNLOCK, self.mutex.id as u64);
    }
}

/// RwLock (read-write lock).
pub struct RwLock<T> {
    inner: Mutex<T>, // simplified: use mutex internally
}

impl<T> RwLock<T> {
    pub fn new(data: T) -> Self {
        Self { inner: Mutex::new(data) }
    }
    pub fn read(&self) -> Result<MutexGuard<T>, ()> { self.inner.lock() }
    pub fn write(&self) -> Result<MutexGuard<T>, ()> { self.inner.lock() }
}

unsafe impl<T: Send + Sync> Send for RwLock<T> {}
unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

/// Condvar.
pub struct Condvar {
    id: u32,
}

impl Condvar {
    pub fn new() -> Self {
        let id = syscall0(SYS_CONDVAR_CREATE) as u32;
        Self { id }
    }
    pub fn notify_one(&self) { syscall1(SYS_CONDVAR_SIGNAL, self.id as u64); }
    pub fn notify_all(&self) { syscall1(239, self.id as u64); }
}

/// Arc — just re-export alloc::sync::Arc.
pub use alloc::sync::Arc;

/// Once — run initialization exactly once.
pub struct Once {
    done: core::sync::atomic::AtomicBool,
}

impl Once {
    pub const fn new() -> Self { Self { done: core::sync::atomic::AtomicBool::new(false) } }
    pub fn call_once(&self, f: impl FnOnce()) {
        if !self.done.swap(true, core::sync::atomic::Ordering::SeqCst) {
            f();
        }
    }
}
