use alloc::alloc::{GlobalAlloc, Layout};
use super::{align_up, Locked};
use core::ptr;

pub struct BumpAllocator {
    head_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    // create new empty bump allocator
    pub const fn new() -> Self {
        BumpAllocator {
            head_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0
        }
    }

    // Initialize bump allocator 
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) [
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    ]
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        let mut bump = self.lock();

        let alloc_start = align_up(bump.next, layout.align());

        let alloc_end = match align_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end > bump.head_end {
            ptr::null() // out of memory
        } else {
            bump.next = alloc_end;
            bump.allocation += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock(); 

        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}