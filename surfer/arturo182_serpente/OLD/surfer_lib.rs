#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
// #![feature(box_syntax)]
// #![feature(lang_items, ptr_internals)]
// #![feature(default_alloc_error_handler)]
// #![feature(alloc)]
//#[global_allocator]
// #![feature(generic_associated_types)]

// pub mod allocator_cortex_m;

extern crate alloc;

use alloc::boxed::Box;

pub trait DeviceApi {
    fn delay(&mut self, ms: u16);
    fn led0_toggle(&mut self);
    fn hardware(&self) -> &str;
    fn device_usb(&self) -> Box<dyn DeviceUsbApi>;
    //    fn as_device(&self) -> Box<dyn DeviceApi>;
}

pub trait DeviceUsbApi {
    fn print(&mut self, s: &str);
    fn flush(&mut self);
}
