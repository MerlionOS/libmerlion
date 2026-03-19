//! Example: Hello World on MerlionOS
//!
//! Build: cargo build --example hello --target ../merlion-kernel/target-specs/x86_64-unknown-merlionos.json

#![no_std]
#![no_main]

extern crate merlion_std;

use merlion_std::io;
use merlion_std::process;
use merlion_std::time::Instant;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    io::println("Hello from Rust on MerlionOS!");
    io::print("PID: ");
    // Print PID (simplified — no format! without alloc)
    let pid = process::id();
    io::println(if pid == 1 { "1" } else { ">1" });

    let start = Instant::now();
    merlion_std::thread::sleep_ms(100);
    let elapsed = start.elapsed();
    io::print("Slept ~");
    io::println(if elapsed.ms > 50 { "100ms" } else { "0ms" });

    io::println("Goodbye!");
    process::exit(0);
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    io::println("PANIC!");
    process::exit(1);
}
