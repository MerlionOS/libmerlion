# libmerlion

Rust `std` shim for MerlionOS — provides familiar Rust standard library types that map to MerlionOS kernel syscalls.

## Usage

```toml
# Cargo.toml
[dependencies]
merlion_std = { path = "../libmerlion" }
```

```rust
#![no_std]
#![no_main]
extern crate merlion_std;

use merlion_std::prelude::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println("Hello from MerlionOS!");

    // TCP server
    let addr = SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0), 8080);
    let listener = TcpListener::bind(addr).unwrap();

    // File I/O
    let mut f = File::open("/proc/version").unwrap();

    // Synchronization
    let mtx = Mutex::new();
    let _guard = mtx.lock();

    // Timing
    let start = Instant::now();
    thread::sleep_ms(100);
    let elapsed = start.elapsed();

    process::exit(0);
}
```

## Modules

| Module | std equivalent | Syscalls |
|--------|---------------|----------|
| `io` | `std::io` | SYS_WRITE (0), SYS_READ (101) |
| `fs` | `std::fs` | SYS_OPEN/READ/WRITE/CLOSE (100-102), SYS_MKDIR (105) |
| `net` | `std::net` | SYS_STD_TCP_* (270-275) |
| `thread` | `std::thread` | SYS_CLONE (190), SYS_YIELD (2), SYS_SLEEP (285) |
| `sync` | `std::sync` | SYS_MUTEX_* (233-236), SYS_CONDVAR_* (237-239) |
| `time` | `std::time` | SYS_STD_INSTANT_NOW (286), SYS_STD_SYSTEM_TIME (287) |
| `env` | `std::env` | SYS_GETENV (269), SYS_STD_CURRENT_DIR (289) |
| `process` | `std::process` | SYS_EXIT (1), SYS_GETPID (3), SYS_FORK (110) |
| `syscall` | — | Raw `int 0x80` wrappers |

## License

MIT
