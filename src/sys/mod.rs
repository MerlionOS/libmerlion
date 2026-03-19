//! System platform abstraction layer for MerlionOS.
//!
//! This module mirrors `library/std/src/sys/` in rust-lang/rust.
//! It provides the platform-specific implementations that the
//! Rust standard library needs to function on MerlionOS.
//!
//! To integrate into a rustc fork:
//! 1. Copy `sys/merlionos/` to `library/std/src/sys/pal/merlionos/`
//! 2. Add `#[cfg(target_os = "merlionos")]` branch in `library/std/src/sys/pal/mod.rs`
//! 3. Add target spec `x86_64-unknown-merlionos` to `compiler/rustc_target/src/spec/`

pub mod merlionos;
