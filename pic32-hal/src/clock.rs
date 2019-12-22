//
// Clock helper and control functions
//


#[cfg(feature = "pic32mx274fxxxb")]
use crate::pac::CRU;
use core::fmt;



use sysconfig::SYS_CLOCK;
use crate::pac;

/// Hertz
#[derive(Clone, Copy, Debug)]
pub struct Hertz(pub u32);

impl fmt::Display for Hertz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Hz", self.0)
    }
}

/// Method to create a Hertz from a 32
pub trait U32Ext {
    /// Convert into a Hertz object
    fn hz(self) -> Hertz;
}

impl U32Ext for u32 {
    fn hz(self) -> Hertz {
        Hertz(self)
    }
}


pub struct Osc {
    cru: CRU,
    sysclock: Hertz,
}


impl Osc {

    /// Create a new Osc from a possibly constant sysclock value. The sysclock
    /// value should be set to the core clock resulting from the configuration
    /// words and, if a crystal oscillator is used, the crystal frequency.
    #[cfg(feature = "pic32mx274fxxxb")]
    pub const fn new(cru: CRU, sysclock: Hertz) -> Osc {
        Osc { cru, sysclock }
    }

    /// Determine the peripheral clock frequency based on the sysclock value
    /// and the peripheral clock divider setting
    #[cfg(feature = "pic32mx274fxxxb")]
    pub fn pb_clock(&self) -> Hertz {
        let div = self.cru.pb1div.read().pbdiv().bits();
        let freq = self.sysclock.0 / (div as u32 + 1);
        freq.hz()
    }
}



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


