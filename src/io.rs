//! I/O types and traits (std::io equivalent).

use alloc::string::String;
use alloc::vec::Vec;
use crate::syscall;
use core::fmt;

/// I/O error type.
#[derive(Debug)]
pub struct Error {
    pub code: i64,
    pub message: &'static str,
}

impl Error {
    pub fn new(code: i64, message: &'static str) -> Self {
        Self { code, message }
    }
    pub fn last_os_error() -> Self {
        Self { code: -1, message: "os error" }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I/O error {}: {}", self.code, self.message)
    }
}

/// I/O Result type (like std::io::Result).
pub type Result<T> = core::result::Result<T, Error>;

/// Read trait (std::io::Read equivalent).
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        let mut total = 0;
        let mut tmp = [0u8; 256];
        loop {
            match self.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => { buf.extend_from_slice(&tmp[..n]); total += n; }
                Err(e) => return Err(e),
            }
        }
        Ok(total)
    }

    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        let mut bytes = Vec::new();
        let n = self.read_to_end(&mut bytes)?;
        if let Ok(s) = core::str::from_utf8(&bytes) {
            buf.push_str(s);
        }
        Ok(n)
    }
}

/// Write trait (std::io::Write equivalent).
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()> { Ok(()) }

    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            let n = self.write(buf)?;
            if n == 0 { return Err(Error::new(-1, "write zero")); }
            buf = &buf[n..];
        }
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> Result<()> where Self: Sized {
        struct FmtWriter<'a, W: Write>(&'a mut W);
        impl<'a, W: Write> fmt::Write for FmtWriter<'a, W> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.write(s.as_bytes()).map_err(|_| fmt::Error)?;
                Ok(())
            }
        }
        fmt::write(&mut FmtWriter(self), args).map_err(|_| Error::new(-1, "fmt error"))
    }
}

/// Standard output.
pub struct Stdout;

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = syscall::syscall2(syscall::SYS_WRITE, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(Error::new(n, "write failed")) } else { Ok(n as usize) }
    }
}

/// Standard error (same as stdout in MerlionOS).
pub struct Stderr;

impl Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = syscall::syscall2(syscall::SYS_WRITE, buf.as_ptr() as u64, buf.len() as u64);
        if n < 0 { Err(Error::new(n, "write failed")) } else { Ok(n as usize) }
    }
}

/// Get stdout handle.
pub fn stdout() -> Stdout { Stdout }

/// Get stderr handle.
pub fn stderr() -> Stderr { Stderr }

/// Print a string to stdout.
pub fn print(s: &str) {
    syscall::syscall2(syscall::SYS_WRITE, s.as_ptr() as u64, s.len() as u64);
}

/// Print a string + newline to stdout.
pub fn println(s: &str) {
    print(s);
    print("\n");
}

/// Print formatted output (used by println! macro).
pub fn _print(args: fmt::Arguments) {
    let _ = Stdout.write_fmt(args);
}

/// println! macro (matches std::println!).
#[macro_export]
macro_rules! println {
    () => { $crate::io::print("\n") };
    ($($arg:tt)*) => {
        $crate::io::_print(core::format_args!($($arg)*));
        $crate::io::print("\n");
    };
}

/// print! macro (matches std::print!).
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::_print(core::format_args!($($arg)*));
    };
}

/// eprintln! macro.
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        $crate::io::_print(core::format_args!($($arg)*));
        $crate::io::print("\n");
    };
}
