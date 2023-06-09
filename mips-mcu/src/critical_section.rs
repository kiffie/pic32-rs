//! Simple critical section implementation based on globally disabling
//! the interrupts

use critical_section::{set_impl, Impl, RawRestoreState};

use crate::interrupt;

struct SingleCoreCriticalSection;
set_impl!(SingleCoreCriticalSection);

unsafe impl Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        interrupt::disable()
    }

    unsafe fn release(previous_status: RawRestoreState) {
        interrupt::restore(previous_status)
    }
}
