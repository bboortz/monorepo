#![no_std]
#![no_main]

pub mod usb;

extern crate panic_halt;

use embedded_hal as emb;
use xiao_m0 as bsp;

use bsp::hal::delay::Delay;
use bsp::{entry, Led0};
use bsp::{hal, pac};
use emb::{digital::v2::ToggleableOutputPin, prelude::*};
use hal::clock::GenericClockController;

pub fn setup_mcu() -> (Delay, Led0) {
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
    usb::setup_usb(
      &mut cp,
      dp.USB,
            &mut clocks,
            &mut dp.PM,
            pins.usb_dm,
            pins.usb_dp,
            pins.led1.into_mode()
            );

    let mut delay = Delay::new(cp.SYST, &mut clocks);


    // boot screen
    delay.delay_ms(4000u16);

    /*

    cortex_m::interrupt::free(|_| unsafe {
        usb::USB_BUS.as_mut().map(|_| {
            usb::USB_SERIAL.as_mut().map(|serial| {
                // Skip errors so we can continue the program
                let _ = serial.write(b"**********************************************\r\n");
                let _ = serial.write(b"* booting device ...\r\n");
                let _ = serial.write(b"* surfer v0.1\r\n");
                let _ = serial.write(b"**********************************************\r\n");
            });
        })
    });
    */

    // returning elements
    let led: Led0 = pins.led0.into_push_pull_output();
    (delay, led)
}

