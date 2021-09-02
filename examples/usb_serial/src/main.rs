//! CDC-ACM serial port example using polling in a busy loop.
//! based on https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/usb_serial.rs
//!
//! Converts characters to upper case and controls the LED with characters
//! '0', '1' and 't'
//!
//! Target: PIC32MX2xx with external 8 MHz crystal and LED connected via a
//! resistor (e.g. 470 Ohms) to RB5 (pin 14)

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use panic_halt as _;

use mips_rt::entry;
use alloc_pic32::Pic32Heap;
use embedded_hal::digital::v2::*;
use pic32_hal::gpio::GpioExt;
use pic32_hal::pac;
use pic32_hal::usb::UsbBus;

use usb_device::prelude::*;

// PIC32 configuration registers for PIC32MX2xx
#[cfg(feature = "pic32mx2xxfxxxb")]
#[link_section = ".configsfrs"]
#[no_mangle]
pub static CONFIGSFRS: [u32; 4] = [
    0x0fffffff, // DEVCFG3
    0xfff979f9, // DEVCFG2
    0xff74cddb, // DEVCFG1
    0x7fffffeb, // DEVCFG0
];

// PIC32 configuration registers for PIC32MX274
#[cfg(feature = "pic32mx2x4fxxxb")]
#[link_section = ".configsfrs"]
#[no_mangle]
pub static CONFIGSFRS: [u32; 4] = [
    0x8fffffff, // DEVCFG3
    0x7fe979d9, // DEVCFG2
    0xff74cdd9, // DEVCFG1
    0xffffffeb, // DEVCFG0
];

#[global_allocator]
static ALLOCATOR: Pic32Heap = Pic32Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    let start = mips_rt::heap_start() as usize;
    let size = 8192; // in bytes
    unsafe { ALLOCATOR.init(start, size) }

    let p = pac::Peripherals::take().unwrap();

    let parts = p.PORTB.split();
    let mut led = parts.rb5.into_push_pull_output();
    led.set_high().unwrap();

    let usb_bus = UsbBus::new(p.USB);
    let mut serial = usbd_serial::SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                for c in buf[0..count].iter_mut() {
                    // Echo back in upper case
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                    // Control LED
                    if *c == b'0' {
                        led.set_low().unwrap();
                    } else if *c == b'1' {
                        led.set_high().unwrap();
                    } else if *c == b'T' {
                        led.toggle().unwrap();
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

#[alloc_error_handler]
fn alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Cannot allocate heap memory: {:?}", layout);
}
