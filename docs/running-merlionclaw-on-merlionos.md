# Running MerlionClaw on MerlionOS

> From zero to MerlionClaw running on a custom OS — the complete guide.

## Overview

```
MerlionClaw (Rust + tokio + axum)
    │
    ├── cargo +merlionos build --target x86_64-unknown-merlionos
    │
    ▼
rustc fork (x86_64-unknown-merlionos target)
    │
    ├── libmerlion (Rust std → MerlionOS syscalls)
    │   ├── sys::pal::merlionos   (platform layer)
    │   ├── std::net::TcpListener → SYS_STD_TCP_LISTEN (270)
    │   ├── std::fs::File         → SYS_OPEN/READ/WRITE (100-102)
    │   ├── std::sync::Mutex      → SYS_FUTEX_WAIT/WAKE (241-242)
    │   ├── std::thread::spawn    → SYS_CLONE (190)
    │   └── mio::Poll             → SYS_EPOLL_* (230-232)
    │
    ▼
MerlionOS kernel (170K lines Rust, 115+ syscalls)
    │
    └── Boots on QEMU / VMware / real hardware
```

## Prerequisites

- Linux x86_64 machine (for building the compiler)
- ~50GB disk space (rustc build)
- ~2 hours build time (one-time)
- QEMU (for testing)

## Step 1: Build the MerlionOS Rust Toolchain

### Option A: Use CI (recommended)

1. Go to https://github.com/MerlionOS/libmerlion/actions
2. Click **"Build MerlionOS Rust Toolchain"** → **"Run workflow"**
3. Wait ~2 hours
4. Download the **merlionos-rust-toolchain** artifact

```sh
# Install the downloaded toolchain
tar xzf merlionos-rust-toolchain.tar.gz -C ~/merlionos-toolchain
rustup toolchain link merlionos ~/merlionos-toolchain

# Verify
rustc +merlionos --print target-list | grep merlionos
# → x86_64-unknown-merlionos
```

### Option B: Build manually

```sh
# Clone the Rust compiler
git clone --depth 1 https://github.com/rust-lang/rust.git
cd rust

# Apply MerlionOS patches (see docs/integrating-into-rustc.md)
# ... copy target spec, platform layer, update mod.rs ...

# Build
cat > config.toml << 'EOF'
[build]
target = ["x86_64-unknown-merlionos"]
[rust]
lld = true
channel = "nightly"
[llvm]
download-ci-llvm = true
EOF

python3 x.py build --stage 1 library/std --target x86_64-unknown-merlionos

# Install
rustup toolchain link merlionos build/host/stage1
```

## Step 2: Build musl libc (for C dependencies)

Some Rust crates link to C libraries. musl provides them.

```sh
git clone https://github.com/MerlionOS/musl-merlionos.git
cd musl-merlionos
./build.sh
# Output: sysroot/ with libc.a and headers
```

## Step 3: Compile MerlionClaw

```sh
git clone https://github.com/MerlionClaw/merlionclaw.git
cd merlionclaw

# Build for MerlionOS
cargo +merlionos build \
    --target x86_64-unknown-merlionos \
    --release

# The ELF binary is at:
# target/x86_64-unknown-merlionos/release/mclaw
```

### If tokio doesn't build (expected on first attempt)

tokio depends on `mio` which depends on `libc` crate. You may need:

```toml
# merlionclaw/Cargo.toml — add patch section
[patch.crates-io]
# Point libc crate to our musl sysroot
libc = { git = "https://github.com/MerlionOS/libc-merlionos" }
```

Or compile with `--no-default-features` and adapt the networking code:

```toml
# Use libmerlion directly instead of tokio
[dependencies]
merlion_std = { git = "https://github.com/MerlionOS/libmerlion" }
```

## Step 4: Run on MerlionOS

### Option A: QEMU

```sh
# Build MerlionOS kernel
cd merlion-kernel
make build

# Boot with networking
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-merlion-kernel.bin \
    -netdev user,id=n0,hostfwd=tcp::18789-:18789 \
    -device virtio-net-pci,netdev=n0 \
    -serial stdio

# In MerlionOS shell:
merlion> run-user mclaw
[mclaw] MerlionClaw starting on :18789...
[mclaw] Gateway ready.
```

### Option B: VMware

```sh
cd merlion-kernel
make vmware-config
open MerlionOS.vmx
# In MerlionOS shell: run-user mclaw
```

### Option C: Real hardware

```sh
cd merlion-kernel
make iso
# Write to USB: sudo dd if=merlionos.iso of=/dev/sdX bs=4M
# Boot from USB → run-user mclaw
```

