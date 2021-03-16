//! I2C oled display example

#![no_main]
#![no_std]
#![feature(panic_info_message)]

use core::panic::PanicInfo;

use mips_rt;
use mips_rt::entry;
use tinylog;
use tinylog::{debug, error, info};

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use embedded_hal::serial::Write;

use pic32_hal::clock::Osc;
use pic32_hal::coretimer::Delay;
use pic32_hal::gpio::GpioExt;
use pic32_hal::i2c::{Fscl, I2c};
use pic32_hal::pac;
use pic32_hal::pac::UART1;
use pic32_hal::time::U32Ext;
use pic32_hal::uart::{Tx, Uart};

use ssd1306::mode::GraphicsMode;
use ssd1306::Builder;

use embedded_graphics::fonts::{Font12x16, Font6x12, Font6x8, Font8x16};
use embedded_graphics::image::Image1BPP;
use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;

const TL_LOGLEVEL: tinylog::Level = tinylog::Level::Debug;

// PIC32 configuration registers for PIC32MX150
#[cfg(feature = "pic32mx1xxfxxxb")]
#[link_section = ".configsfrs"]
#[no_mangle]
pub static CONFIGSFRS: [u32; 4] = [
    0xdfffffff, // DEVCFG3
    0xfff9ffd9, // DEVCFG2
    0xff7fcfd9, // DEVCFG1
    0x7ffffffb, // DEVCFG0
];

// PIC32 configuration registers for PIC32MX274
#[cfg(feature = "pic32mx274fxxxb")]
#[link_section = ".configsfrs"]
#[no_mangle]
pub static CONFIGSFRS: [u32; 4] = [
    0xcf3fffff, // DEVCFG3
    0x7fe9f9d9, // DEVCFG2
    0xff74cfd9, // DEVCFG1
    0xfffffff3, // DEVCFG0
];

static mut LOG_TX: Option<Tx<UART1>> = None;

fn log_bwrite_all(buffer: &[u8]) {
    unsafe {
        if let Some(ref mut tx) = LOG_TX {
            for b in buffer {
                while match tx.write(*b) {
                    Ok(()) => false,
                    Err(_) => true,
                } {}
            }
        }
    }
}

#[entry]
fn main() -> ! {
    //configure IO ports for UART
    let p = pac::Peripherals::take().unwrap();
    let pps = p.PPS;
    pps.rpa0r.write(|w| unsafe { w.rpa0r().bits(0b0001) }); // U1TX on RPA0
    pps.u1rxr.write(|w| unsafe { w.u1rxr().bits(0b0010) }); // U1RX on RPA4

    // setup clock control object
    let sysclock = 40_000_000_u32.hz();
    #[cfg(feature = "pic32mx1xxfxxxb")]
    let clock = Osc::new(p.OSC, sysclock);
    #[cfg(feature = "pic32mx274fxxxb")]
    let clock = Osc::new(p.CRU, sysclock);

    let mut timer = Delay::new(sysclock);

    /* initialize clock control and uart */
    let uart = Uart::uart1(p.UART1, &clock, 115200);
    timer.delay_ms(10u32);
    let (tx, _) = uart.split();
    unsafe { LOG_TX = Some(tx) };
    tinylog::set_bwrite_all(log_bwrite_all);
    info!("I2C oled display example");
    debug!("sysclock = {} Hz", sysclock.0);

    /* LED */
    let parts = p.PORTB.split();
    let mut led = parts.rb0.into_push_pull_output();

    let mut state = false;

    led.set_high().unwrap();
    for _i in 1..10 {
        led.toggle().unwrap();
        timer.delay_ms(100);
    }

    info!("initializing display");
    let i2c = I2c::i2c1(p.I2C1, clock.pb_clock(), Fscl::F400KHZ);
    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
    disp.init().unwrap();
    disp.flush().unwrap();

    disp.draw(
        Font6x8::render_str("Hello World 6x8")
            .translate(Coord::new(0, 0))
            .into_iter(),
    );

    disp.draw(
        Font6x12::render_str("Hello World 6x12")
            .translate(Coord::new(0, 8))
            .into_iter(),
    );

    disp.draw(
        Font8x16::render_str("Hello World 8x16")
            .translate(Coord::new(0, 20))
            .into_iter(),
    );

    disp.draw(
        Font12x16::render_str("Hello 12x16")
            .translate(Coord::new(0, 36))
            .into_iter(),
    );

    disp.flush().unwrap();

    timer.delay_ms(10000u32);

    let bitmap = include_bytes!("./rust.raw");

    info!("starting loop");
    let mut x = 0;
    let mut move_right = true;

    loop {
        let im = Image1BPP::new(bitmap, 64, 64).translate(Coord::new(x, 0));
        disp.clear();
        disp.draw(im.into_iter());
        disp.flush().unwrap();
        state = !state;
        if move_right {
            if x < 64 {
                x += 1;
            } else {
                debug!("left");
                move_right = false;
            }
        } else {
            if x > 0 {
                x -= 1;
            } else {
                debug!("right");
                move_right = true;
            }
        }
    }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo<'_>) -> ! {
    if let Some(s) = panic_info.message() {
        error!("Panic: {:?}", s);
    } else {
        error!("Panic");
    }
    error!("entering endless loop.");
    loop {}
}
