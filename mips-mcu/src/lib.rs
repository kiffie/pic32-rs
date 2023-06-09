//! Low level access to MIPS MCU cores
//!
//! This crate includes Rust function to deal with low level aspects related to
//! the MIPS MCU cores (e.g. the M4K core). Routines requiring special or
//! privileged instructions are included in a binary library, thereby avoiding
//! inline assembly.

#![no_std]

pub mod core_timer;
pub mod fmt;
pub mod interrupt;

#[cfg(feature = "critical-section-single-core")]
pub mod critical_section;

/// Physical address
#[derive(Clone, Copy, Debug, Default)]
pub struct PhysicalAddress {
    addr: usize,
}

impl PhysicalAddress {
    /// Create a PhysicalAddress by specifying its value directly
    pub const fn from_usize(addr: usize) -> Self {
        Self { addr }
    }

    /// get the value of the DmaAddress. Useful for programming of bus master
    /// peripherals, which typically access physical memory
    pub fn address(&self) -> usize {
        self.addr
    }
}
