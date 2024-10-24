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

#[cfg(not(feature = "device-selected"))]
compile_error!("This crate requires one device feature to be enabled");

#[cfg(feature = "pic32mx1xxfxxxb")]
use pic32mx2xx as pac_crate;
#[cfg(feature = "pic32mx1xxfxxxb")]
pub use pic32mx2xx::pic32mx1xxfxxxb as pac;

#[cfg(feature = "pic32mx2xxfxxxb")]
use pic32mx2xx as pac_crate;
#[cfg(feature = "pic32mx2xxfxxxb")]
pub use pic32mx2xx::pic32mx2xxfxxxb as pac;

#[cfg(feature = "pic32mx2x4fxxxb")]
use pic32mx2xx as pac_crate;
#[cfg(feature = "pic32mx2x4fxxxb")]
pub use pic32mx2xx::pic32mx2x4fxxxb as pac;

#[cfg(feature = "pic32mx37x")]
use pic32mx470 as pac_crate;
#[cfg(feature = "pic32mx37x")]
pub use pic32mx470::pic32mx37xfxxxl as pac;

#[cfg(feature = "pic32mx47x")]
use pic32mx470 as pac_crate;
#[cfg(feature = "pic32mx47x")]
pub use pic32mx470::pic32mx47xfxxxl as pac;

pub mod adc;
pub mod clock;
pub mod coretimer;
pub mod dma;
pub mod gpio;
pub mod i2c;
pub mod int;
pub mod oc;
pub mod pps;
pub mod spi;
pub mod time;
pub mod timer;
pub mod uart;

#[cfg(any(
    feature = "pic32mx2xxfxxxb",
    feature = "pic32mx2x4fxxxb",
    feature = "pic32mx47x",
))]
#[cfg(feature = "usb-device")]
pub mod usb;
