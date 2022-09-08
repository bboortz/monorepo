#![no_std]
#![no_main]

pub mod mcu;

extern crate panic_halt;

use crate::mcu::DeviceApi;

fn setup() -> mcu::Device {
    let mut d = mcu::Device::new();

    // 40 sec delay
    for _ in 0..40 {
        d.led0_toggle();
        d.delay(100u16);
    }

    // boot screen
    cortex_m::interrupt::free(|_| unsafe {
        crate::mcu::usb::USB_BUS.as_mut().map(|_| {
            crate::mcu::usb::USB_SERIAL.as_mut().map(|serial| {
                // Skip errors so we can continue the program
                let _ = serial.write(
                    b"*****************************************************************\r\n",
                );
                let _ = serial.write(b"* booting device ...\r\n");
                let _ = serial.write(b"* software: surfer v0.2\r\n");
                let _ = serial.write(b"* hardware: Seeed Studio XIAO SAMD21 Cortex M+\r\n");
                let _ = serial.write(
                    b"*****************************************************************\r\n",
                );
            });
        })
    });

    d
}

fn run_loop(d: &mut mcu::Device) {
    d.led0_toggle();
    d.delay(1000u16);
}

#[mcu::entry]
fn main() -> ! {
    let mut d = setup();

    loop {
        run_loop(&mut d);
    }
}
