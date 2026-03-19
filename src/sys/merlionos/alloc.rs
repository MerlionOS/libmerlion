//! Memory allocation via SYS_BRK.
//! Equivalent to std::sys::pal::unix::alloc (which uses mmap/brk).

use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicU64, Ordering};

const HEAP_INIT: u64 = 0x0080_0000;

static HEAP_POS: AtomicU64 = AtomicU64::new(0);
static HEAP_END: AtomicU64 = AtomicU64::new(0);

pub struct System;

unsafe impl GlobalAlloc for System {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        if HEAP_POS.load(Ordering::SeqCst) == 0 {
            let brk = crate::syscall::syscall1(113, 0) as u64;
            let start = if brk > HEAP_INIT { brk } else { HEAP_INIT };
            HEAP_POS.store(start, Ordering::SeqCst);
            HEAP_END.store(start, Ordering::SeqCst);
        }

        loop {
            let pos = HEAP_POS.load(Ordering::SeqCst);
            let aligned = (pos + align as u64 - 1) & !(align as u64 - 1);
            let new_pos = aligned + size as u64;

            if new_pos > HEAP_END.load(Ordering::SeqCst) {
                let new_end = (new_pos + 4095) & !4095;
                let result = crate::syscall::syscall1(113, new_end);
                if result <= 0 { return core::ptr::null_mut(); }
                HEAP_END.store(new_end, Ordering::SeqCst);
            }

            if HEAP_POS.compare_exchange(pos, new_pos, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                return aligned as *mut u8;
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator — memory reclaimed on process exit
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = self.alloc(Layout::from_size_align_unchecked(new_size, layout.align()));
        if !new_ptr.is_null() {
            core::ptr::copy_nonoverlapping(ptr, new_ptr, layout.size().min(new_size));
        }
        new_ptr
    }
}
