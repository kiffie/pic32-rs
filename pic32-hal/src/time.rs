//! Types for time units and frequency
//! TODO: replace with fugit
// Based on https://github.com/stm32-rs/stm32f4xx-hal/blob/master/src/time.rs

#![allow(clippy::from_over_into)]

/// Bits per second
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Bps(pub u32);

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Hertz(pub u32);

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct KiloHertz(pub u32);

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct MegaHertz(pub u32);

/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;

    /// Wrap in `Hertz`
    fn hz(self) -> Hertz;

    /// Wrap in `KiloHertz`
    fn khz(self) -> KiloHertz;

    /// Wrap in `MegaHertz`
    fn mhz(self) -> MegaHertz;

    /// Wrap in `MilliSeconds`
    fn ms(self) -> MilliSeconds;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        Bps(self)
    }

    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn khz(self) -> KiloHertz {
        KiloHertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }

    fn ms(self) -> MilliSeconds {
        MilliSeconds(self)
    }
}

impl Into<Hertz> for KiloHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000_000)
    }
}

impl Into<KiloHertz> for MegaHertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 * 1_000)
    }
}

/// Time unit
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct MilliSeconds(pub u32);
