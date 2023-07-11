//! Timer

pub mod timer_a;
pub mod timer_b;

/// Clocking Modes
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clocking {
    /// Internal clock (Pbclock)
    Pbclock,

    /// Internal clock, gated
    PbclockGated,

    /// External clock
    External,
}
