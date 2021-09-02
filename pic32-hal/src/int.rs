//! Interrupt controller
//!
//! Enable/disable and set priorities of interrupts in Multi-vectored mode

use crate::pac::INT;
use crate::pac_crate::{RegisterSpec, Reg};
use core::convert::TryFrom;
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};

/// Interrupt source (from PAC)
///
/// Multiple interrupt sources can share a single interrupt vector.
pub use crate::pac::interrupt::InterruptSource;

/// Interrupt vector (from PAC)
///
/// One interrupt vector may assigned more than one interrupt source.
pub use crate::pac::interrupt::Interrupt;

/// Invalid numerical priority level
#[derive(Debug, Copy, Clone)]
pub struct PriorityConvertError;

/// Interrupt priority level assignable to an interrupt vector
#[derive(Copy, Clone, Debug)]
pub struct Ipl(u8);

impl TryFrom<u8> for Ipl {
    type Error = PriorityConvertError;

    fn try_from(ipl: u8) -> Result<Self, Self::Error> {
        if ipl <= 7 {
            Ok(Ipl(ipl))
        } else {
            Err(PriorityConvertError)
        }
    }
}

impl From<Ipl> for u8 {
    fn from(ipl: Ipl) -> Self {
        ipl.0
    }
}

/// Interrupt priority level 1 (lowest interrupt priority; interrupts disabled)
pub const IPL0: Ipl = Ipl(0);
/// Interrupt priority level 1
pub const IPL1: Ipl = Ipl(1);
/// Interrupt priority level 2
pub const IPL2: Ipl = Ipl(2);
/// Interrupt priority level 3
pub const IPL3: Ipl = Ipl(3);
/// Interrupt priority level 4
pub const IPL4: Ipl = Ipl(4);
/// Interrupt priority level 5
pub const IPL5: Ipl = Ipl(5);
/// Interrupt priority level 6
pub const IPL6: Ipl = Ipl(6);
/// Interrupt priority level 6 (highest interrupt priority)
pub const IPL7: Ipl = Ipl(7);

/// Interrupt sub priority levels
#[derive(Copy, Clone, Debug)]
pub struct Isl(u8);

impl TryFrom<u8> for Isl {
    type Error = PriorityConvertError;

    fn try_from(isl: u8) -> Result<Self, Self::Error> {
        if isl <= 3 {
            Ok(Isl(isl))
        } else {
            Err(PriorityConvertError)
        }
    }
}

impl From<Isl> for u8 {
    fn from(isl: Isl) -> Self {
        isl.0
    }
}

/// Interrupt sub priority level 0 (lowest interrupt sub priority)
pub const ISL0: Isl = Isl(0);
/// Interrupt sub priority level 1
pub const ISL1: Isl = Isl(1);
/// Interrupt sub priority level 2
pub const ISL2: Isl = Isl(2);
/// Interrupt sub priority level 3 (highest interrupt sub priority)
pub const ISL3: Isl = Isl(3);

/// Access to interrupt controller.
pub struct Int {
    _int: PhantomData<INT>,
}

impl Int {
    /// Create a new instance. Configures the interrupt controller to work in
    /// the Multi-vectored mode. The Cause register of the MIPS core must be
    /// already configured for use with External Interrupt Controller (EIC);
    /// done by startup code.
    pub fn new(int: INT) -> Int {
        int.intconset.write(|w| w.mvec().bit(true));
        Int { _int: PhantomData }
    }

    fn bitaddr<REG: RegisterSpec>(s: InterruptSource, breg: &Reg<REG>) -> (*mut u32, u32) {
        let regndx = (s as usize) / 32;
        let mask = 1 << ((s as usize) % 32);
        let base = breg as *const _ as usize;
        let reg = (base + regndx * 0x10) as *mut u32;
        (reg, mask)
    }

    /// Enable interrupts for a specific source
    pub fn ei(&self, s: InterruptSource) {
        let (reg, mask) = Self::bitaddr(s, unsafe { &(*INT::ptr()).iec0set });
        unsafe { write_volatile(reg, mask) };
    }

    /// Disable interrupts for a specific source
    pub fn di(&self, s: InterruptSource) {
        let (reg, mask) = Self::bitaddr(s, unsafe { &(*INT::ptr()).iec0clr });
        unsafe { write_volatile(reg, mask) };
    }

    /// Check if interrupts for a specific source are enabled
    pub fn is_ie(&self, s: InterruptSource) -> bool {
        let (reg, mask) = Self::bitaddr(s, unsafe { &(*INT::ptr()).iec0 });
        unsafe { read_volatile(reg) & mask != 0 }
    }

    /// Read the interrupt flag of a specific interrupt source
    pub fn get_if(&self, s: InterruptSource) -> bool {
        let (reg, mask) = Self::bitaddr(s, unsafe { &(*INT::ptr()).ifs0 });
        unsafe { read_volatile(reg) & mask != 0 }
    }

    /// Clear the interrupt flag of a specific interrupt source
    /// To be called before terminating an ISR.
    pub fn clear_if(&self, s: InterruptSource) {
        let (reg, mask) = Self::bitaddr(s, unsafe { &(*INT::ptr()).ifs0clr });
        unsafe { write_volatile(reg, mask) };
    }

    /// Set the interrupt flag of a specific interrupt source
    pub fn set_if(&self, s: InterruptSource) {
        let (reg, mask) = Self::bitaddr(s, unsafe { &(*INT::ptr()).ifs0set });
        unsafe { write_volatile(reg, mask) };
    }

    fn byteaddr<REG: RegisterSpec>(iv: Interrupt, breg: &Reg<REG>) -> (*mut u32, usize) {
        let regndx = (iv as usize) / 4;
        let bytepos = ((iv as usize) % 4) * 8;
        let base = breg as *const _ as usize;
        let reg = (base + regndx * 0x10) as *mut u32;
        (reg, bytepos)
    }

    /// Set the interrupt priority level of a specific interrupt vector
    pub fn set_ipl(&self, iv: Interrupt, ipl: Ipl) {
        let (reg, bytepos) = Self::byteaddr(iv, unsafe { &(*INT::ptr()).ipc0 });
        let bitpos = bytepos + 2;
        let mask = 0x07 << bitpos;
        unsafe { write_volatile(reg, read_volatile(reg) & !mask | ((ipl.0 as u32) << bitpos)) };
    }

    /// Get the interrupt priority level of a specific interrupt vector
    pub fn ipl(&self, iv: Interrupt) -> Ipl {
        let (reg, bytepos) = Self::byteaddr(iv, unsafe { &(*INT::ptr()).ipc0 });
        let bitpos = bytepos + 2;
        unsafe { Ipl((read_volatile(reg) >> bitpos) as u8 & 0x07) }
    }

    /// Set the interrupt sub priority level of a specific interrupt vector
    pub fn set_isl(&self, iv: Interrupt, isl: Isl) {
        let (reg, bitpos) = Self::byteaddr(iv, unsafe { &(*INT::ptr()).ipc0 });
        let mask = 0x03 << bitpos;
        unsafe { write_volatile(reg, read_volatile(reg) & !mask | ((isl.0 as u32) << bitpos)) };
    }

    /// Get the interrupt sub priority level of a specific interrupt vector
    pub fn isl(&self, iv: Interrupt) -> Isl {
        let (reg, bitpos) = Self::byteaddr(iv, unsafe { &(*INT::ptr()).ipc0 });
        unsafe { Isl((read_volatile(reg) >> bitpos) as u8 & 0x03) }
    }
}
