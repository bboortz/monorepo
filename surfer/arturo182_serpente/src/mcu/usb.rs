use surfer_lib::DeviceUsbApi;

/*
use embedded_hal as emb;
use serpente as bsp;
*/

/*
use bsp::pins::UsbDm;
use bsp::pins::UsbDp;
use bsp::Led1;
*/
// use bsp::{hal, pac};
/*
use cortex_m::peripheral::NVIC;
use emb::digital::v2::ToggleableOutputPin;
use hal::clock::GenericClockController;
*/
// use hal::usb::UsbBus;
/*
use pac::{interrupt, CorePeripherals};
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};
*/

// static objects

/*
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

#[allow(clippy::option_map_unit_fn)]
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
                        //serial.write(&[c.clone()]).unwrap();
                        serial.write(&[*c]).unwrap();
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

#[derive(Debug, Copy, Clone)]
pub struct DeviceUsb {}

impl DeviceUsb {
    pub fn new() -> Self {
        DeviceUsb {}
    }
}

impl DeviceUsbApi for DeviceUsb {
    #[allow(clippy::option_map_unit_fn)]
    fn print(&mut self, s: &str) {
        // boot screen
        /*
        cortex_m::interrupt::free(|_| unsafe {
            USB_SERIAL.as_mut().map(|serial| {
                let _ = serial.write(s.as_bytes());
            });
        });
        */
    }

    #[allow(clippy::option_map_unit_fn)]
    fn flush(&mut self) {
        /*
          cortex_m::interrupt::free(|_| unsafe {
              USB_SERIAL.as_mut().map(|serial| {
                  let _ = serial.flush();
              });
          });
        */
    }
}

impl Default for DeviceUsb {
    fn default() -> Self {
        Self::new()
    }
}
