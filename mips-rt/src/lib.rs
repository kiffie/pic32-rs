//! Startup code and minimal runtime for MIPS microcontrollers
//! targets a MIPS microcontroller.

#![no_std]


#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn rust_reset() -> ! {
    extern "C" {

        // These symbols come from `link.x`
        static mut __sbss: u32;
        static mut __ebss: u32;

        static mut __sdata: u32;
        static mut __edata: u32;
        static __sidata: u32;

    }

    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        fn main() -> !;

        // This symbol will be provided by the user via `#[pre_init]`
        fn __pre_init();
    }

    __pre_init();

    // Initialize RAM
    r0::zero_bss(&mut __sbss, &mut __ebss);
    r0::init_data(&mut __sdata, &mut __edata, &__sidata);

    main();
}

#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}

pub mod interrupt;
pub mod fmt;

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

/// Physical address
#[derive(Clone, Copy, Debug, Default)]
pub struct PhysicalAddress {
    addr: usize,
}

impl PhysicalAddress {

    /// Create a PhysicalAddress by specifying its value directly
    pub const fn from_usize(addr: usize) -> Self {
        Self { addr }
    }

    /// get the value of the DmaAddress. Useful for programming of bus master
    /// peripherals, which typically access physical memory
    pub fn address(&self) -> usize {
        self.addr
    }
}
