//! # Core timer (MIPS cp0 registers) access and Delay implementation

use crate::pac::INT; // interrupt controller

use crate::time::Hertz;
use crate::hal::blocking::delay::{DelayMs, DelayUs};

/// Access to the MIPS core timer
///
/// Write access to the count register is intentionally not implemented to avoid
/// conflicts with the `Delay` functions. The `Delay` functions do not write to the
/// core timer registers.
///
/// The `CountDown` timer implementation, however, modifies the compare register
/// of the core timer so that an IRQ can be triggered on timer expiration.
pub struct Coretimer {
    ticks_per_us: u32,
}

impl Coretimer {
    pub const fn new(sysclock: Hertz) -> Self {
        let ticks_per_us = sysclock.0 / 1_000_000 / 2;
        Coretimer{ ticks_per_us }
    }

    /// Read Count register (CP0 register 9, select 0)
    pub fn read_count(&self) -> u32 {
        let mut count;
        // does not work with MIPS16
        unsafe { asm!("mfc0 $0, $$9" : "=r"(count) : : : "volatile")};
        count
    }

    /// Read Compare register (CP0 register 11, select 0)
    pub fn read_compare(&self) -> u32 {
        let mut compare;
        // does not work with MIPS16
        unsafe { asm!("mfc0 $0, $$11" : "=r"(compare) : : : "volatile")};
        compare
    }

    /// Write to Compare register (CPP0 register 11, select 0)
    pub fn write_compare(&self, compare: u32) {
        // does not work with MIPS16
        unsafe { asm!("mtc0   $0, $$11" : : "r"(compare) : : "volatile"); }
    }

    /// Enable interrupts
    pub fn enable_interrupts(&self, int: &INT){
        int.iec0set.write(|w| w.ctie().bit(true));
    }

    /// Disable interrupts and return whether interrupts were previously enabled.
    pub fn disable_interrupts(&self, int: &INT) -> bool {
        let was_enabled = int.iec0.read().ctie().bit();
        int.iec0clr.write(|w| w.ctie().bit(true));
        was_enabled
    }

    /// Set interrupt priority and sub priority. A priority level of 0 is the
    /// lowest priority level and disables the interrupts.
    pub fn set_interrupt_prio(&self, int: &INT, prio: u8, subprio: u8){
        int.ipc0.modify(|_, w| unsafe { w.ctip().bits(prio).ctis().bits(subprio) });
    }
}

impl DelayMs<u32> for Coretimer {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayMs<i32> for Coretimer {
    fn delay_ms(&mut self, ms: i32) {
        if ms >= 0 {
            self.delay_us((ms as u32) * 1000);
        }
    }
}

impl DelayMs<u16> for Coretimer {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(ms as u32);
    }
}

impl DelayMs<u8> for Coretimer {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u32);
    }
}

/// Pauses execution for `us` microseconds
/// Less efficient implementation that can handle long delays but is not so
/// well-suited for short delays in the order of a few µs.
impl DelayUs<u32> for Coretimer {
    fn delay_us(&mut self, us: u32) {
        let mut total_ticks = us as u64 * self.ticks_per_us as u64;
        while total_ticks != 0 {
            let current_ticks = if total_ticks <= 0xffff_ffffu64 {
                total_ticks as u32
            } else {
                0xffff_ffffu32
            };
            let start = self.read_count();
            total_ticks -= current_ticks as u64;
            while self.read_count().wrapping_sub(start) < current_ticks { }
        }
    }
}

/// Pauses execution for `us` microseconds
/// A more efficient implementation suitable for short delays
impl DelayUs<u16> for Coretimer {
    fn delay_us(&mut self, us: u16) {
        // read the count first for most accurate timing
        let start = self.read_count();
        let ticks = us as u32 * self.ticks_per_us;
        while self.read_count().wrapping_sub(start) < ticks { }
    }
}

impl DelayUs<u8> for Coretimer {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u16)
    }
}

