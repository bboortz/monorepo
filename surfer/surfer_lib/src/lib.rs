#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

pub mod allocator;
pub mod panic;

extern crate alloc;
extern crate panic_halt;

use alloc::boxed::Box;

pub trait DeviceApi {
    fn delay(&mut self, ms: u16);
    fn led0_toggle(&mut self);
    fn hardware(&self) -> &str;
    fn device_usb(&self) -> Box<dyn DeviceUsbApi>;
    fn as_device(self) -> Box<dyn DeviceApi>;
}

pub trait DeviceUsbApi {
    fn print(&mut self, s: &str);
    fn flush(&mut self);
}
