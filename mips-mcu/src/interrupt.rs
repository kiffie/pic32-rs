//! Interrupts
// This is based on Work (c) by Jorge Aparicio, see
// https://github.com/rust-embedded/cortex-m

// use core::sync::atomic::{self, Ordering};

pub use bare_metal::{CriticalSection, Mutex};

type IrqSave = u32;

/// Enable multi-vectored interrupts
#[inline]
pub unsafe fn enable_mv_irq() {
    extern "C" {
        fn mips_enable_mv_irq();
    }
    mips_enable_mv_irq();
}

/// Disables all interrupts and return previous status
#[inline]
pub unsafe fn disable() -> IrqSave {
    extern "C" {
        fn mips_di() -> u32;
    }
    mips_di()
}

/// Enables all the interrupts and return previous status
///
/// # Safety
///
/// - Do not call this function inside an `interrupt::free` critical section
#[inline]
pub unsafe fn enable() -> IrqSave {
    extern "C" {
        fn mips_ei() -> u32;
    }
    mips_ei()
}

pub unsafe fn restore(previous_status: IrqSave) {
    extern "C" {
        fn mips_restore_irq(previous_status: u32);
    }
    mips_restore_irq(previous_status)
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    let irq_save = unsafe { disable() };

    let r = f(unsafe { &CriticalSection::new() });

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    unsafe { restore(irq_save) };

    r
}
