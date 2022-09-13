use core::alloc::Layout;

#[allow(clippy::empty_loop)]
#[alloc_error_handler]
pub fn oom(_: Layout) -> ! {
    loop {}
}

/*
 * implemented using panic-halt dependency
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
*/
