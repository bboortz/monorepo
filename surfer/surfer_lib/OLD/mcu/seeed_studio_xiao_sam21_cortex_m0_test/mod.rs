pub mod usb;

extern crate panic_halt;

use crate::mcu::DeviceApi;
use crate::mcu::DeviceUsbApi;
use usb::DeviceUsb;

use emb::{digital::v2::ToggleableOutputPin, prelude::*};
use embedded_hal as emb;

pub use bsp::entry;
use xiao_m0 as bsp;

use bsp::hal::clock::GenericClockController;
use bsp::hal::delay::Delay;
use bsp::pac;
use bsp::Led0;

pub struct Device<'dusb> {
    delay: bsp::hal::delay::Delay,
    led0: Led0,
    pub dusb: DeviceUsb<'dusb>,
}

impl Device<'_> {
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

        // usb
        let dusb = DeviceUsb::new(
            &mut cp,
            dp.USB,
            &mut clocks,
            &mut dp.PM,
            pins.usb_dm,
            pins.usb_dp,
            pins.led1.into_mode(),
        );
        /*
        usb::setup_usb(
            &mut cp,
            dp.USB,
            &mut clocks,
            &mut dp.PM,
            pins.usb_dm,
            pins.usb_dp,
            pins.led1.into_mode(),
        );
        */

        let delay = Delay::new(cp.SYST, &mut clocks);
        let led0: Led0 = pins.led0.into_push_pull_output();

        Device { delay, led0, dusb }
    }
}

impl DeviceApi for Device<'_> {
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

    fn as_device(&self) -> &Device {
        self
    }

    /*
    fn usb<'a>(&self) -> Option<Self::UsbItem<'a>> {
        self.dusb
    }
    */
}

impl Default for Device<'_> {
    fn default() -> Self {
        Self::new()
    }
}
