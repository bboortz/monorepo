#![no_std]
#![no_main]

// pub mod usb;

extern crate panic_halt;

use embedded_hal as emb;
use xiao_m0 as bsp;

use bsp::hal::delay::Delay;
use bsp::{entry, Led0, Led1};
use bsp::{hal, pac};
use cortex_m::{asm::delay as asm_delay, peripheral::NVIC};
use emb::{digital::v2::ToggleableOutputPin, prelude::*};
use hal::clock::GenericClockController;
use pac::{interrupt, CorePeripherals, Peripherals};

// types
// type PA17 = atsamd_hal::gpio::v2::pin::PA17;
//type DevicePin = atsamd_hal::gpio::v2::Pin<PA17, atsamd_hal::gpio::v2::Output<atsamd_hal::gpio::v2::PushPull>>;

use hal::usb::UsbBus;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

// static objects
static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static mut LED_DATA: Option<Led1> = None;

fn setup() -> (Delay, Led0) {
    // peripherals
    let mut cp = pac::CorePeripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();

    // clock
    let mut clocks = GenericClockController::with_external_32kosc(
        dp.GCLK,
        &mut dp.PM,
        &mut dp.SYSCTRL,
        &mut dp.NVMCTRL,
    );
    let mut delay = Delay::new(cp.SYST, &mut clocks);

    // pins
    let pins = bsp::Pins::new(dp.PORT);

    // usb::setup_usb(&mut cp, &mut dp, &mut clocks, &mut pins);

    // usb
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::usb_allocator(
            dp.USB,
            &mut clocks,
            &mut dp.PM,
            pins.usb_dm,
            pins.usb_dp,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };
    unsafe {
        USB_SERIAL = Some(SerialPort::new(bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0xdead, 0xbeef))
                .manufacturer("Hackers University")
                .product("xiao_usb_echo")
                .serial_number("42")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
        LED_DATA = Some(pins.led1.into_mode());
    }

    unsafe {
        cp.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    // boot screen
    delay.delay_ms(4000u16);

    cortex_m::interrupt::free(|_| unsafe {
        USB_BUS.as_mut().map(|_| {
            USB_SERIAL.as_mut().map(|serial| {
                // Skip errors so we can continue the program
                let _ = serial.write(b"**********************************************\r\n");
                let _ = serial.write(b"* booting device ...\r\n");
                let _ = serial.write(b"* surfer v0.1\r\n");
                let _ = serial.write(b"**********************************************\r\n");
            });
        })
    });

    // returning elements
    let led: Led0 = pins.led0.into_push_pull_output();
    (delay, led)
}

fn run_loop(delay: &mut Delay, led: &mut Led0) {
    led.toggle().ok();
    delay.delay_ms(1000u16);
}

#[entry]
fn main() -> ! {
    let (mut delay, mut led) = setup();

    loop {
        run_loop(&mut delay, &mut led);
    }
}

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    for (i, c) in buf.iter().enumerate() {
                        if i >= count {
                            break;
                        }
                        serial.write(&[c.clone()]).unwrap();
                        LED_DATA.as_mut().map(|led| led.toggle());
                    }
                };
            });
        });
    };
}

#[interrupt]
fn USB() {
    poll_usb();
}