## Step 5: Connect to MerlionClaw

Once MerlionClaw is running on MerlionOS:

```sh
# From your laptop/phone:

# Telegram bot
# (configure TELEGRAM_TOKEN in ~/.merlionclaw/config.toml)

# CLI
curl http://<merlionos-ip>:18789/api/status

# WebSocket
wscat -c ws://<merlionos-ip>:18789/ws
```

## Architecture Deep Dive

### Syscall Flow

```
MerlionClaw calls:
    TcpListener::bind("0.0.0.0:18789")

Which goes through:
    std::net::TcpListener::bind()           ← Rust std API
    → sys::pal::merlionos::net::tcp_listen() ← our platform layer
    → syscall2(270, 18789, 128)              ← SYS_STD_TCP_LISTEN
    → int 0x80                               ← CPU trap to kernel
    → MerlionOS syscall dispatch             ← kernel handles it
    → tcp_real::listen(18789)                ← TCP stack
    → e1000e NIC driver                      ← hardware
```

### Thread Model

```
MerlionClaw's tokio runtime:
    tokio::runtime::Builder::new_multi_thread()
    → mio::Poll::new()                      ← our mio module
    → syscall0(230)                          ← SYS_EPOLL_CREATE
    → kernel epoll instance

    tokio::spawn(async { ... })
    → std::thread::spawn()
    → sys::pal::merlionos::thread::Thread::new()
    → syscall2(190, 0, stack_size)           ← SYS_CLONE
    → kernel creates new task
```

### File System

```
MerlionClaw reads config:
    std::fs::read_to_string("~/.merlionclaw/config.toml")
    → sys::pal::merlionos::fs::File::open()
    → syscall3(100, path_ptr, path_len, 0)   ← SYS_OPEN
    → kernel VFS lookup
    → returns file contents
```

## Dependency Chain

```
MerlionClaw
├── tokio (async runtime)
│   └── mio (I/O polling)
│       └── epoll → SYS_EPOLL_* (230-232)     ✅
├── axum (HTTP server)
│   └── hyper (HTTP protocol)
│       └── tokio::net::TcpListener            ✅
├── reqwest (HTTP client)
│   └── hyper + tokio::net::TcpStream           ✅
├── serde + serde_json (serialization)
│   └── no OS dependency                        ✅
├── clap (CLI parsing)
│   └── std::env::args()                        ✅
├── tracing (logging)
│   └── std::io::stderr()                       ✅
├── uuid (IDs)
│   └── getrandom → SYS_GETRANDOM (266)        ✅
└── chrono (time)
    └── clock_gettime → SYS_CLOCK_MONOTONIC (255) ✅
```

## What's Different from Linux

| Aspect | Linux | MerlionOS |
|--------|-------|-----------|
| Syscall instruction | `syscall` | `int 0x80` |
| Syscall numbers | Linux-specific (0=read, 1=write) | MerlionOS-specific (0=write, 101=read) |
| libc | glibc / musl | musl-merlionos |
| Networking | kernel TCP/IP + iptables | kernel TCP/IP + iptables |
| Filesystem | ext4/btrfs on disk | In-memory VFS |
| Init system | systemd | MerlionOS init_system |
| Container runtime | Docker/containerd | MerlionOS OCI runtime |
| GPU | nvidia/amdgpu kernel drivers | MerlionOS GPU compute drivers |

## Troubleshooting

### "unknown target x86_64-unknown-merlionos"
→ Toolchain not installed. Download from CI or build manually (Step 1).

### "can't find crate for std"
→ std not built for MerlionOS target. Ensure `--target x86_64-unknown-merlionos` is in the build command and the toolchain was built with MerlionOS std.

### "undefined reference to __syscall"
→ musl not linked. Add `-L/path/to/sysroot/lib -lc` to linker flags.

### "page fault at 0x..."
→ Program accessing unmapped memory. Check that the ELF is loaded at 0x400000 (TEXT_BASE) and heap starts at 0x800000.

### "connection refused" on port 18789
→ MerlionOS networking requires QEMU with `-netdev user`. Use `make run-full` instead of `make run`.

## Related Repositories

| Repo | What | URL |
|------|------|-----|
| merlion-kernel | OS kernel | https://github.com/MerlionOS/merlion-kernel |
| libmerlion | Rust std library | https://github.com/MerlionOS/libmerlion |
| musl-merlionos | C standard library | https://github.com/MerlionOS/musl-merlionos |
| merlionclaw | Agent runtime | https://github.com/MerlionClaw/merlionclaw |
