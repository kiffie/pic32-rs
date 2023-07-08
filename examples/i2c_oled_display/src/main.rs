//! I2C oled display example

#![no_main]
#![no_std]

use core::fmt::Write;
use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{
        ascii::{FONT_10X20, FONT_6X9, FONT_8X13},
        MonoTextStyle,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_hal::{
    blocking::delay::DelayMs,
    digital::v2::{OutputPin, ToggleableOutputPin},
};
use mips_rt::{self, entry};
use panic_halt as _;
use pic32_hal::{
    clock::Osc,
    coretimer::Delay,
    gpio::GpioExt,
    i2c::{Fscl, I2c},
    pac,
    pps::{MapPin, NoPin, PpsExt},
    pps_no_pin,
    time::U32Ext,
    uart::Uart,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[cfg(feature = "pic32mx1xxfxxxb")]
use pic32_config_sector::pic32mx1xx::*;
#[cfg(feature = "pic32mx2x4fxxxb")]
use pic32_config_sector::pic32mx2x4::*;
#[cfg(feature = "pic32mx2xxfxxxb")]
use pic32_config_sector::pic32mx2xx::*;

// PIC32 configuration registers for PIC32MX150
#[cfg(feature = "pic32mx1xxfxxxb")]
#[link_section = ".configsfrs"]
#[used]
pub static CONFIGSFRS: ConfigSector = ConfigSector::default()
    // DEVCFG3
    .IOL1WAY(IOL1WAY::OFF)
    .PMDL1WAY(PMDL1WAY::OFF)
    // DEVCFG2
    .FPLLIDIV(FPLLIDIV::DIV_2)
    .FPLLMUL(FPLLMUL::MUL_20)
    .FPLLODIV(FPLLODIV::DIV_2)
    // DEVCFG1
    .FWDTEN(FWDTEN::OFF)
    .FPBDIV(FPBDIV::DIV_1)
    .FSOSCEN(FSOSCEN::OFF)
    .FNOSC(FNOSC::FRCPLL)
    // DEVCFG0
    .JTAGEN(JTAGEN::OFF)
    .ICESEL(ICESEL::ICS_PGx1)
    .build();

// PIC32 configuration registers for PIC32MX1xx and PIC32MX2xx
#[cfg(any(feature = "pic32mx1xxfxxxb", feature = "pic32mx2xxfxxxb"))]
#[link_section = ".configsfrs"]
#[used]
pub static CONFIGSFRS: ConfigSector = ConfigSector::default()
    .FVBUSONIO(FVBUSONIO::OFF)
    .FUSBIDIO(FUSBIDIO::OFF)
    .IOL1WAY(IOL1WAY::OFF)
    .PMDL1WAY(PMDL1WAY::OFF)
    .FPLLIDIV(FPLLIDIV::DIV_2)
    .FPLLMUL(FPLLMUL::MUL_20)
    .FPLLODIV(FPLLODIV::DIV_2)
    .FNOSC(FNOSC::FRCPLL)
    .FSOSCEN(FSOSCEN::OFF)
    .FPBDIV(FPBDIV::DIV_1)
    .FWDTEN(FWDTEN::OFF)
    .JTAGEN(JTAGEN::OFF)
    .ICESEL(ICESEL::ICS_PGx1)
    .build();

// PIC32 configuration registers for PIC32MX274
#[cfg(feature = "pic32mx2x4fxxxb")]
#[link_section = ".configsfrs"]
#[used]
pub static CONFIGSFRS: ConfigSector = ConfigSector::default()
    // DEVCFG3
    .IOL1WAY(IOL1WAY::OFF)
    .PMDL1WAY(PMDL1WAY::OFF)
    .AI2C2(AI2C2::OFF)
    .AI2C1(AI2C1::OFF)
    // DEVCFG2
    .FDSEN(FDSEN::OFF)
    .BOREN(BOREN::OFF)
    .FPLLODIV(FPLLODIV::DIV_2)
    .UPLLIDIV(UPLLIDIV::DIV_2)
    .FPLLMUL(FPLLMUL::MUL_20)
    .FPLLIDIV(FPLLIDIV::DIV_2)
    // DEVCFG1
    .FWDTEN(FWDTEN::OFF)
    .WDTPS(WDTPS::PS1048576)
    .FPBDIV(FPBDIV::DIV_1)
    .FSOSCEN(FSOSCEN::OFF)
    .FNOSC(FNOSC::SPLL)
    // FEVCFG0
    .ICESEL(ICESEL::ICS_PGx2)
    .JTAGEN(JTAGEN::OFF)
    .DEBUG(DEBUG::OFF)
    .build();

#[entry]
fn main() -> ! {
    //configure IO ports for UART
    let p = pac::Peripherals::take().unwrap();
    let portb = p.PORTB.split();
    let vpins = p.PPS.split();

    // setup clock control object
    let sysclock = 40_000_000_u32.hz();
    #[cfg(any(feature = "pic32mx1xxfxxxb", feature = "pic32mx2xxfxxxb"))]
    let clock = Osc::new(p.OSC, sysclock);
    #[cfg(feature = "pic32mx2x4fxxxb")]
    let clock = Osc::new(p.CRU, sysclock);

    let mut timer = Delay::new(sysclock);

    /* initialize clock control and uart */
    let txd = portb
        .rb0
        .into_push_pull_output()
        .map_pin(vpins.outputs.u2tx);
    let uart = Uart::uart2(p.UART2, &clock, 115200, pps_no_pin!(vpins.inputs.u2rx), txd);
    timer.delay_ms(10u32);
    let (mut tx, _) = uart.split();
    writeln!(tx, "I2C oled display example").unwrap();

    /* LED */
    let mut led = portb.rb5.into_push_pull_output();

    let mut state = false;

    led.set_high().unwrap();
    for _i in 1..10 {
        led.toggle().unwrap();
        timer.delay_ms(100);
    }

    writeln!(tx, "initializing display").unwrap();
    let i2c = I2c::i2c1(p.I2C1, clock.pb_clock(), Fscl::F400KHZ);
    let interface = I2CDisplayInterface::new(i2c);
    let mut disp = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    disp.init().unwrap();

    Text::new(
        "Hello 10x20",
        Point::new(0, 20),
        MonoTextStyle::new(&FONT_10X20, BinaryColor::On),
    )
    .draw(&mut disp)
    .unwrap();

    Text::new(
        "Hello World 8x13",
        Point::new(0, 33),
        MonoTextStyle::new(&FONT_8X13, BinaryColor::On),
    )
    .draw(&mut disp)
    .unwrap();

    Text::new(
        "Hello World 6x9",
        Point::new(0, 42),
        MonoTextStyle::new(&FONT_6X9, BinaryColor::On),
    )
    .draw(&mut disp)
    .unwrap();

    disp.flush().unwrap();

    timer.delay_ms(10000u32);

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);

    writeln!(tx, "starting loop").unwrap();
    let mut x = 0;
    let mut move_right = true;

    loop {
        let im = Image::new(&raw, Point::new(x, 0));
        disp.clear(BinaryColor::Off).unwrap();
        im.draw(&mut disp).unwrap();
        disp.flush().unwrap();
        state = !state;
        if move_right {
            if x < 64 {
                x += 1;
            } else {
                writeln!(tx, "left").unwrap();
                led.set_high().unwrap();
                move_right = false;
            }
        } else if x > 0 {
            x -= 1;
        } else {
            writeln!(tx, "right").unwrap();
            led.set_low().unwrap();
            move_right = true;
        }
    }
}
