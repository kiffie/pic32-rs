//! Low level access to MIPS MCU cores

#![no_std]

pub mod interrupt;
pub mod fmt;

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
