//! Clock helper and control functions
// Work in progress, can calculate pb_clock only, no real control functions
// implemented
//

use crate::time::Hertz;
use crate::time::U32Ext;

#[cfg(feature = "pic32mx274fxxxb")]
use crate::pac::CRU;

#[cfg(any(
    feature = "pic32mx1xxfxxxb",
    feature = "pic32mx2xxfxxxb",
    feature = "pic32mx4xxfxxxh"
))]
use crate::pac::OSC;

#[cfg(feature = "pic32mx274fxxxb")]
pub struct Osc {
    cru: CRU,
    sysclock: Hertz,
}

#[cfg(any(
    feature = "pic32mx1xxfxxxb",
    feature = "pic32mx2xxfxxxb",
    feature = "pic32mx4xxfxxxh"
))]
pub struct Osc {
    osc: OSC,
    sysclock: Hertz,
}

#[cfg(feature = "pic32mx274fxxxb")]
impl Osc {

    /// Create a new Osc from a possibly constant sysclock value. The sysclock
    /// value should be set to the core clock resulting from the configuration
    /// words and, if a crystal oscillator is used, the crystal frequency.
    pub const fn new(cru: CRU, sysclock: Hertz) -> Osc {
        Osc { cru, sysclock }
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
    feature = "pic32mx4xxfxxxh"
))]
impl Osc {

    /// Create a new Osc from a possibly constant sysclock value. The sysclock
    /// value should be set to the core clock resulting from the configuration
    /// words and, if a crystal oscillator is used, the crystal frequency.
    pub const fn new(osc: OSC, sysclock: Hertz) -> Osc {
        Osc { osc, sysclock }
    }

    /// Determine the peripheral clock frequency based on the sysclock value
    /// and the peripheral clock divider setting
    pub fn pb_clock(&self) -> Hertz {
        let div = self.osc.osccon.read().pbdiv().bits();
        let freq = self.sysclock.0 >> div;
        freq.hz()
    }
}
