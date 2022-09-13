#![no_std]
#![no_main]

pub mod mcu;

extern crate no_std_compat as std;
//extern crate panic_halt;

use surfer_lib::allocator::ALLOCATOR;
use surfer_lib::DeviceApi;

use mcu::device::Device;
use mcu::entry;
//use crate::mcu::seeed_studio_xiao_sam21_cortex_m0 as m;
// use surfer_lib::DeviceUsbApi;

extern crate alloc;

fn setup(d: &mut dyn DeviceApi) {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
    }

    // 40 sec delay
    for _ in 0..40 {
        d.led0_toggle();
        d.delay(100u16);
    }

    // boot screen
    let mut dusb = d.device_usb();
    dusb.print("*****************************************************************\r\n");
    dusb.print("* booting device ...\r\n");
    dusb.print("* software: surfer v0.4\r\n");
    dusb.print("* hardware: ");
    dusb.print(d.hardware());
    dusb.print("\r\n");
    dusb.print("*****************************************************************\r\n");
    dusb.flush();
}

fn run_loop(d: &mut dyn DeviceApi) {
    d.led0_toggle();
    d.delay(1000u16);
}

#[entry]
fn main() -> ! {
    let mut d = Device::new();
    setup(&mut d);

    loop {
        run_loop(&mut d);
    }
}
