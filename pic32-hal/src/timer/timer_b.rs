//! Timer type B

use super::Clocking;
use crate::pac::{TMR2, TMR3, TMR4, TMR5};

/// Clock pre scaler configuration for timer type B
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ClockPrescale {
    /// 1:1 prescale value
    Prescale1 = 0,

    /// 1:2 prescale value
    Prescale2 = 1,

    /// 1:4 prescale value
    Prescale4 = 2,

    /// 1:8 prescale value
    Prescale8 = 3,

    /// 1:16 prescale value
    Prescale16 = 4,

    /// 1:32 prescale value
    Prescale32 = 5,

    /// 1:64 prescale value
    Prescale64 = 6,

    /// 1:256 prescale value
    Prescale256 = 7,
}

/// HAL struct of timer type B
pub struct Timer<TIMER> {
    timer: TIMER,
}

macro_rules! timerb_impl {
    ($constructor: ident, $timer: ty) => {
        impl Timer<$timer> {
            /// Initialize the timer
            pub fn $constructor(
                timer: $timer,
                clocking: Clocking,
                prescale: ClockPrescale,
                period: u16,
                stop_in_idle_mode: bool,
            ) -> Self {
                timer.cont.write(|w| unsafe { w
                    .sidl().bit(stop_in_idle_mode)
                    .tgate().bit(clocking == Clocking::PbclockGated)
                    .tckps().bits(prescale as u8)
                    .tcs().bit(clocking == Clocking::External)
                });
                timer.tmr.write(|w| unsafe { w.tmr().bits(0) });
                timer.pr.write(|w| unsafe { w.pr().bits(period as u32) });
                timer.contset.write(|w| w.on().set_bit());

                Self { timer }
            }

            /// Turn the timer off
            pub fn free(self) -> $timer {
                self.timer.contclr.write(|w| w.on().set_bit());
                self.timer
            }

            /// Read the current timer count value (TMR register)
            pub fn tmr(&self) -> u16 {
                self.timer.tmr.read().tmr().bits() as u16
            }

            /// Set the current timer count value (TMR register)
            pub fn set_tmr(&mut self, tmr: u16) {
                self.timer.tmr.write(|w| unsafe { w.tmr().bits(tmr as u32) });
            }

            /// Read the maximum value (PR register)
            pub fn pr(&self) -> u16 {
                self.timer.pr.read().pr().bits() as u16
            }

            /// Set the maximum value (PR register)
            pub fn set_pr(&mut self, period: u16) {
                self.timer.pr.write(|w| unsafe { w.pr().bits(period as u32) });
            }
        }
    };
}

timerb_impl!(timer2, TMR2);
timerb_impl!(timer3, TMR3);
timerb_impl!(timer4, TMR4);
timerb_impl!(timer5, TMR5);

/// HAL struct for a pair of timers of type B (32-bit mode)
pub struct Timer32<TIMERL, TIMERH> {
    timer_low: TIMERL,
    timer_high: TIMERH,
}

macro_rules! timer32_impl {
    ($constructor: ident, $timer_low: ty, $timer_high: ty) => {
        impl Timer32<$timer_low, $timer_high> {
            /// Initialize the timer
            pub fn $constructor(
                timer_low: $timer_low,
                timer_high: $timer_high,
                clocking: Clocking,
                prescale: ClockPrescale,
                period: u32,
                stop_in_idle_mode: bool,
            ) -> Self {
                timer_low.cont.write(|w| unsafe { w
                    .sidl().bit(stop_in_idle_mode)
                    .tgate().bit(clocking == Clocking::PbclockGated)
                    .tckps().bits(prescale as u8)
                    .t32().set_bit()
                    .tcs().bit(clocking == Clocking::External)
                });
                timer_high.cont.write(|w| w.sidl().bit(stop_in_idle_mode));

                timer_low.tmr.write(|w| unsafe { w.tmr().bits(0) });
                timer_low.pr.write(|w| unsafe { w.pr().bits(period) });
                timer_low.contset.write(|w| w.on().set_bit());

                Self { timer_low, timer_high }
            }

            /// Turn the timer off
            pub fn free(self) -> ($timer_low, $timer_high) {
                self.timer_low.contclr.write(|w| w.on().set_bit());
                self.timer_high.contclr.write(|w| w.on().set_bit());
                (self.timer_low, self.timer_high)
            }

            /// Read the current timer count value (TMR register)
            pub fn tmr(&self) -> u32 {
                self.timer_low.tmr.read().tmr().bits()
            }

            /// Set the current timer count value (TMR register)
            pub fn set_tmr(&mut self, tmr: u32) {
                self.timer_low.tmr.write(|w| unsafe { w.tmr().bits(tmr) });
            }

            /// Read the maximum value (PR register)
            pub fn pr(&self) -> u32 {
                self.timer_low.pr.read().pr().bits()
            }

            /// Set the maximum value (PR register)
            pub fn set_pr(&mut self, period: u32) {
                self.timer_low.pr.write(|w| unsafe { w.pr().bits(period) });
            }
        }
    };
}

timer32_impl!(timer2_3, TMR2, TMR3);
timer32_impl!(timer4_5, TMR4, TMR5);
