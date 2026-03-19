//! # libmerlion — Rust `std` for MerlionOS
//!
//! Drop-in replacement for the Rust standard library, mapping
//! `std` types and traits to MerlionOS kernel syscalls via `int 0x80`.
//!
//! ## Usage
//!
//! ```rust
//! #![no_std]
//! #![no_main]
//! extern crate alloc;
//! extern crate merlion_std;
//!
//! use merlion_std::prelude::*;
//!
//! #[no_mangle]
//! pub extern "C" fn _start() -> ! {
//!     let msg = format!("Hello from PID {}!", merlion_std::process::id());
//!     merlion_std::io::println(&msg);
//!     merlion_std::process::exit(0);
//! }
//! ```

#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

// Global allocator (brk-based)
mod alloc_impl;
pub use alloc_impl::MerlionAlloc;

#[global_allocator]
static ALLOCATOR: MerlionAlloc = MerlionAlloc;

#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    syscall::syscall1(1, 137); // SYS_EXIT with code 137 (OOM)
    loop {}
}

// Panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    io::print("PANIC: ");
    if let Some(msg) = info.message().as_str() {
        io::println(msg);
    } else {
        io::println("(no message)");
    }
    process::exit(1);
}

// Modules matching std::*
pub mod syscall;
pub mod io;
pub mod fs;
pub mod net;
pub mod os;
pub mod mio;
pub mod thread;
pub mod sync;
pub mod time;
pub mod env;
pub mod process;
pub mod fmt;
pub mod collections;
pub mod prelude;

// Re-export alloc types at top level (like std does)
pub use alloc::string::String;
pub use alloc::vec::Vec;
pub use alloc::vec;
pub use alloc::format;
pub use alloc::boxed::Box;
