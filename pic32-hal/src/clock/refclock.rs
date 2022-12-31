//! Reference clock generator

use core::marker::PhantomData;

#[cfg(any(
    feature = "pic32mx1xxfxxxb",
    feature = "pic32mx2xxfxxxb",
    feature = "pic32mx4xxfxxxh"
))]
use crate::pac::{osc::RegisterBlock, OSC};

#[cfg(feature = "pic32mx2x4fxxxb")]
use crate::pac::{cru::RegisterBlock, CRU};

use super::Error;

/// Selected input clock for reference oscillator
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Source {
    Sysclock = 0,
    Pbclock = 1,
    Posc = 2,
    Frc = 3,
    Lprc = 4,
    Sosc = 5,
    Usbpll = 6,
    Syspll = 7,
    Refclki = 8,
}

/// Reference clock generator singleton
pub struct Refclock {
    pub(super) _private: PhantomData<()>, // prevent direct construction outside of the clock module
}

#[cfg(any(
    feature = "pic32mx1xxfxxxb",
    feature = "pic32mx2xxfxxxb",
    feature = "pic32mx4xxfxxxh"
))]
macro_rules! regs {
    () => {
        unsafe { &*OSC::ptr() as &RegisterBlock }
    };
}

#[cfg(feature = "pic32mx2x4fxxxb")]
macro_rules! regs {
    () => {
        unsafe { &*CRU::ptr() as &RegisterBlock }
    };
}

impl Refclock {
    /// Enable reference clock generator
    pub fn enable(&self) {
        regs!().refoconset.write(|w| w.on().bit(true));
        // busy waiting until active bit is set
        while !regs!().refocon.read().active().bit() {}
    }

    /// Disable reference clock generator
    pub fn disable(&self) {
        regs!().refoconclr.write(|w| w.on().bit(true));
        // busy waiting until active bit is clear
        while regs!().refocon.read().active().bit() {}
    }

    /// Enable clock output
    pub fn output_enable(&self) {
        regs!().refoconset.write(|w| w.oe().bit(true));
    }

    /// Disable clock output
    pub fn output_disable(&self) {
        regs!().refoconclr.write(|w| w.oe().bit(true));
    }

    /// Select a clock source
    ///
    /// Returns an `InvalidState` error if called when active. Call this before
    /// `enable()` or after `disable()`
    pub fn select_source(&self, s: Source) -> Result<(), Error> {
        if regs!().refocon.read().active().bit() {
            return Err(Error::InvalidState);
        }
        regs!()
            .refocon
            .write(|w| unsafe { w.rosel().bits(s as u8) });
        Ok(())
    }

    /// Set reference clock divisor
    ///
    /// `div_q8` is the fractional clock divisor in Q16.8 representation. The
    /// minimum value is 2 (represented as 0x000200). The maximum value is
    /// 2^16 - 1/(2^8) (represented as 0xffffff). `div_q8` refers to the total
    /// clock ratio including an additional /2 division described in the data sheet.
    /// Returns an `InvalidArgument` error if the divisor is out of range and an
    /// `InvalidState` error if called during an ongoing divisor change
    /// operation.
    ///
    /// Remark: not clear (to me) from the documentation if changing the integer
    /// part of the divisor works when the reference clock module is active;
    /// needs to be tested.
    pub fn set_divisor(&self, div_q8: u32) -> Result<(), Error> {
        // this might look strange but there is also a fixed /2 clock division
        // in addition to the fractional divider.
        let m = div_q8 & 0x1ff; // fractional part: 9 LSB
        let n = div_q8 >> 9; // integer part (15 bit)
        if n == 0 || n > 32767 {
            return Err(Error::InvalidArgument);
        }
        if self.set_divisor_ongoing() {
            return Err(Error::InvalidState);
        }
        regs!()
            .refotrim
            .write(|w| unsafe { w.rotrim().bits(m as u16) });
        regs!()
            .refocon
            .modify(|_, w| unsafe { w.rodiv().bits(n as u16).divswen().bit(true) });
        Ok(())
    }

    /// Check if a clock divisor setting operation is ongoing.
    ///
    /// Returns false if the ACTIVE bit is not set regardless of whether a
    /// divisor setting has previously been initiated.
    pub fn set_divisor_ongoing(&self) -> bool {
        regs!().refocon.read().active().bit() && regs!().refocon.read().divswen().bit()
    }
}
