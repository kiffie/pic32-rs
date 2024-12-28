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

use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use mips_mcu_alloc::MipsMcuHeap;
use mips_rt::entry;
use panic_halt as _;
use pic32_hal::{gpio::GpioExt, pac, usb::UsbBus};
use usb_device::prelude::*;

#[cfg(feature = "pic32mx2x4fxxxb")]
use pic32_config_sector::pic32mx2x4::*;
#[cfg(feature = "pic32mx2xxfxxxb")]
use pic32_config_sector::pic32mx2xx::*;

// PIC32 configuration registers for PIC32MX2xx
#[cfg(feature = "pic32mx2xxfxxxb")]
#[link_section = ".configsfrs"]
#[used]
pub static CONFIGSFRS: ConfigSector = ConfigSector::default()
    // DEVCFG3
    .FVBUSONIO(FVBUSONIO::OFF)
    .FUSBIDIO(FUSBIDIO::OFF)
    // DEVCFG2
    .FPLLODIV(FPLLODIV::DIV_2)
    .UPLLEN(UPLLEN::ON)
    .UPLLIDIV(UPLLIDIV::DIV_2)
    .FPLLMUL(FPLLMUL::MUL_24)
    .FPLLIDIV(FPLLIDIV::DIV_2)
    // DEVCFG 1
    .FWDTEN(FWDTEN::OFF)
    .WDTPS(WDTPS::PS1048576)
    .FPBDIV(FPBDIV::DIV_1)
    .POSCMOD(POSCMOD::XT)
    .FSOSCEN(FSOSCEN::OFF)
    .FNOSC(FNOSC::PRIPLL)
    // DEVCFG 0
    .JTAGEN(JTAGEN::OFF)
    .build();

// PIC32 configuration registers for PIC32MX274
#[cfg(feature = "pic32mx2x4fxxxb")]
#[link_section = ".configsfrs"]
#[used]
pub static CONFIGSFRS: ConfigSector = ConfigSector::default()
    // DEVCFG3
    .FUSBIDIO(FUSBIDIO::OFF)
    // DEVCFG2
    .FDSEN(FDSEN::OFF)
    .FPLLODIV(FPLLODIV::DIV_2)
    .UPLLEN(UPLLEN::ON)
    .UPLLIDIV(UPLLIDIV::DIV_2)
    .FPLLICLK(FPLLICLK::PLL_POSC)
    .FPLLMUL(FPLLMUL::MUL_24)
    .FPLLIDIV(FPLLIDIV::DIV_2)
    // DEVCFG1
    .FWDTEN(FWDTEN::OFF)
    .WDTPS(WDTPS::PS1048576)
    .FPBDIV(FPBDIV::DIV_1)
    .POSCMOD(POSCMOD::XT)
    .FSOSCEN(FSOSCEN::OFF)
    .FNOSC(FNOSC::SPLL)
    // DEVCFG0
    .JTAGEN(JTAGEN::OFF)
    .build();

#[global_allocator]
static ALLOCATOR: MipsMcuHeap = MipsMcuHeap::empty();

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    ALLOCATOR.init();

    let p = pac::Peripherals::take().unwrap();

    let parts = p.PORTB.split();
    let mut led = parts.rb5.into_push_pull_output();
    led.set_high().unwrap();

    let usb_bus = UsbBus::new(p.USB);
    let mut serial = usbd_serial::SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::new(LangID::EN)
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")])
        .unwrap()
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
