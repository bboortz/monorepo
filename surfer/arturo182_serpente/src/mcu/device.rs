use alloc::boxed::Box;

// use crate::mcu::usb::setup_usb;
use crate::mcu::usb::DeviceUsb;
use surfer_lib::DeviceApi;
use surfer_lib::DeviceUsbApi;

// use emb::digital::v2::ToggleableOutputPin;
use emb::prelude::*;
use embedded_hal as emb;

use bsp::hal::clock::GenericClockController;
use bsp::hal::delay::Delay;
use bsp::pac;
// use bsp::Led0;
use serpente as bsp;

pub struct Device {
    delay: bsp::hal::delay::Delay,
    //  led0: Led0,
    device_usb: DeviceUsb,
}

impl Device {
    pub fn new() -> Self {
        // peripherals
        let cp = pac::CorePeripherals::take().unwrap();
        let mut dp = pac::Peripherals::take().unwrap();

        // pins
        let _pins = bsp::Pins::new(dp.PORT);

        // clock
        let mut clocks = GenericClockController::with_external_32kosc(
            dp.GCLK,
            &mut dp.PM,
            &mut dp.SYSCTRL,
            &mut dp.NVMCTRL,
        );

        /*
        setup_usb(
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
        //        let led0: Led0 = pins.led0.into_push_pull_output();
        let device_usb = DeviceUsb::new();

        Device {
            delay,
            //            led0,
            device_usb,
        }
    }
}

impl DeviceApi for Device {
    // type UsbItem<'a> = DeviceUsb<'a>;
    fn delay(&mut self, ms: u16) {
        self.delay.delay_ms(ms);
    }

    fn led0_toggle(&mut self) {
        //        self.led0.toggle().ok();
    }

    fn hardware(&self) -> &str {
        "Seeed Studio XIAO SAMD21 Cortex M0+"
    }

    fn device_usb(&self) -> Box<dyn DeviceUsbApi> {
        Box::new(self.device_usb)
    }

    fn to_device(self) -> Box<dyn DeviceApi> {
        Box::new(self)
    }
}

impl Default for Device {
    fn default() -> Self {
        Self::new()
    }
}
