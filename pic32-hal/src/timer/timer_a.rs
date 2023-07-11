//! Timer type A

use super::Clocking;
use crate::pac::TMR1;
use core::marker::PhantomData;

/// Clock pre scaler configuration for timer type A
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ClockPrescale {
    /// 1:1 prescale value
    Prescale1 = 0,

    /// 1:8 prescale value
    Prescale8 = 1,

    /// 1:64 prescale value
    Prescale64 = 2,

    /// 1:256 prescale value
    Prescale256 = 3,
}

/// Marker for Synchronous operation
pub struct TimerSynchronous;

/// Marker for Asynchronous operation
pub struct TimerAsynchronous;

/// HAL struct for Timer1 (timer type A)
pub struct Timer<MODE> {
    timer: TMR1,
    _marker: PhantomData<MODE>,
}

impl<MODE> Timer<MODE> {
    /// Initalize the timer for synchronous operation
    ///
    /// The timer peripheral is running synchronously with the PBCLOCK and
    /// cannot count external pulses when the PBCLOCK is gated.
    pub fn timer1_synchronous(
        timer: TMR1,
        clocking: Clocking,
        prescale: ClockPrescale,
        period: u16,
        stop_in_idle_mode: bool,
    ) -> Timer<TimerSynchronous> {
        timer.cont.write(|w| unsafe { w
            .sidl().bit(stop_in_idle_mode)
            .tgate().bit(clocking == Clocking::PbclockGated)
            .tckps().bits(prescale as u8)
            .tsync().set_bit()
            .tcs().bit(clocking == Clocking::External)
        });
        timer.tmr.write(|w| unsafe { w.tmr().bits(0) });
        timer.pr.write(|w| unsafe { w.pr().bits(period as u32) });
        timer.contset.write(|w| w.on().set_bit());

        Timer { timer, _marker: PhantomData }
    }

    /// Initialize the timer for asynchronous operation
    ///
    /// The timer will operate asynchronously with respect and independently
    /// from to the PBCLOCK When operating asynchronously, the timer is always
    /// external clocked, e.g. by a 32 kHz clock source or the like.
    pub fn timer1_asynchronous(
        timer: TMR1,
        prescale: ClockPrescale,
        period: u16,
        stop_in_idle_mode: bool,
    ) -> Timer<TimerAsynchronous> {
        timer.tmr.write(|w| unsafe { w.tmr().bits(0) });
        timer.pr.write(|w| unsafe { w.pr().bits(period as u32) });
        timer.cont.write(|w| unsafe { w
            .sidl().bit(stop_in_idle_mode)
            .twdis().clear_bit()
            .tgate().clear_bit()
            .tckps().bits(prescale as u8)
            .tsync().clear_bit()
            .tcs().set_bit()
        });
        timer.contset.write(|w| w.on().set_bit());

        Timer { timer, _marker: PhantomData }
    }

    /// Turn the timer off
    pub fn free(self) -> TMR1 {
        self.timer.contclr.write(|w| w.on().set_bit());
        self.timer
    }

    /// Read the current timer count value
    pub fn tmr(&self) -> u16 {
        self.timer.tmr.read().tmr().bits() as u16
    }

    /// Read the period (PR register)
    pub fn period(&self) -> u16 {
        self.timer.pr.read().pr().bits() as u16
    }

    /// Write to the period (PR register)
    pub fn set_period(&mut self, period: u16) {
        self.timer.pr.write(|w| unsafe { w.pr().bits(period as u32) });
    }
}

impl Timer<TimerSynchronous> {
    /// Set the current timer count value
    pub fn set_tmr(&mut self, tmr: u16) {
        self.timer.tmr.write(|w| unsafe { w.tmr().bits(tmr as u32) });
    }
}

impl Timer<TimerAsynchronous> {
    /// Set the current timer count value
    pub fn set_tmr(&mut self, tmr: u16) {
        while self.timer.cont.read().twip().bit_is_set() {}
        self.timer.tmr.write(|w| unsafe { w.tmr().bits(tmr as u32) });
    }
}
