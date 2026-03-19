//! Thread creation and management.

use crate::syscall::*;

pub struct Thread {
    id: u64,
}

impl Thread {
    pub fn new(stack_size: usize) -> Result<Thread, i64> {
        let id = syscall2(SYS_CLONE, 0, stack_size as u64);
        if id < 0 { Err(id) } else { Ok(Thread { id: id as u64 }) }
    }

    pub fn yield_now() { syscall0(SYS_YIELD); }

    pub fn sleep(ms: u64) {
        syscall1(141, ms); // SYS_NANOSLEEP
    }

    pub fn id(&self) -> u64 { self.id }

    pub fn join(self) -> Result<(), i64> {
        let r = syscall1(112, self.id); // SYS_WAITPID
        if r < 0 { Err(r) } else { Ok(()) }
    }
}

pub fn current_id() -> u64 { syscall0(SYS_GETPID) as u64 }

pub fn available_parallelism() -> usize {
    // Query SMP CPU count
    1 // default single core
}
