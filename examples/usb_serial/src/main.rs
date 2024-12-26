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

mod timer1_isr;
use timer1_isr::Timer1Isr;

extern crate alloc;

use embedded_hal::{digital::{OutputPin, StatefulOutputPin}, delay::DelayNs};
use mips_mcu_alloc::MipsMcuHeap;
use mips_rt::entry;
// use panic_halt as _;
use pic32_hal::{clock::Osc, coretimer::Delay, gpio::GpioExt, int::Int, pac, pps::{MapPin, NoPin, PpsExt}, time::Hertz, uart::Uart, usb::UsbBus};
use usb_device::prelude::*;
use usb_log::log_buffer::LogBuffer;
use log::debug;
use embedded_hal_02::serial::Write;

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

static LOG_BUFFER: LogBuffer<4096> = LogBuffer::new();

const SYS_CLOCK: Hertz = Hertz(48_000_000);

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    ALLOCATOR.init();
    log::set_logger(&LOG_BUFFER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    unsafe {
        mips_mcu::interrupt::enable();
    }

    let p = pac::Peripherals::take().unwrap();

    let parts = p.PORTB.split();
    let vpins = p.PPS.split();
    let clock = Osc::new(p.OSC, SYS_CLOCK);
    let int = Int::new(p.INT);
    let mut delay = Delay::new(SYS_CLOCK);

    let mut led = parts.rb5.into_push_pull_output();
    led.set_high().unwrap();

    // #[cfg(feature = "uart-logger")]
    let _timer1_isr = {
        let rxd = NoPin::new().map_pin(vpins.inputs.u2rx);
        let txd = parts
            .rb0
            .into_push_pull_output()
            .map_pin(vpins.outputs.u2tx);
        //let uart = Uart::uart2(p.UART2, &clock, 921600, rxd, txd);
        let uart = Uart::uart2(p.UART2, &clock, 115200, rxd, txd);
        delay.delay_ms(100);

        let (mut tx, _) = uart.split();
        let mut timer1_isr = Timer1Isr::new(p.TMR1);
        let mut log_byte = None;
        timer1_isr.start(&int, move || {
            if log_byte.is_none() {
                log_byte = LOG_BUFFER.read();
            }
            if let Some(byte) = log_byte {
                if !tx.write(byte).is_err() {
                    log_byte = None;
                }
            }
        });
        timer1_isr
    };


    debug!("USB test");
    let usb_bus = UsbBus::new(p.USB);
    let mut serial = usbd_serial::SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::new(LangID::EN)
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")]).unwrap()
        .max_packet_size_0(8).unwrap()
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    debug!("USB test - entering loop");
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
