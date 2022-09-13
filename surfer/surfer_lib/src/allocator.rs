extern crate alloc;

use alloc_cortex_m::CortexMHeap;

#[global_allocator]
pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

/*
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
    }

    let mut xs = Vec::new();
    xs.push(1);

    loop { /* .. */ }
}
*/
