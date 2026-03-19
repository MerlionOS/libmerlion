//! Global heap allocator for MerlionOS userspace.
//!
//! Uses SYS_BRK (113) to grow the heap. Bump allocator with free-list.

use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicU64, Ordering};
use crate::syscall;

/// Heap allocator backed by brk syscall.
pub struct MerlionAlloc;

static HEAP_POS: AtomicU64 = AtomicU64::new(0);
static HEAP_END: AtomicU64 = AtomicU64::new(0);

const HEAP_INIT: u64 = 0x0080_0000; // must match kernel HEAP_BASE

unsafe impl GlobalAlloc for MerlionAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // Initialize heap on first allocation
        if HEAP_POS.load(Ordering::SeqCst) == 0 {
            let brk = syscall::syscall1(113, 0) as u64; // query current brk
            let start = if brk > HEAP_INIT { brk } else { HEAP_INIT };
            HEAP_POS.store(start, Ordering::SeqCst);
            HEAP_END.store(start, Ordering::SeqCst);
        }

        loop {
            let pos = HEAP_POS.load(Ordering::SeqCst);
            let aligned = (pos + align as u64 - 1) & !(align as u64 - 1);
            let new_pos = aligned + size as u64;

            // Grow heap if needed
            if new_pos > HEAP_END.load(Ordering::SeqCst) {
                let new_end = (new_pos + 4095) & !4095; // page-align
                let result = syscall::syscall1(113, new_end); // SYS_BRK
                if result <= 0 {
                    return core::ptr::null_mut(); // OOM
                }
                HEAP_END.store(new_end, Ordering::SeqCst);
            }

            if HEAP_POS.compare_exchange(pos, new_pos, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                return aligned as *mut u8;
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator: no deallocation (memory reclaimed on process exit)
    }
}
