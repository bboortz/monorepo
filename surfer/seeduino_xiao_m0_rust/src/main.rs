#![no_std]
#![no_main]
// #![feature(generic_associated_types)]

pub mod mcu;

extern crate panic_halt;

use crate::mcu::seeed_studio_xiao_sam21_cortex_m0 as m;
use crate::mcu::DeviceApi;
use crate::mcu::DeviceUsbApi;
//use heapless::String;
use m::usb::DeviceUsb;
use m::Device;

// fn setup<'a>(d: &mut dyn DeviceApi<UsbItem<'a> = DeviceUsb>) {
fn setup(d: &mut dyn DeviceApi) {
    // 40 sec delay
    for _ in 0..40 {
        d.led0_toggle();
        d.delay(100u16);
    }

    // boot screen
    let mut dusb = DeviceUsb::new();
    dusb.print("*****************************************************************\r\n");
    dusb.print("* booting device ...\r\n");
    dusb.print("* software: surfer v0.4\r\n");
    dusb.print("* hardware: ");
    dusb.print(d.hardware());
    dusb.print("\r\n");
    dusb.print("*****************************************************************\r\n");
    dusb.flush();
}

// fn run_loop<'a>(d: &mut dyn DeviceApi<UsbItem<'a> = DeviceUsb>) {
fn run_loop(d: &mut dyn DeviceApi) {
    d.led0_toggle();
    d.delay(1000u16);
}

#[mcu::entry]
fn main() -> ! {
    //let mut d = setup();
    // let mut u: m::Device = *d;
    let mut d = Device::new();
    setup(&mut d);

    loop {
        run_loop(&mut d);
    }
}
