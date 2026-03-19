//! I/O traits and types (std::io equivalent).

use crate::syscall::*;

/// Write bytes to stdout.
pub fn write_stdout(buf: &[u8]) -> usize {
    syscall2(SYS_WRITE, buf.as_ptr() as u64, buf.len() as u64) as usize
}

/// Write a string to stdout.
pub fn print(s: &str) {
    write_stdout(s.as_bytes());
}

/// Write a string + newline to stdout.
pub fn println(s: &str) {
    print(s);
    write_stdout(b"\n");
}

/// Read trait.
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, i64>;
}

/// Write trait.
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, i64>;
    fn flush(&mut self) -> Result<(), i64> { Ok(()) }

    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), i64> {
        while !buf.is_empty() {
            let n = self.write(buf)?;
            buf = &buf[n..];
        }
        Ok(())
    }
}

/// Stdout writer.
pub struct Stdout;

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize, i64> {
        let n = write_stdout(buf);
        Ok(n)
    }
}

/// Get stdout handle.
pub fn stdout() -> Stdout {
    Stdout
}
