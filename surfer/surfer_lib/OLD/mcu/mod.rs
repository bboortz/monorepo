// pub mod seeed_studio_xiao_sam21_cortex_m0;
//use seeduino_xiao_m0_rust::mcu::seeed_studio_xiao_sam21_cortex_m0::usb::DeviceUsb;
//use seeduino_xiao_m0_rust::mcu::seeed_studio_xiao_sam21_cortex_m0::Device;

extern crate panic_halt;

pub use bsp::entry;
use xiao_m0 as bsp;

pub trait DeviceApi {
    fn delay(&mut self, ms: u16);
    fn led0_toggle(&mut self);
    fn hardware(&self) -> &str;
    /*
    fn device_usb(&self) -> DeviceUsb;
    fn as_device(&self) -> &Device;
    */
}

pub trait DeviceUsbApi {
    fn print(&mut self, s: &str);
    fn flush(&mut self);
}