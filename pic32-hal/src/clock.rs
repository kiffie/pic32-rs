//! Clock helper and control functions
// Work in progress, can calculate pb_clock only, no real control functions
// implemented
//

use crate::time::Hertz;
use crate::time::U32Ext;
use core::marker::PhantomData;

#[cfg(any(
    feature = "pic32mx1xxfxxxb",
    feature = "pic32mx2xxfxxxb",
    feature = "pic32mx2x4fxxxb",
    feature = "pic32mx4xxfxxxh",
    feature = "pic32mx37x",
    feature = "pic32mx47x",
))]
pub mod refclock;

#[cfg(feature = "pic32mx2x4fxxxb")]
use crate::pac::CRU;

#[cfg(any(
    feature = "pic32mx1xxfxxxb",
    feature = "pic32mx2xxfxxxb",
    feature = "pic32mx4xxfxxxh",
    feature = "pic32mx37x",
    feature = "pic32mx47x",
))]
use crate::pac::OSC;

#[cfg(feature = "pic32mx2x4fxxxb")]
pub struct Osc {
    cru: CRU,
    sysclock: Hertz,
}

#[cfg(any(
    feature = "pic32mx1xxfxxxb",
    feature = "pic32mx2xxfxxxb",
    feature = "pic32mx4xxfxxxh",
    feature = "pic32mx37x",
    feature = "pic32mx47x",
))]
pub struct Osc {
    osc: OSC,
    sysclock: Hertz,
}

pub struct Simple;

pub struct WithRefclock;

/// Clock module errors
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Error {
    /// Operation cannot performed in the current state of the clock module
    InvalidState,

    /// Value is out of range supported by the hardware
    InvalidArgument,
}

#[cfg(feature = "pic32mx2x4fxxxb")]
impl Osc {
    /// Create a new `Osc` from a possibly constant sysclock value. The sysclock
    /// value should be set to the core clock resulting from the configuration
    /// words and, if a crystal oscillator is used, the crystal frequency.
    pub const fn new(cru: CRU, sysclock: Hertz) -> Osc {
        Osc { cru, sysclock }
    }

    /// Create a new `Osc` and `Refclock` from a possibly constant sysclock
    /// value. The sysclock value should be set to the core clock resulting from
    /// the configuration words and, if a crystal oscillator is used, the
    /// crystal frequency.
    pub const fn new_with_refclock(cru: CRU, sysclock: Hertz) -> (Osc, refclock::Refclock) {
        (
            Osc { cru, sysclock },
            refclock::Refclock {
                _private: PhantomData,
            },
        )
    }

    /// Get the sysclock
    pub fn sysclock(&self) -> Hertz {
        self.sysclock
    }

    /// Determine the peripheral clock frequency based on the sysclock value
    /// and the peripheral clock divider setting
    pub fn pb_clock(&self) -> Hertz {
        let div = self.cru.pb1div.read().pbdiv().bits();
        let freq = self.sysclock.0 / (div as u32 + 1);
        freq.hz()
    }
}

#[cfg(any(
    feature = "pic32mx1xxfxxxb",
    feature = "pic32mx2xxfxxxb",
    feature = "pic32mx4xxfxxxh",
    feature = "pic32mx37x",
    feature = "pic32mx47x",
))]
impl Osc {
    /// Create a new `Osc` from a possibly constant sysclock value. The sysclock
    /// value should be set to the core clock resulting from the configuration
    /// words and, if a crystal oscillator is used, the crystal frequency.
    pub const fn new(osc: OSC, sysclock: Hertz) -> Osc {
        Osc { osc, sysclock }
    }

    /// Create a new `Osc` and `Refclock` from a possibly constant sysclock
    /// value. The sysclock value should be set to the core clock resulting from
    /// the configuration words and, if a crystal oscillator is used, the
    /// crystal frequency.
    pub const fn new_with_refclock(osc: OSC, sysclock: Hertz) -> (Osc, refclock::Refclock) {
        (
            Osc { osc, sysclock },
            refclock::Refclock {
                _private: PhantomData,
            },
        )
    }

    /// Get the sysclock
    pub fn sysclock(&self) -> Hertz {
        self.sysclock
    }

    /// Determine the peripheral clock frequency based on the sysclock value
    /// and the peripheral clock divider setting
    pub fn pb_clock(&self) -> Hertz {
        let div = self.osc.osccon.read().pbdiv().bits();
        let freq = self.sysclock.0 >> div;
        freq.hz()
    }
}
