#![no_std]
#![no_main]

pub mod mcu;

extern crate panic_halt;

use crate::mcu::seeed_studio_xiao_sam21_cortex_m0 as m;
use crate::mcu::DeviceApi;

fn setup(d: &mut dyn DeviceApi) {
    // 40 sec delay
    for _ in 0..40 {
        d.led0_toggle();
        d.delay(100u16);
    }

    // boot screen
    cortex_m::interrupt::free(|_| unsafe {
        m::usb::USB_BUS.as_mut().map(|_| {
            m::usb::USB_SERIAL.as_mut().map(|serial| {
                // Skip errors so we can continue the program
                let _ = serial.write(
                    b"*****************************************************************\r\n",
                );
                let _ = serial.write(b"* booting device ...\r\n");
                let _ = serial.write(b"* software: surfer v0.3\r\n");
                let _ = serial.write(b"* hardware: ");
                let _ = serial.write(d.hardware().as_bytes());
                let _ = serial.write(b"\r\n");
                let _ = serial.write(
                    b"*****************************************************************\r\n",
                );
                let _ = serial.flush();
            });
        })
    });
}

fn run_loop(d: &mut dyn DeviceApi) {
    d.led0_toggle();
    d.delay(1000u16);
}

#[mcu::entry]
fn main() -> ! {
    //let mut d = setup();
    // let mut u: m::Device = *d;
    let mut d = m::Device::new();
    setup(&mut d);

    loop {
        run_loop(&mut d);
    }
}
