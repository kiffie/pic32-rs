//! A hardware abstraction layer for some PIC32 microcontrollers.
//!
//! This crate provides a thin low-level API on top of the register access API
//! implemented by the PAC crate. The MCU can be selected by means of specific
//! features.
//!
//! | Feature | Devices |
//! |:--------------:|:-------:|
//! | pic32mx1xxfxxxb | 28-pin PIC32MX1xx |
//! | pic32mx2xxfxxxb | 28-pin PIC32MX2xx |
//! | pic32mx2x4fxxxb | 28-pin PIC32MX2xx XLP |
//!
//! This documentation has been generated with `--features pic32mx2xxfxxxb`.

#![no_std]
#![feature(llvm_asm)]

use pic32mx2xx as pac_crate;

#[cfg(feature = "pic32mx1xxfxxxb")]
pub use pic32mx2xx::pic32mx1xxfxxxb as pac;

#[cfg(feature = "pic32mx2xxfxxxb")]
pub use pic32mx2xx::pic32mx2xxfxxxb as pac;

#[cfg(feature = "pic32mx2x4fxxxb")]
pub use pic32mx2xx::pic32mx2x4fxxxb as pac;

#[cfg(feature = "pic32mx4xxfxxxh")]
pub use pic32mx4xxfxxxh as pac;

use embedded_hal as hal;

pub mod time;
pub mod int;
pub mod gpio;
pub mod pps;
pub mod uart;
pub mod spi;
pub mod i2c;
pub mod coretimer;
pub mod clock;
pub mod dma;

#[cfg(any(
    feature = "pic32mx2xxfxxxb",
    feature = "pic32mx2x4fxxxb",
    feature = "pic32mx4xxfxxxh"
))]
#[cfg(feature = "usb-device")]
pub mod usb;
