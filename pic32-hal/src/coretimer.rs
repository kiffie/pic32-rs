//! Access to the MIPS core timer and implementation of the `DelayMs/DelayUs`
//! trait
//!
//! Uses the MIPS CP0 registers "count" and "compare".

use crate::hal::blocking::delay::{DelayMs, DelayUs};
use crate::pac::INT; // interrupt controller
use crate::time::Hertz;

use mips_rt::interrupt;
use mips_rt::interrupt::Mutex;

use core::cell::Cell;

/// Read Count register (CP0 register 9, select 0)
fn read_count() -> u32 {
    let mut count;
    // does not work with MIPS16
    unsafe { asm!("mfc0 $0, $$9" : "=r"(count) : : : "volatile") };
    count
}

/// Delay implementation based on read-only access to the core timer "count"
/// register
pub struct Delay {
    ticks_per_us: u32,
}

impl Delay {
    pub const fn new(sysclock: Hertz) -> Self {
        let ticks_per_us = sysclock.0 / 1_000_000 / 2;
        Delay { ticks_per_us }
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayMs<i32> for Delay {
    fn delay_ms(&mut self, ms: i32) {
        if ms >= 0 {
            self.delay_us((ms as u32) * 1000);
        }
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(ms as u32);
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u32);
    }
}

/// Pauses execution for `us` microseconds
/// Less efficient implementation that can handle long delays but is not so
/// well-suited for short delays in the order of a few µs.
impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        let mut total_ticks = us as u64 * self.ticks_per_us as u64;
        while total_ticks != 0 {
            let current_ticks = if total_ticks <= 0xffff_ffffu64 {
                total_ticks as u32
            } else {
                0xffff_ffffu32
            };
            let start = read_count();
            total_ticks -= current_ticks as u64;
            while read_count().wrapping_sub(start) < current_ticks {}
        }
    }
}

/// Pauses execution for `us` microseconds
/// A more efficient implementation suitable for short delays
impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        // read the count first for most accurate timing
        let start = read_count();
        let ticks = us as u32 * self.ticks_per_us;
        while read_count().wrapping_sub(start) < ticks {}
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u16)
    }
}

/// Direct access to the MIPS core timer (CP0 timer) and related interrupt control
///
/// Can write to the "compare" register of MIPS core timer and can thus be
/// instantiated only once. Write to the "count" register is not implemented to
/// avoid conflicts with the `Delay` struct.
pub struct Timer {}

static TIMER: Mutex<Cell<Option<Timer>>> = Mutex::new(Cell::new(Some(Timer {})));

impl Timer {
    /// Get the `Timer` singleton. Panics if the singleton is not available.
    pub fn take() -> Self {
        let timeropt = interrupt::free(|cs| {
            let cell = TIMER.borrow(cs);
            cell.take()
        });
        timeropt.unwrap()
    }

    /// Return the `Timer` singleton.
    pub fn free(self) {
        interrupt::free(|cs| {
            let cell = TIMER.borrow(cs);
            cell.replace(Some(self));
        });
    }

    /// Read Count register (CP0 register 9, select 0)
    pub fn read_count(&self) -> u32 {
        read_count()
    }

    /// Read Compare register (CP0 register 11, select 0)
    pub fn read_compare(&self) -> u32 {
        let mut compare;
        // does not work with MIPS16
        unsafe { asm!("mfc0 $0, $$11" : "=r"(compare) : : : "volatile") };
        compare
    }

    /// Write to Compare register (CPP0 register 11, select 0)
    pub fn write_compare(&self, compare: u32) {
        // does not work with MIPS16
        unsafe { asm!("mtc0   $0, $$11" : : "r"(compare) : : "volatile") };
    }

    /// Enable interrupts
    pub fn enable_interrupts(&self, int: &INT) {
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
    pub fn set_interrupt_prio(&self, int: &INT, prio: u8, subprio: u8) {
        int.ipc0.modify(|_, w| unsafe { w.ctip().bits(prio).ctis().bits(subprio) });
    }
}
