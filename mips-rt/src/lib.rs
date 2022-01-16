//! Startup code and minimal runtime for MIPS microcontrollers
//! targets a MIPS microcontroller.

#![no_std]

pub use mips_rt_macros::{entry, interrupt, pre_init};

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}

/// Returns a pointer to the start of the heap
///
/// The returned pointer is guaranteed to be 4-byte aligned.
#[inline]
pub fn heap_start() -> *mut u32 {
    extern "C" {
        static mut __sheap: u32;
    }

    unsafe { &mut __sheap }
}
