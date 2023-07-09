//! Interrupts
// This is based on Work (c) by Jorge Aparicio, see
// https://github.com/rust-embedded/cortex-m

type IrqSave = u32;

/// Enable multi-vectored interrupts
#[inline]
pub fn enable_mv_irq() {
    extern "C" {
        fn mips_enable_mv_irq();
    }
    unsafe {
        mips_enable_mv_irq();
    }
}

/// Disables all interrupts and return previous status
#[inline]
pub fn disable() -> IrqSave {
    extern "C" {
        fn mips_di() -> u32;
    }
    unsafe { mips_di() }
}

/// Enables all the interrupts and return previous status
///
/// # Safety
///
/// Do not call this function inside a critical section
#[inline]
pub unsafe fn enable() -> IrqSave {
    extern "C" {
        fn mips_ei() -> u32;
    }
    mips_ei()
}

/// Restore previously saved IRQ enablement state
///
/// # Safety
///
/// Do not call this function inside a critical section
pub unsafe fn restore(previous_status: IrqSave) {
    extern "C" {
        fn mips_restore_irq(previous_status: u32);
    }
    mips_restore_irq(previous_status)
}

/// Wait for interrupts
///
/// Use the MIPS `wait` instruction to wait for interrupts and to put the
/// processor in a power saving mode.
pub fn wait() {
    extern "C" {
        fn mips_wait();
    }
    unsafe {
        mips_wait();
    }
}
