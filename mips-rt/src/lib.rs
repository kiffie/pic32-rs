//! Startup code and minimal runtime for MIPS microcontrollers
//! targets a MIPS microcontroller.

#![no_std]

/// Attribute to declare a `pre_init` hook function
pub use mips_rt_macros::pre_init;

/// Attribute to declare the entry point of the program
///
/// The specified function will be called by the reset handler after RAM has
/// been initialized.
///
/// The type of the specified function must be `[unsafe] fn() -> !` (never
/// ending function)
///
/// The entry point will be called by the reset handler. The program can't
/// reference to the entry point, much less invoke it.
///
/// `static mut` variables declared within the entry function are safe to access
/// and can be used to preserve state across invocations of the handler. The
/// compiler can't prove this is safe so the attribute will help by making a
/// transformation to the source code: for this reason a variable like `static
/// mut FOO: u32` will be accessible from within the entry function as `let FOO:
/// &mut u32;`.
///
/// ## Example
///
/// ```ignore
/// # #![no_main]
/// # use mips_rt_macros::entry;
/// #[entry]
/// fn main() -> ! {
///     loop {
///         /* .. */
///     }
/// }
/// ```
pub use mips_rt_macros::entry;

/// Attribute to declare an Interrupt Service Routine (ISR)
///
/// The name of the ISRs must correspond to the interrupts names defined in the
/// Peripheral Access Crate of the respective device.
///
/// `static mut` variables declared within an ISR are safe to access and can be
/// used to preserve state across invocations of the handler. The compiler can't
/// prove this is safe so the attribute will help by making a transformation to
/// the source code: for this reason a variable like `static mut FOO: u32` will
/// be accessible from within the ISR as `let FOO: &mut u32;`.
///
/// ## Example
///
/// ```ignore
/// #[interrupt]
/// fn CORE_TIMER() {
///     static mut COUNTER: u32 = 0;
///
///     *COUNTER += 1;  // access of a static mut variable as described above
///
///     // clear interrupt flag
///     unsafe {
///         (*INT::ptr()).ifs0clr.write(|w| w.ctif().bit(true));
///     }
/// }
/// ```
///
pub use mips_rt_macros::interrupt;

/// Attribute to declare an exception handler and setting up respective symbols
/// for linking
///
/// The exception handlers must have the name `nmi` or `general_exception` to
/// create an NMI or an General Exception handler, respectively. The handlers
/// take two u32 arguments for the CP0 Cause and CP0 Status registers.
///
/// ## Example (General Exception)
///
/// ```ignore
/// #[exception]
/// fn general_exception(cp0_cause: u32, cp0_status: u32) {
///
///     // ...
/// }
/// ```
///
/// ## Example (NMI)
///
/// ```ignore
/// #[exception]
/// fn nmi(cp0_cause: u32, cp0_status: u32) {
///
///     // ...
/// }
/// ```
pub use mips_rt_macros::exception;

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
