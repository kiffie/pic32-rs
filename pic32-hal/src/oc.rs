//! Output Compare
//!

use crate::pac::{OCMP1, OCMP2, OCMP3, OCMP4, OCMP5};
use crate::pac::{TMR2, TMR3};
use core::convert::Infallible;
use embedded_hal::pwm::{ErrorType, SetDutyCycle};
use embedded_hal_0_2::PwmPin;

use core::marker::PhantomData;

/// Marker for configurations where the even numbered 16-bit timer (e.g. Timer 2) is used as a time base
pub struct Timebase16even;

/// Marker for configurations where the odd numbered 16-bit timer (e.g. Timer 3) is used as a time base
pub struct Timebase16odd;

/// Marker for configurations where two 16-bits timer form a 32-bit time base
pub struct Timebase32;

/// Output compare configuration (excluding PWM modes)
pub enum OcConfig {
    /// No operation
    Off,
    /// Rising slope when the timer is equal to the specified value
    RisingSlope(u32),

    /// Falling slope when the timer is equal to the specified value
    FallingSlope(u32),

    /// Toggle when the timer is equal to the specified value
    Toggle(u32),

    /// Single pulse with values defining the rising edge and falling edge, respectively
    SinglePulse(u32, u32),

    /// Continuous pulses with values defining the rising edges and falling edges, respectively
    ContinuousPulses(u32, u32),
}

impl OcConfig {
    const fn ocm_bits(&self) -> u8 {
        match self {
            Self::Off => 0b000,
            Self::RisingSlope(_) => 0b001,
            Self::FallingSlope(_) => 0b010,
            Self::Toggle(_) => 0b011,
            Self::SinglePulse(_, _) => 0b100,
            Self::ContinuousPulses(_, _) => 0b101,
        }
    }
}

/// Output compare modules configured for non-PWM output compare operations
pub struct Oc<OCMP, TIMEBASE> {
    ocmp: OCMP,
    _timebase: PhantomData<TIMEBASE>,
}

macro_rules! oc_impl {
    ($constructor: ident, $ocmp: ty) => {
        impl<TIMEBASE> Oc<$ocmp, TIMEBASE> {
            pub fn $constructor(ocmp: $ocmp, stop_in_idle_mode: bool) -> Self {
                ocmp.cont.write(|w| w.on().clear_bit().sidl().bit(stop_in_idle_mode));
                Self { ocmp, _timebase: PhantomData }
            }

            /// Deactivate the output compare module and return the PAC object
            pub fn free(self) -> $ocmp {
                self.ocmp.cont.write(|w| w.on().clear_bit());
                self.ocmp
            }

            /// Turn output compare module off
            #[inline(never)] // forces some cycles after OCMP access (as required by data sheet)
            pub fn turn_off(&mut self) {
                self.ocmp.contclr.write(|w| w.on().set_bit());
            }
        }

        impl Oc<$ocmp, Timebase16even> {
            pub fn turn_on(&mut self, config: OcConfig) {
                self.ocmp.cont.modify(|_, w| unsafe {
                    w.oc32().bit(false).octsel().bit(false).ocm().bits(config.ocm_bits())
                });
                let (r, rs) = match config {
                    OcConfig::RisingSlope(r) | OcConfig::FallingSlope(r) | OcConfig::Toggle(r) => {
                        (r, 0)
                    }
                    OcConfig::SinglePulse(r, rs) | OcConfig::ContinuousPulses(r, rs) => (r, rs),
                    OcConfig::Off => (0, 0),
                };
                self.ocmp.r.write(|w| unsafe { w.r().bits(r) });
                self.ocmp.rs.write(|w| unsafe { w.rs().bits(rs) });
                self.ocmp.contset.write(|w| w.on().set_bit());
            }
        }

        impl Oc<$ocmp, Timebase16odd> {
            pub fn turn_on(&mut self, config: OcConfig) {
                self.ocmp.cont.modify(|_, w| unsafe {
                    w.oc32().bit(false).octsel().bit(true).ocm().bits(config.ocm_bits())
                });
                let (r, rs) = match config {
                    OcConfig::RisingSlope(r) | OcConfig::FallingSlope(r) | OcConfig::Toggle(r) => {
                        (r, 0)
                    }
                    OcConfig::SinglePulse(r, rs) | OcConfig::ContinuousPulses(r, rs) => (r, rs),
                    OcConfig::Off => (0, 0),
                };
                self.ocmp.r.write(|w| unsafe { w.r().bits(r) });
                self.ocmp.rs.write(|w| unsafe { w.rs().bits(rs) });
                self.ocmp.contset.write(|w| w.on().set_bit());
            }
        }
        impl Oc<$ocmp, Timebase32> {
            pub fn turn_on(&mut self, config: OcConfig) {
                self.ocmp.cont.modify(|_, w| unsafe {
                    w.oc32().bit(true).octsel().bit(false).ocm().bits(config.ocm_bits())
                });
                let (r, rs) = match config {
                    OcConfig::RisingSlope(r) | OcConfig::FallingSlope(r) | OcConfig::Toggle(r) => {
                        (r, 0)
                    }
                    OcConfig::SinglePulse(r, rs) | OcConfig::ContinuousPulses(r, rs) => (r, rs),
                    OcConfig::Off => (0, 0),
                };
                self.ocmp.r.write(|w| unsafe { w.r().bits(r) });
                self.ocmp.rs.write(|w| unsafe { w.rs().bits(rs) });
                self.ocmp.contset.write(|w| w.on().set_bit());
            }
        }
    };
}

oc_impl!(oc1, OCMP1);
oc_impl!(oc2, OCMP2);
oc_impl!(oc3, OCMP3);
oc_impl!(oc4, OCMP4);
oc_impl!(oc5, OCMP5);

