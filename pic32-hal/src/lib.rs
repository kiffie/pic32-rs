//! HAL library for PIC32

#![no_std]
#![feature(asm)]

#[cfg(feature = "pic32mx1xxfxxxb")]
pub use pic32mx1xxfxxxb as pac;

#[cfg(feature = "pic32mx4xxfxxxh")]
pub use pic32mx4xxfxxxh as pac;

#[cfg(feature = "pic32mx274fxxxb")]
pub use pic32mx274fxxxb as pac;

use embedded_hal as hal;

pub mod time;
pub mod gpio;
pub mod uart;
pub mod i2c;
pub mod cp0timer;
pub mod coretimer;
pub mod clock;

