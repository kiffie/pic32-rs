//! HAL library for PIC32

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
