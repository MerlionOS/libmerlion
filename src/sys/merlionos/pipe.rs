//! Pipe operations.

use crate::syscall::*;
use super::fd::{FileDesc, RawFd};

pub fn pipe() -> Result<(FileDesc, FileDesc), i64> {
    let mut fds = [0u64; 2];
    let r = syscall1(SYS_PIPE, fds.as_ptr() as u64);
    if r < 0 { Err(r) }
    else { Ok((FileDesc::new(fds[0] as RawFd), FileDesc::new(fds[1] as RawFd))) }
}
