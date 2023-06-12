#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

pub mod allocator;
pub mod panic;

extern crate alloc;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use alloc::boxed::Box;

pub static SURFER_VERSION: &str = "0.2";

pub trait DeviceApi {
    fn delay(&mut self, ms: u16);
    fn led0_toggle(&mut self);
    fn hardware(&self) -> &str;
    fn device_usb(&self) -> Box<dyn DeviceUsbApi>;
    fn to_device(self) -> Box<dyn DeviceApi>;
}

pub trait DeviceUsbApi {
    fn print(&mut self, s: &str);
    fn flush(&mut self);
}
