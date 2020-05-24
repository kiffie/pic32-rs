//! Reference clock generator

use crate::pac::OSC;

/// Selected input clock for reference oscillator
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Source {
    Sysclock = 0,
    Pbclock = 1,
    Posc =  2,
    Frc =  3,
    Lprc =  4,
    Sosc = 5,
    Usbpll = 6,
    Syspll = 7,
    Refclki = 8,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct DivisorOutOfRangeError;

pub struct Refclock<'a> {
    pub(super) osc: &'a OSC,
}

impl Refclock<'_> {
    /// Enable reference clock generator
    pub fn enable(&self) {
        self.osc.refoconset.write(|w| w.on().bit(true));
    }

    /// Disable reference clock generator
    pub fn disable(&self) {
        self.osc.refoconclr.write(|w| w.on().bit(true));
    }

    /// Enable clock output
    pub fn output_enable(&self) {
        self.osc.refoconset.write(|w| w.oe().bit(true));
    }

    /// Disable clock output
    pub fn output_disable(&self) {
        self.osc.refoconclr.write(|w| w.oe().bit(true));
    }

    pub fn select_source(&self, s: Source) {
        self.wait_active();
        self.osc.refocon.write(|w| unsafe { w.rosel().bits(s as u8) });
    }

    /// Set reference clock divisor
    ///
    /// div_q3 is the clock divisor in Q16.8 representation. The minimum value
    /// is 2. The maximum value is 2^16 - 1/(2^8).
    pub fn set_divisor(&self, div_q8: u32) -> Result<(), DivisorOutOfRangeError> {
        // this might look strange but there is also a fixed /2 clock division
        // in addition to the fractional dividor
        let m = div_q8 & 0x1ff; // fractional part: 9 LSB
        let n = div_q8 >> 9; // integer part (15 bit)
        if n == 0 || n > 32767 {
            return Err(DivisorOutOfRangeError);
        }
        self.wait_active();
        self.osc.refotrim.write(|w| unsafe { w.rotrim().bits(m as u16) });
        self.osc.refocon.modify(|_, w| unsafe {
            w.rodiv().bits(n as u16).divswen().bit(true)
        });
        Ok(())
    }

    /// busy waiting until active bit is clear
    fn wait_active(&self) {
        while self.osc.refocon.read().active().bit() { }
    }
}