/// Output compare modules configured for PWM
pub struct Pwm<OCMP, TIMEBASE> {
    ocmp: OCMP,
    _timebase: PhantomData<TIMEBASE>,
}

macro_rules! pwm_impl {
    ($constructor: ident, $ocmp: ty, $timer_even: ty, $timer_odd: ty) => {
        impl<TIMEBASE> Pwm<$ocmp, TIMEBASE> {
            /// Initialize the output compare module for 16-bit PWM
            ///
            /// The respetive timer must be set up previously. The HAL code for PWM
            /// reads the respective timer period register to get the maximum duty cycle
            /// value.
            pub fn $constructor(
                ocmp: $ocmp,
                enable_fault_pin: bool,
                stop_in_idle_mode: bool,
            ) -> Self {
                let ocm = match enable_fault_pin {
                    false => 0b110,
                    true => 0b111,
                };
                ocmp.cont.write(|w| unsafe { w.sidl().bit(stop_in_idle_mode).ocm().bits(ocm) });
                Pwm { ocmp, _timebase: PhantomData }
            }

            /// Get fault state
            ///
            /// Returns false if fault pin is not enabled.
            pub fn fault(&self) -> bool {
                self.ocmp.cont.read().ocflt().bit()
            }

            /// Deactivate the output compare module and return the PAC object
            pub fn free(self) -> $ocmp {
                self.ocmp.cont.write(|w| w.on().clear_bit());
                self.ocmp
            }
        }

        impl PwmPin for Pwm<$ocmp, Timebase16even> {
            type Duty = u32;

            fn enable(&mut self) {
                self.ocmp.cont.modify(|_, w| w.octsel().bit(false).oc32().bit(false));
                self.ocmp.contset.write(|w| w.on().set_bit());
            }

            fn disable(&mut self) {
                self.ocmp.contclr.write(|w| w.on().set_bit());
            }

            fn get_duty(&self) -> Self::Duty {
                self.ocmp.rs.read().rs().bits() as Self::Duty
            }

            fn set_duty(&mut self, duty: Self::Duty) {
                self.ocmp.rs.write(|w| unsafe { w.rs().bits(duty) });
            }

            fn get_max_duty(&self) -> Self::Duty {
                unsafe { (*<$timer_even>::ptr()).pr.read().pr().bits() + 1 }
            }
        }

        impl PwmPin for Pwm<$ocmp, Timebase16odd> {
            type Duty = u32;

            fn enable(&mut self) {
                self.ocmp.cont.modify(|_, w| w.octsel().bit(true).oc32().bit(false));
                self.ocmp.contset.write(|w| w.on().set_bit());
            }

            fn disable(&mut self) {
                self.ocmp.contclr.write(|w| w.on().set_bit());
            }

            fn get_duty(&self) -> Self::Duty {
                self.ocmp.rs.read().rs().bits() as Self::Duty
            }

            fn set_duty(&mut self, duty: Self::Duty) {
                self.ocmp.rs.write(|w| unsafe { w.rs().bits(duty) });
            }

            fn get_max_duty(&self) -> Self::Duty {
                unsafe { (*<$timer_odd>::ptr()).pr.read().pr().bits() + 1 }
            }
        }

        impl PwmPin for Pwm<$ocmp, Timebase32> {
            type Duty = u32;

            fn enable(&mut self) {
                self.ocmp.cont.modify(|_, w| w.octsel().bit(false).oc32().bit(true));
                self.ocmp.contset.write(|w| w.on().set_bit());
            }

            fn disable(&mut self) {
                self.ocmp.contclr.write(|w| w.on().set_bit());
            }

            fn get_duty(&self) -> Self::Duty {
                self.ocmp.rs.read().rs().bits() as Self::Duty
            }

            fn set_duty(&mut self, duty: Self::Duty) {
                self.ocmp.rs.write(|w| unsafe { w.rs().bits(duty) });
            }

            fn get_max_duty(&self) -> Self::Duty {
                let pr = unsafe { (*<$timer_even>::ptr()).pr.read().pr().bits() };
                pr.saturating_add(1)
            }
        }

        impl<TIMEBASE> ErrorType for Pwm<$ocmp, TIMEBASE> {
            type Error = Infallible;
        }

        impl SetDutyCycle for Pwm<$ocmp, Timebase16even> {

            fn max_duty_cycle(&self) -> u16 {
                unsafe { (*<$timer_even>::ptr()).pr.read().pr().bits() as u16 + 1 }
            }

            fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
                self.ocmp.cont.modify(|_, w| w.octsel().bit(false).oc32().bit(false));
                self.ocmp.contset.write(|w| w.on().set_bit());
                self.ocmp.rs.write(|w| unsafe { w.rs().bits(duty as u32) });
                Ok(())
            }
        }

        impl SetDutyCycle for Pwm<$ocmp, Timebase16odd> {

            fn max_duty_cycle(&self) -> u16 {
                unsafe { (*<$timer_odd>::ptr()).pr.read().pr().bits() as u16 + 1 }
            }

            fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
                self.ocmp.cont.modify(|_, w| w.octsel().bit(true).oc32().bit(false));
                self.ocmp.contset.write(|w| w.on().set_bit());
                self.ocmp.rs.write(|w| unsafe { w.rs().bits(duty as u32) });
                Ok(())
            }
        }

    };
}

pwm_impl!(oc1, OCMP1, TMR2, TMR3);
pwm_impl!(oc2, OCMP2, TMR2, TMR3);
pwm_impl!(oc3, OCMP3, TMR2, TMR3);
pwm_impl!(oc4, OCMP4, TMR2, TMR3);
pwm_impl!(oc5, OCMP5, TMR2, TMR3);
