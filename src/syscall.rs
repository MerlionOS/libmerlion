//! Raw MerlionOS syscall interface.
//!
//! All syscalls use `int 0x80` with:
//! - rax = syscall number
//! - rdi = arg1, rsi = arg2, rdx = arg3
//! - Return value in rax

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn syscall0(num: u64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("rax") num,
            lateout("rax") ret,
            options(nostack),
        );
    }
    ret
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn syscall1(num: u64, arg1: u64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("rax") num,
            in("rdi") arg1,
            lateout("rax") ret,
            options(nostack),
        );
    }
    ret
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn syscall2(num: u64, arg1: u64, arg2: u64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("rax") num,
            in("rdi") arg1,
            in("rsi") arg2,
            lateout("rax") ret,
            options(nostack),
        );
    }
    ret
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn syscall3(num: u64, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("rax") num,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            lateout("rax") ret,
            options(nostack),
        );
    }
    ret
}

// Stub for non-x86_64 (allows cargo check on host)
#[cfg(not(target_arch = "x86_64"))]
pub fn syscall0(_num: u64) -> i64 { -1 }
#[cfg(not(target_arch = "x86_64"))]
pub fn syscall1(_num: u64, _a1: u64) -> i64 { -1 }
#[cfg(not(target_arch = "x86_64"))]
pub fn syscall2(_num: u64, _a1: u64, _a2: u64) -> i64 { -1 }
#[cfg(not(target_arch = "x86_64"))]
pub fn syscall3(_num: u64, _a1: u64, _a2: u64, _a3: u64) -> i64 { -1 }

// Syscall numbers
pub const SYS_WRITE: u64 = 0;
pub const SYS_EXIT: u64 = 1;
pub const SYS_YIELD: u64 = 2;
pub const SYS_GETPID: u64 = 3;
pub const SYS_SLEEP: u64 = 4;
pub const SYS_OPEN: u64 = 100;
pub const SYS_READ: u64 = 101;
pub const SYS_CLOSE: u64 = 102;
pub const SYS_STAT: u64 = 103;
pub const SYS_MKDIR: u64 = 105;
pub const SYS_READDIR: u64 = 107;
pub const SYS_CHDIR: u64 = 108;
pub const SYS_GETCWD: u64 = 109;
pub const SYS_FORK: u64 = 110;
pub const SYS_EXEC: u64 = 111;
pub const SYS_WAITPID: u64 = 112;
pub const SYS_BRK: u64 = 113;
pub const SYS_EXIT_CODE: u64 = 1;
pub const SYS_MMAP: u64 = 120;
pub const SYS_SOCKET: u64 = 130;
pub const SYS_CONNECT: u64 = 131;
pub const SYS_SENDTO: u64 = 132;
pub const SYS_RECVFROM: u64 = 133;
pub const SYS_BIND: u64 = 134;
pub const SYS_LISTEN: u64 = 135;
pub const SYS_ACCEPT: u64 = 136;
pub const SYS_TIME: u64 = 140;
pub const SYS_NANOSLEEP: u64 = 141;
pub const SYS_CLOCK_MONOTONIC: u64 = 255;
pub const SYS_PIPE: u64 = 151;
pub const SYS_DUP2: u64 = 152;
pub const SYS_FWRITE: u64 = 195;
pub const SYS_GETENV: u64 = 269;
pub const SYS_EPOLL_CREATE: u64 = 230;
pub const SYS_EPOLL_CTL: u64 = 231;
pub const SYS_EPOLL_WAIT: u64 = 232;
pub const SYS_MUTEX_CREATE: u64 = 233;
pub const SYS_MUTEX_LOCK: u64 = 234;
pub const SYS_MUTEX_UNLOCK: u64 = 235;
pub const SYS_MUTEX_DESTROY: u64 = 236;
pub const SYS_CONDVAR_CREATE: u64 = 237;
pub const SYS_CONDVAR_WAIT: u64 = 238;
pub const SYS_CONDVAR_SIGNAL: u64 = 239;
pub const SYS_FUTEX_WAIT: u64 = 241;
pub const SYS_FUTEX_WAKE: u64 = 242;
pub const SYS_CLONE: u64 = 190;
pub const SYS_GETRANDOM: u64 = 266;

// std shim syscalls (270-290)
pub const SYS_STD_TCP_LISTEN: u64 = 270;
pub const SYS_STD_TCP_ACCEPT: u64 = 271;
pub const SYS_STD_TCP_CONNECT: u64 = 272;
pub const SYS_STD_TCP_READ: u64 = 273;
pub const SYS_STD_TCP_WRITE: u64 = 274;
pub const SYS_STD_TCP_SHUTDOWN: u64 = 275;
pub const SYS_STD_FILE_OPEN: u64 = 276;
pub const SYS_STD_FILE_READ: u64 = 277;
pub const SYS_STD_FILE_WRITE: u64 = 278;
pub const SYS_STD_FILE_CLOSE: u64 = 279;
pub const SYS_STD_THREAD_SLEEP: u64 = 285;
pub const SYS_STD_INSTANT_NOW: u64 = 286;
pub const SYS_STD_SYSTEM_TIME: u64 = 287;
pub const SYS_STD_CURRENT_DIR: u64 = 289;
pub const SYS_STD_SPAWN_PROCESS: u64 = 290;
