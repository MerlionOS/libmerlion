//! Formatting and print macros.

/// Print to stdout (no newline).
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::print(core::format_args!($($arg)*).as_str().unwrap_or(""))
    };
}

/// Print to stdout with newline.
#[macro_export]
macro_rules! println {
    () => { $crate::io::println("") };
    ($($arg:tt)*) => {
        // Without alloc, we can only print static strings
        // With alloc: format!() then print
        $crate::io::println(core::stringify!($($arg)*))
    };
}
