//! Process operations.

use crate::syscall::*;

pub fn exit(code: i32) -> ! {
    syscall1(SYS_EXIT, code as u64);
    loop { core::hint::spin_loop(); }
}

pub fn getpid() -> u32 { syscall0(SYS_GETPID) as u32 }
pub fn getppid() -> u32 { syscall0(114) as u32 } // SYS_GETPPID

pub fn fork() -> Result<u32, i64> {
    let r = syscall0(SYS_FORK);
    if r < 0 { Err(r) } else { Ok(r as u32) }
}

pub fn exec(path: &str) -> Result<(), i64> {
    let r = syscall2(111, path.as_ptr() as u64, path.len() as u64); // SYS_EXEC
    if r < 0 { Err(r) } else { Ok(()) }
}

pub fn waitpid(pid: u32) -> Result<i32, i64> {
    let r = syscall1(112, pid as u64); // SYS_WAITPID
    if r < 0 { Err(r) } else { Ok(r as i32) }
}

pub fn kill(pid: u32, signal: u8) -> Result<(), i64> {
    let r = syscall2(115, pid as u64, signal as u64); // SYS_KILL
    if r < 0 { Err(r) } else { Ok(()) }
}
