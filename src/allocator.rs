use std::alloc::{GlobalAlloc, Layout};
use std::cell::UnsafeCell;

pub struct Locked<T> {
    inner: UnsafeCell<T>
}

impl Locked<BasicAllocator> {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(BasicAllocator::new())
        }
    }
}

unsafe impl Sync for Locked<BasicAllocator> {}

pub struct BasicAllocator {
    memory: [u8; 1024],
    point: usize,
}

impl BasicAllocator {
    pub const fn new() -> Self {
        Self{
            memory: [0; 1024],
            point: 0,
        }
    }
}

unsafe impl GlobalAlloc for Locked<BasicAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let allocator = unsafe {self.inner.get().as_mut().unwrap()};
        let point = allocator.point;
        allocator.point += layout.size();
        allocator.memory[point..allocator.point].as_mut_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}