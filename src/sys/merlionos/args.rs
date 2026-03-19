//! Command-line arguments.

use alloc::string::String;
use alloc::vec::Vec;

/// Get command-line arguments (simplified — returns program name only).
pub fn args() -> Vec<String> {
    alloc::vec![String::from("merlionos-program")]
}
