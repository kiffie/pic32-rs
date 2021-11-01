//! Calculate constant values for PIC32 configuration words.
//!
//! This crate defines const structures that can be used to calculate values of
//! configuration words to be stored in the configuration word section of a
//! Flash memory image for PIC32 microcontrollers. The `build()` method returns
//! a constant struct to be output to the configuration word section (typically
//! `.configsfrs`).
//!
//! Example:
//! ```
//! use pic32_config_sfrs::pic32mx2xx::*;
//!
//! #[link_section = ".configsfrs"]
//! #[used]
//! pub static CONFIGSFRS: ConfigSector = ConfigSector::default()
//!     .FVBUSONIO(FVBUSONIO::OFF)
//!     .FUSBIDIO(FUSBIDIO::OFF)
//!     .IOL1WAY(IOL1WAY::OFF)
//!     .PMDL1WAY(PMDL1WAY::OFF)
//!     .FPLLIDIV(FPLLIDIV::DIV_2)
//!     .FPLLMUL(FPLLMUL::MUL_20)
//!     .FPLLODIV(FPLLODIV::DIV_2)
//!     .FNOSC(FNOSC::FRCPLL)
//!     .FSOSCEN(FSOSCEN::OFF)
//!     .FPBDIV(FPBDIV::DIV_1)
//!     .FWDTEN(FWDTEN::OFF)
//!     .JTAGEN(JTAGEN::OFF)
//!     .ICESEL(ICESEL::ICS_PGx1)
//!     .build();
//! ```
//!
#![no_std]

/// Configuration sector struct and builder for PIC32MX1xx
pub mod pic32mx1xx;

/// Configuration sector struct and builder for PIC32MX2xx
pub mod pic32mx2xx;

/// Configuration sector struct and builder for PIC32MX1x4 (XLP)
pub mod pic32mx1x4;

/// Configuration sector struct and builder for PIC32MX2x4 (XLP)
pub mod pic32mx2x4;

/// Configuration sector struct and builder for PIC32MZEF
pub mod pic32mzef;
