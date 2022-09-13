pub mod usb;

extern crate panic_halt;

use crate::mcu::DeviceApi;
use usb::DeviceUsb;

use emb::{digital::v2::ToggleableOutputPin, prelude::*};
use embedded_hal as emb;

pub use bsp::entry;
use xiao_m0 as bsp;

use bsp::hal::clock::GenericClockController;
use bsp::hal::delay::Delay;
use bsp::pac;
use bsp::Led0;

pub struct Device {
    delay: bsp::hal::delay::Delay,
    led0: Led0,
}

impl Device {
    pub fn new() -> Self {
        // peripherals
        let mut cp = pac::CorePeripherals::take().unwrap();
        let mut dp = pac::Peripherals::take().unwrap();

        // pins
        let pins = bsp::Pins::new(dp.PORT);

        // clock
        let mut clocks = GenericClockController::with_external_32kosc(
            dp.GCLK,
            &mut dp.PM,
            &mut dp.SYSCTRL,
            &mut dp.NVMCTRL,
        );

        usb::setup_usb(
            &mut cp,
            dp.USB,
            &mut clocks,
            &mut dp.PM,
            pins.usb_dm,
            pins.usb_dp,
            pins.led1.into_mode(),
        );

        let delay = Delay::new(cp.SYST, &mut clocks);
        let led0: Led0 = pins.led0.into_push_pull_output();

        Device { delay, led0 }
    }
}

impl DeviceApi for Device {
    // type UsbItem<'a> = DeviceUsb<'a>;
    fn delay(&mut self, ms: u16) {
        self.delay.delay_ms(ms);
    }

    fn led0_toggle(&mut self) {
        self.led0.toggle().ok();
    }

    fn hardware(&self) -> &str {
        "Seeed Studio XIAO SAMD21 Cortex M0+"
    }

    fn device_usb(&self) -> DeviceUsb {
        DeviceUsb::new()
    }

    fn as_device(&self) -> &Device {
        self
    }
}

impl Default for Device {
    fn default() -> Self {
        Self::new()
    }
}
