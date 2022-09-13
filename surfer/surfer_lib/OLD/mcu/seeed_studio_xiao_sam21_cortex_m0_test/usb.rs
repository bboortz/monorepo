extern crate panic_halt;

use crate::mcu::DeviceUsbApi;

use core::cell::Cell;

use embedded_hal as emb;
use xiao_m0 as bsp;

use bsp::pins::UsbDm;
use bsp::pins::UsbDp;
use bsp::Led1;
use bsp::{hal, pac};
use cortex_m::peripheral::NVIC;
use emb::digital::v2::ToggleableOutputPin;
use hal::clock::GenericClockController;
use hal::usb::UsbBus;
use pac::{interrupt, CorePeripherals};
use usb_device::class_prelude::UsbBus as UsbDeviceBus;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

/*
// static objects
static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
pub static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
pub static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static mut LED_DATA: Option<Led1> = None;

pub fn setup_usb(
    cp: &mut CorePeripherals,
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
    led: Led1,
) {
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::usb_allocator(usb, clocks, pm, dm, dp));
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
        LED_DATA = Some(led.into_mode());
    }

    unsafe {
        cp.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
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
                        serial.write(b"\r\n").unwrap();
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
*/

////
//
//

pub static mut DEVICE_USB: Option<DeviceUsb> = None;
/*
static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
pub static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
pub static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static mut LED_DATA: Option<Led1> = None;
*/

pub struct DeviceUsb<'a> {
    //    usb_allocator: UsbBusAllocator<UsbBus>,
    //    usb_bus: UsbDevice<'a, UsbBus>,
    pub usb_serial: SerialPort<'a, UsbBus>,
    led_data: Led1,
}

impl DeviceUsb<'_> {
    pub fn new(
        cp: &mut CorePeripherals,
        usb: pac::USB,
        clocks: &mut GenericClockController,
        pm: &mut pac::PM,
        dm: impl Into<UsbDm>,
        dp: impl Into<UsbDp>,
        led: Led1,
    ) -> Self {
        let usb_serial;
        {
            /*
            let bus_allocator = unsafe {
                USB_ALLOCATOR = Some(bsp::usb_allocator(usb, clocks, pm, dm, dp));
                USB_ALLOCATOR.as_ref().unwrap()
            };
            */
            let usb_allocator = bsp::usb_allocator(usb, clocks, pm, dm, dp);
            /*
            let bus_allocator = unsafe {
                let usb_allocator = Some(bsp::usb_allocator(usb, clocks, pm, dm, dp));
                usb_allocator.as_ref().unwrap()
            };
            */
            //            usb_allocator.as_ref().unwrap()
            unsafe {
                usb_serial = SerialPort::new(&usb_allocator);
            }
            /*
            let _usb_bus = UsbDeviceBuilder::new(&usb_allocator, UsbVidPid(0xdead, 0xbeef))
                .manufacturer("Hackers University")
                .product("xiao_usb_echo")
                .serial_number("42")
                .device_class(USB_CLASS_CDC)
                .build();
            */
        }
        let led_data = led.into_mode();

        unsafe {
            cp.NVIC.set_priority(interrupt::USB, 1);
            NVIC::unmask(interrupt::USB);
        }
        DeviceUsb {
            //           usb_allocator,
            //           usb_bus,
            usb_serial,
            led_data,
        }
    }
}

pub struct UsbAllocator<UsbBus> {
    usb_allocator: UsbBusAllocator<UsbBus>,
}

impl UsbAllocator<UsbBus> {
    pub fn new(
        cp: &mut CorePeripherals,
        usb: pac::USB,
        clocks: &mut GenericClockController,
        pm: &mut pac::PM,
        dm: impl Into<UsbDm>,
        dp: impl Into<UsbDp>,
        led: Led1,
    ) -> Self {
        let usb_allocator = bsp::usb_allocator(usb, clocks, pm, dm, dp);
        UsbAllocator { usb_allocator }
    }
}

/*
impl DeviceUsbApi for DeviceUsb<'_> {
    fn print(&self, s: &str) {
        //cortex_m::interrupt::free(|serial| {
        /*
        let _ = self.usb_serial.get().write(s.as_bytes());
        let _ = self.usb_serial.flush();
        */
        // });
        //

        cortex_m::interrupt::free(|_| unsafe {
            // self.usb_serial.as_mut().map(|serial| {
            // Skip errors so we can continue the program
            let _ = self.usb_serial.write(s.as_bytes());
            // let _ = serial.flush();
            // });
        });
    }
}
*/

/*
impl Default for DeviceUsb {
    fn default() -> Self {
        Self::new()
    }
}
*/
