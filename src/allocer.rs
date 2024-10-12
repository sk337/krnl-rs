#![no_std]
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: AtomicUsize,
}

impl BumpAllocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        Self {
            heap_start,
            heap_end: heap_start + heap_size,
            next: AtomicUsize::new(heap_start),
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc_start = self.next.fetch_add(layout.size(), Ordering::SeqCst);
        let alloc_end = alloc_start + layout.size();

        if alloc_end > self.heap_end {
            null_mut() // Out of memory
        } else {
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Simple bump allocator doesn't support deallocation
    }
}
