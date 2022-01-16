//! Core timer (CPO count and compare registers)
//!
//! Functions to access the CP0 registers 9 and 11 using the (privileged)
//! MIPS instructions `mfc0` and `mtc0`.
//!

/// Read count register (CP0 register 9, select 0)
#[inline]
pub fn read_count() -> u32 {
    extern "C" {
        fn mips_read_cp0_count() -> u32;
    }
    unsafe { mips_read_cp0_count() }
}

/// Write count register (CP0 register 9, select 0)
#[inline]
pub unsafe fn write_count(count: u32) {
    extern "C" {
        fn mips_write_cp0_count(count: u32);
    }
    mips_write_cp0_count(count);
}

/// Read compare register (CP0 register 11, select 0)
#[inline]
pub fn read_compare() -> u32 {
    extern "C" {
        fn mips_read_cp0_compare() -> u32;
    }
    unsafe { mips_read_cp0_compare() }
}

/// Write compare register (CP0 register 11, select 0)
#[inline]
pub unsafe fn write_compare(compare: u32) {
    extern "C" {
        fn mips_write_cp0_compare(compare: u32);
    }
    mips_write_cp0_compare(compare);
}
