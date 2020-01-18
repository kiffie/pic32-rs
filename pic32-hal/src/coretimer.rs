//! # Core timer (MIPS cp0 registers) access and Delay implementation
///
/// Write access to the count register is intentionally not implemented to avoid
/// conflicts with the `Delay` functions. The `Delay` functions do not write to the
/// core timer registers.
///
/// The `CountDown` timer implementation, however, modifies the compare register
/// of the core timer so that an IRQ can be triggered on timer expiration,


use crate::pac::INT; // interrupt controller

use crate::time::Hertz;
use crate::hal::blocking::delay::{DelayMs, DelayUs};

pub struct Coretimer {
    ticks_per_us: u32,
    max_delay_us: u32,
}

impl Coretimer {
    pub const fn new(sysclock: Hertz) -> Self {
        let ticks_per_us = sysclock.0 / 1_000_000 / 2;
        let max_delay_us = (1_000_000 * 2 * 0x1_0000_0000 / (sysclock.0 as u64)) as u32;
        Coretimer{ ticks_per_us, max_delay_us }
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
/// Pause time is limited to the duration
impl DelayUs<u32> for Coretimer {
    fn delay_us(&mut self, us: u32) {
        // read the count first for most accurate timing
        let mut count = self.read_count();
        if us > self.max_delay_us {
            panic!("delay too long");
        }
        let ticks = us * self.ticks_per_us;
        let mut n_wraps = if u32::max_value() - count < ticks  { 1 } else { 0 };
        let when = count.wrapping_add(ticks);
        let mut last = count;
        while n_wraps > 0 || count < when {
            if last > count { // count wrapped
                n_wraps -= 1;
            }
            last = count;
            count = self.read_count();
        }
    }
}

// impl DelayUs<i32> for Coretimer {
//     fn delay_us(&mut self, us: i32) {
//         if us >= 0 {
//             self.delay_us(us as u32);
//         }
//     }
// }

impl DelayUs<u16> for Coretimer {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(us as u32)
    }
}

impl DelayUs<u8> for Coretimer {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u32)
    }
}

