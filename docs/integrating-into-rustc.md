# Integrating MerlionOS into rust-lang/rust

This guide explains how to add `x86_64-unknown-merlionos` as a proper Rust target so that standard Rust programs (including tokio, axum, MerlionClaw) compile and run on MerlionOS.

## Overview

```
rust-lang/rust (forked)
├── compiler/rustc_target/src/spec/
│   └── targets/
│       └── x86_64_unknown_merlionos.rs    ← Target definition
├── library/std/src/sys/pal/
│   ├── mod.rs                              ← Add merlionos cfg branch
│   └── merlionos/                          ← Platform layer (from libmerlion)
│       ├── mod.rs
│       ├── alloc.rs
│       ├── fd.rs
│       ├── fs.rs
│       ├── net.rs
│       ├── sync.rs
│       ├── thread.rs
│       ├── time.rs
│       ├── stdio.rs
│       ├── io.rs
│       ├── pipe.rs
│       ├── process.rs
│       ├── args.rs
│       ├── env.rs
│       ├── os.rs
│       └── thread_local.rs
└── src/bootstrap/src/core/sanity.rs        ← Allow new target in bootstrap
```

## Step 1: Fork rust-lang/rust

```sh
git clone https://github.com/rust-lang/rust.git
cd rust
git checkout -b merlionos-target
```

## Step 2: Add target specification

Create `compiler/rustc_target/src/spec/targets/x86_64_unknown_merlionos.rs`:

```rust
use crate::spec::{Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetOptions, base};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "x86_64-unknown-none".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("MerlionOS (x86_64)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options: TargetOptions {
            os: "merlionos".into(),
            vendor: "unknown".into(),
            executables: true,
            has_thread_local: false,
            panic_strategy: PanicStrategy::Abort,
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            position_independent_executables: false,
            relocation_model: RelocModel::Static,
            disable_redzone: true,
            stack_probes: StackProbeType::None,
            features: "-mmx,-sse,+soft-float".into(),
            pre_link_args: TargetOptions::link_args(
                LinkerFlavor::Gnu(Cc::No, Lld::Yes),
                &["-Ttext=0x400000", "--gc-sections"],
            ),
            entry_name: "__start".into(),
            ..Default::default()
        },
    }
}
```

Register it in `compiler/rustc_target/src/spec/mod.rs`:

```rust
// In the supported_targets! macro, add:
("x86_64-unknown-merlionos", x86_64_unknown_merlionos),
```

## Step 3: Copy platform layer

```sh
# From the libmerlion repo
cp -r /path/to/libmerlion/src/sys/merlionos/ \
      library/std/src/sys/pal/merlionos/
```

## Step 4: Wire up in std::sys::pal

Edit `library/std/src/sys/pal/mod.rs` — add a `cfg` branch:

```rust
cfg_if::cfg_if! {
    if #[cfg(target_os = "merlionos")] {
        mod merlionos;
        pub use self::merlionos::*;
    } else if #[cfg(unix)] {
        mod unix;
        pub use self::unix::*;
    } else if #[cfg(windows)] {
        mod windows;
        pub use self::windows::*;
    }
    // ... existing branches
}
```

## Step 5: Update bootstrap

Edit `src/bootstrap/src/core/sanity.rs` — add to `STAGE0_MISSING_TARGETS`:

```rust
const STAGE0_MISSING_TARGETS: &[&str] = &[
    // ... existing entries
    "x86_64-unknown-merlionos",
];
```

## Step 6: Adapt feature modules

Some `library/std/src/sys/` feature modules route to `pal` implementations.
For each, ensure MerlionOS is covered:

### `sys/alloc/mod.rs`
```rust
#[cfg(target_os = "merlionos")]
pub use crate::sys::pal::merlionos::alloc::System;
```

### `sys/sync/mod.rs`
```rust
#[cfg(target_os = "merlionos")]
pub use crate::sys::pal::merlionos::sync::{Mutex, Condvar, RwLock};
```

### `sys/thread/mod.rs`
```rust
#[cfg(target_os = "merlionos")]
pub use crate::sys::pal::merlionos::thread::Thread;
```

Repeat for: `sys/fs/`, `sys/net/`, `sys/stdio/`, `sys/time/`, `sys/process/`,
`sys/pipe/`, `sys/env/`, `sys/args/`, `sys/fd/`, `sys/thread_local/`.

## Step 7: Build the compiler

