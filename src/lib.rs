//! libmerlion — Rust std shim for MerlionOS
//!
//! Provides `std`-compatible types that map to MerlionOS syscalls (int 0x80).
//! Programs link against this crate instead of `std` to run on MerlionOS.
//!
//! # Usage
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! merlion_std = { path = "../libmerlion" }
//! ```
//!
//! ```rust
//! #![no_std]
//! #![no_main]
//! extern crate merlion_std;
//! use merlion_std::prelude::*;
//!
//! #[no_mangle]
//! pub extern "C" fn _start() -> ! {
//!     println!("Hello from MerlionOS!");
//!     merlion_std::process::exit(0);
//! }
//! ```

#![no_std]

extern crate alloc;

pub mod syscall;
pub mod io;
pub mod fs;
pub mod net;
pub mod thread;
pub mod sync;
pub mod time;
pub mod env;
pub mod process;
pub mod fmt;
pub mod prelude;
