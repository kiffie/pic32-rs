///Clock helper and control functions

use sysconfig::SYS_CLOCK;
use crate::pac;

/// gets the system clock
pub const fn sys_clock() -> u32 {
    SYS_CLOCK as u32
}


/// determines the peripheral clock frequency in Hz based on the SYS_CLOCK
/// constant and the peripheral clock divider setting
#[cfg(feature = "pic32mx1xxfxxxb")]
#[cfg(feature = "pic32mx4xxfxxxh")]
pub fn pb_clock() -> u32 {
    let p = unsafe { pac::Peripherals::steal()};
    let div = p.OSC.osccon.read().pbdiv().bits();
    SYS_CLOCK as u32 >> div
}

/// determines the peripheral clock frequency in Hz based on the SYS_CLOCK
/// constant and the peripheral clock divider setting
#[cfg(feature = "pic32mx274fxxxb")]
pub fn pb_clock() -> u32 {
    let p = unsafe { pac::Peripherals::steal()};
    let div = p.CRU.pb1div.read().pbdiv().bits();
    SYS_CLOCK as u32 / (div as u32 + 1)
}