```sh
# Configure
cat > config.toml << 'EOF'
[build]
target = ["x86_64-unknown-merlionos"]

[rust]
lld = true
EOF

# Build stage 1 compiler with MerlionOS support
./x.py build --stage 1 library/std --target x86_64-unknown-merlionos
```

## Step 8: Install the toolchain

```sh
# Install as a custom toolchain
rustup toolchain link merlionos build/host/stage1

# Verify
rustc +merlionos --print target-list | grep merlionos
# Should show: x86_64-unknown-merlionos
```

## Step 9: Compile programs for MerlionOS

```sh
# Hello World
cat > hello.rs << 'EOF'
fn main() {
    println!("Hello from Rust std on MerlionOS!");
}
EOF

rustc +merlionos --target x86_64-unknown-merlionos hello.rs -o hello

# MerlionClaw
cd /path/to/merlionclaw
cargo +merlionos build --target x86_64-unknown-merlionos
```

## Step 10: Run on MerlionOS

```sh
# Copy ELF to MerlionOS VFS and execute
# In MerlionOS shell:
run-user hello
```

## Platform Module Reference

| Module | Syscalls Used | std API Provided |
|--------|--------------|-----------------|
| `alloc.rs` | SYS_BRK (113) | GlobalAlloc, Vec, String, Box |
| `fd.rs` | READ/WRITE/CLOSE/FCNTL | RawFd, FileDesc |
| `fs.rs` | OPEN/STAT/MKDIR/UNLINK/GETCWD | File, metadata, read_dir |
| `net.rs` | SOCKET/CONNECT/BIND/LISTEN/ACCEPT/SEND/RECV | TcpListener, TcpStream, UdpSocket |
| `sync.rs` | FUTEX_WAIT/WAKE | Mutex, RwLock, Condvar, Once |
| `thread.rs` | CLONE/WAITPID/YIELD/NANOSLEEP | Thread, spawn, sleep |
| `time.rs` | CLOCK_MONOTONIC/GETTIMEOFDAY | Instant, SystemTime |
| `stdio.rs` | WRITE/READ (fd 0/1/2) | Stdin, Stdout, Stderr |
| `io.rs` | READ/FWRITE | vectored I/O |
| `pipe.rs` | PIPE | anonymous pipes |
| `process.rs` | EXIT/FORK/EXEC/WAITPID/KILL | Command, Child, exit |
| `args.rs` | — | env::args() |
| `env.rs` | GETENV | env::var() |
| `os.rs` | GETCWD/CHDIR | current_dir, temp_dir |
| `thread_local.rs` | TLS_KEY_CREATE/SET/GET | thread_local! |

## MerlionOS Syscall ABI

```
Instruction:  int 0x80
Registers:    rax = syscall number
              rdi = arg1, rsi = arg2, rdx = arg3
Return:       rax (negative = error)
```

Full syscall reference: see `docs/syscall-reference.md` in merlion-kernel.

## Comparison with Other OS Targets

| OS | Platform Dir | Syscall Method | Status |
|----|-------------|---------------|--------|
| Linux | `sys/pal/unix/` | `syscall` instruction | Tier 1 |
| macOS | `sys/pal/unix/` | `syscall` instruction | Tier 1 |
| Windows | `sys/pal/windows/` | Win32 API | Tier 1 |
| Redox | `sys/pal/unix/` (shared) | `int 0x80` | Tier 3 |
| WASI | `sys/pal/wasi/` | WASI imports | Tier 2 |
| UEFI | `sys/pal/uefi/` | UEFI services | Tier 3 |
| **MerlionOS** | `sys/pal/merlionos/` | `int 0x80` | Tier 3 |

## What Works After Integration

- `println!`, `format!`, `String`, `Vec`, `HashMap` — heap allocation via brk
- `std::fs::File`, `read_to_string`, `write` — VFS operations
- `std::net::TcpListener`, `TcpStream` — TCP networking
- `std::thread::spawn`, `sleep` — multithreading via clone
- `std::sync::Mutex`, `Arc`, `Condvar` — futex-based synchronization
- `std::time::Instant`, `SystemTime` — monotonic + wall clock
- `std::process::exit`, `Command` — process management
- `std::env::var`, `args` — environment access
- `tokio` — via mio → epoll syscalls (230-232)
- `axum`, `hyper`, `reqwest` — via tokio
- **MerlionClaw** — the entire agent runtime
