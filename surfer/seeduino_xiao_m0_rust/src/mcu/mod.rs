pub mod seeed_studio_xiao_sam21_cortex_m0;
use seeed_studio_xiao_sam21_cortex_m0::Device;

extern crate panic_halt;

pub use bsp::entry;
use xiao_m0 as bsp;

pub trait DeviceApi {
    fn delay(&mut self, ms: u16);
    fn led0_toggle(&mut self);
    fn hardware(&self) -> &str;
    fn as_device(&self) -> &Device;
}
