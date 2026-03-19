//! Environment (std::env equivalent).

/// Get an environment variable.
pub fn var(name: &str) -> Option<&'static str> {
    // Simplified: would need allocator for dynamic string
    None
}

/// Get current working directory.
pub fn current_dir() -> &'static str {
    "/"
}

/// Get process arguments.
pub fn args() -> &'static [&'static str] {
    &["merlionos-program"]
}
