// copied from https://stackoverflow.com/questions/70351076/in-rust-no-std-how-can-i-return-one-of-multiple-closures-implementing-a-trait-u

use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::mem::MaybeUninit;
use core::ptr::{self, NonNull};

use cortex_m::interrupt::Mutex;
use linked_list_allocator::Heap;

pub struct CortexMHeap {
    heap: Mutex<RefCell<MaybeUninit<Heap>>>,
}

impl CortexMHeap {
    /// Crate a new UNINITIALIZED heap allocator
    ///
    /// # Safety
    ///
    /// You must initialize this heap using the
    /// [`init`](struct.CortexMHeap.html#method.init) method before using the allocator.
    pub const unsafe fn empty() -> CortexMHeap {
        CortexMHeap {
            heap: Mutex::new(RefCell::new(MaybeUninit::uninit())),
        }
    }

    fn heap(&self, cs: &cortex_m::interrupt::CriticalSection) -> &mut Heap {
        let heap = &mut *self.heap.borrow(cs).borrow_mut();
        // SAFETY: `init()` initializes this, and it's guaranteed to be called by preconditions of `empty()`.
        unsafe { &mut *heap.as_mut_ptr() }
    }

    /// Initializes the heap
    ///
    /// This function must be called BEFORE you run any code that makes use of the
    /// allocator.
    ///
    /// `start_addr` is the address where the heap will be located.
    ///
    /// `size` is the size of the heap in bytes.
    ///
    /// Note that:
    ///
    /// - The heap grows "upwards", towards larger addresses. Thus `end_addr` must
    ///   be larger than `start_addr`
    ///
    /// - The size of the heap is `(end_addr as usize) - (start_addr as usize)`. The
    ///   allocator won't use the byte at `end_addr`.
    ///
    /// # Safety
    ///
    /// Obey these or Bad Stuff will happen.
    ///
    /// - This function must be called exactly ONCE.
    /// - `size > 0`
    pub unsafe fn init(&self, start_addr: usize, size: usize) {
        cortex_m::interrupt::free(|cs| {
            let heap = &mut *self.heap.borrow(cs).borrow_mut();
            *heap = MaybeUninit::new(Heap::empty());
            (*heap.as_mut_ptr()).init(start_addr, size);
        });
    }

    /// Returns an estimate of the amount of bytes in use.
    pub fn used(&self) -> usize {
        cortex_m::interrupt::free(|cs| self.heap(cs).used())
    }

    /// Returns an estimate of the amount of bytes available.
    pub fn free(&self) -> usize {
        cortex_m::interrupt::free(|cs| self.heap(cs).free())
    }
}

unsafe impl GlobalAlloc for CortexMHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        cortex_m::interrupt::free(|cs| {
            self.heap(cs)
                .allocate_first_fit(layout)
                .ok()
                .map_or(ptr::null_mut(), |allocation| allocation.as_ptr())
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        cortex_m::interrupt::free(|cs| {
            self.heap(cs)
                .deallocate(NonNull::new_unchecked(ptr), layout)
        });
    }
}
