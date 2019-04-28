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
