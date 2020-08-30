//! HAL library for PIC32

#![no_std]
#![feature(llvm_asm)]

#[cfg(feature = "pic32mx1xxfxxxb")]
pub use pic32mx1xxfxxxb as pac;

#[cfg(feature = "pic32mx2xxfxxxb")]
pub use pic32mx2xxfxxxb as pac;

#[cfg(feature = "pic32mx4xxfxxxh")]
pub use pic32mx4xxfxxxh as pac;

#[cfg(feature = "pic32mx274fxxxb")]
pub use pic32mx274fxxxb as pac;

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
    feature = "pic32mx274fxxxb",
    feature = "pic32mx4xxfxxxh"
))]
#[cfg(feature = "usb-device")]
pub mod usb;
