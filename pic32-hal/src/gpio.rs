//! General Purpose Input / Output

use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Analog input (type state)
pub struct Analog;
/// Floating input (type state)
pub struct Floating;
/// Pulled down input (type state)
pub struct PullDown;
/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;
/// Open drain output (type state)
pub struct OpenDrain;

macro_rules! port {
    ($PORTX:ident, $portx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty $(, $has_ansel:expr)?),)+
    ]) => {
        /// GPIO
        pub mod $portx {
            use core::marker::PhantomData;

            use crate::hal::digital::v2::*;
            use crate::pac::$PORTX;

            use super::{
                Analog, Floating, GpioExt, Input, OpenDrain, Output,
                PullDown, PullUp, PushPull,
            };

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $PORTX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    Parts {
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(
                        self,
                    ) -> $PXi<Input<Floating>> {
                        unsafe {
                            $(
                                $has_ansel; // dummy statement to satisfy macro processor
                                (*$PORTX::ptr()).anselclr.write(|w| w.bits(1 << $i));
                            )?
                            (*$PORTX::ptr()).trisset.write(|w| w.bits(1 << $i));
                            (*$PORTX::ptr()).cnpuclr.write(|w| w.bits(1 << $i));
                            (*$PORTX::ptr()).cnpdclr.write(|w| w.bits(1 << $i));
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(
                        self,
                    ) -> $PXi<Input<PullDown>> {
                        unsafe {
                            $(
                                $has_ansel; // dummy statement to satisfy macro processor
                                (*$PORTX::ptr()).anselclr.write(|w| w.bits(1 << $i));
                            )?
                            (*$PORTX::ptr()).trisset.write(|w| w.bits(1 << $i));
                            (*$PORTX::ptr()).cnpuclr.write(|w| w.bits(1 << $i));
                            (*$PORTX::ptr()).cnpdset.write(|w| w.bits(1 << $i));
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(
                        self,
                    ) -> $PXi<Input<PullUp>> {
                        unsafe {
                            $(
                                $has_ansel; // dummy statement to satisfy macro processor
                                (*$PORTX::ptr()).anselclr.write(|w| w.bits(1 << $i));
                            )?
                            (*$PORTX::ptr()).trisset.write(|w| w.bits(1 << $i));
                            (*$PORTX::ptr()).cnpuset.write(|w| w.bits(1 << $i));
                            (*$PORTX::ptr()).cnpdclr.write(|w| w.bits(1 << $i));
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open drain output pin
                    pub fn into_open_drain_output(
                        self,
                    ) -> $PXi<Output<OpenDrain>> {
                        unsafe {
                            $(
                                $has_ansel; // dummy statement to satisfy macro processor
                                (*$PORTX::ptr()).anselclr.write(|w| w.bits(1 << $i));
                            )?
                            (*$PORTX::ptr()).trisclr.write(|w| w.bits(1 << $i));
                            (*$PORTX::ptr()).odcset.write(|w|  w.bits(1 << $i));
                            (*$PORTX::ptr()).cnpuclr.write(|w| w.bits(1 << $i));
                            (*$PORTX::ptr()).cnpdclr.write(|w| w.bits(1 << $i));
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(
                        self,
                    ) -> $PXi<Output<PushPull>> {
                        unsafe {
                            $(
                                $has_ansel; // dummy statement to satisfy macro processor
                                (*$PORTX::ptr()).anselclr.write(|w| w.bits(1 << $i));
                            )?
                            (*$PORTX::ptr()).trisclr.write(|w| w.bits(1 << $i));
                            (*$PORTX::ptr()).odcclr.write(|w| w.bits(1 << $i));
                            (*$PORTX::ptr()).cnpuclr.write(|w| w.bits(1 << $i));
                            (*$PORTX::ptr()).cnpdclr.write(|w| w.bits(1 << $i));
                        }
                        $PXi { _mode: PhantomData }
                    }

                    $(
                        /// Configures the pin to operate as an analog input pin
                        pub fn into_analog_input(
                            self,
                        ) -> $PXi<Input<Analog>> {
                            $has_ansel; // dummy statement to satisfy macro processor
                            unsafe {
                                (*$PORTX::ptr()).anselset.write(|w| w.bits(1 << $i));
                                (*$PORTX::ptr()).trisset.write(|w| w.bits(1 << $i));
                                (*$PORTX::ptr()).cnpuclr.write(|w| w.bits(1 << $i));
                                (*$PORTX::ptr()).cnpdclr.write(|w| w.bits(1 << $i));
                            }
                            $PXi { _mode: PhantomData }
                        }
                    )?
                }

                impl $PXi<Output<OpenDrain>> {
                    /// Enables / disables the internal pull up
                    pub fn internal_pull_up(&mut self, on: bool) {
                        unsafe {
                            if on {
                                (*$PORTX::ptr()).cnpuset.write(|w| w.bits(1 << $i));
                            } else {
                                (*$PORTX::ptr()).cnpuclr.write(|w| w.bits(1 << $i));
                            }
                        }
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {

                    type Error = ();

                    fn set_high(&mut self) -> Result<(), Self::Error>  {
                         // NOTE(unsafe) atomic write to a stateless register
                         unsafe { (*$PORTX::ptr()).latset.write(|w| w.bits(1 << $i)) }
                        Ok(())
                    }

                    fn set_low(&mut self) ->  Result<(), Self::Error>{
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$PORTX::ptr()).latclr.write(|w| w.bits(1 << $i)) }
                        Ok(())
                    }
                }

                impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {

                    fn is_set_high(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).lat.read().bits() & (1 << $i) != 0 })
                    }

                    fn is_set_low(&self) -> Result<bool, Self::Error> {
                        self.is_set_high().map(|b| !b)
                    }
                }

                impl<MODE> ToggleableOutputPin for $PXi<Output<MODE>> {

                    type Error = ();

                    fn toggle(&mut self) -> Result<(), Self::Error> {
                        unsafe { (*$PORTX::ptr()).latinv.write(|w| w.bits(1 << $i)) };
                        Ok(())
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {

                    type Error = ();

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).port.read().bits() & (1 << $i) != 0 })
                    }

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        self.is_high().map(|b| !b)
                    }
                }

                impl InputPin for $PXi<Output<OpenDrain>> {

                    type Error = ();

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).port.read().bits() & (1 << $i) != 0 })
                    }

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        self.is_high().map(|b| !b)
                    }
                }
            )+
        }
    }
}

// configuration for general purpose (non-USB) devices
#[cfg(any(feature = "pic32mx1xxfxxxb", feature = "pic32mx2xxfxxxb"))]
port!(PORTA, porta, [
    RA0: (ra0, 0, Input<Analog>, true),
    RA1: (ra1, 1, Input<Analog>, true),
    RA2: (ra2, 2, Input<Floating>),
    RA3: (ra3, 3, Input<Floating>),
    RA4: (ra4, 4, Input<Floating>),

    RA7: (ra7, 7, Input<Floating>),
    RA8: (ra8, 8, Input<Floating>),
    RA9: (ra9, 9, Input<Floating>),
    RA10: (ra10, 10, Input<Floating>),
]);

#[cfg(feature = "pic32mx1xxfxxxb")]
port!(PORTB, portb, [
    RB0: (rb0, 0, Input<Analog>, true),
    RB1: (rb1, 1, Input<Analog>, true),
    RB2: (rb2, 2, Input<Analog>, true),
    RB3: (rb3, 3, Input<Analog>, true),
    RB4: (rb4, 4, Input<Floating>),
    RB5: (rb5, 5, Input<Floating>),
    RB6: (rb6, 6, Input<Floating>),
    RB7: (rb7, 7, Input<Floating>),
    RB8: (rb8, 8, Input<Floating>),
    RB9: (rb9, 9, Input<Floating>),
    RB10: (rb10, 10, Input<Floating>),
    RB11: (rb11, 11, Input<Floating>),
    RB12: (rb12, 12, Input<Analog>, true),
    RB13: (rb13, 13, Input<Analog>, true),
    RB14: (rb14, 14, Input<Analog>, true),
    RB15: (rb15, 15, Input<Analog>, true),
]);

#[cfg(feature = "pic32mx2xxfxxxb")]
port!(PORTB, portb, [
    RB0: (rb0, 0, Input<Analog>, true),
    RB1: (rb1, 1, Input<Analog>, true),
    RB2: (rb2, 2, Input<Analog>, true),
    RB3: (rb3, 3, Input<Analog>, true),
    RB4: (rb4, 4, Input<Floating>),
    RB5: (rb5, 5, Input<Floating>),
    RB7: (rb7, 7, Input<Floating>),
    RB8: (rb8, 8, Input<Floating>),
    RB9: (rb9, 9, Input<Floating>),
    RB10: (rb10, 10, Input<Floating>),
    RB11: (rb11, 11, Input<Floating>),
    RB13: (rb13, 13, Input<Analog>, true),
    RB14: (rb14, 14, Input<Analog>, true),
    RB15: (rb15, 15, Input<Analog>, true),
]);

// PIC32MX2xx 28pin XLP USB devices
#[cfg(feature = "pic32mx2x4fxxxb")]
port!(PORTA, porta, [
    RA0: (ra0, 0, Input<Analog>, true),
    RA1: (ra1, 1, Input<Analog>, true),
    RA2: (ra2, 2, Input<Floating>),
    RA3: (ra3, 3, Input<Floating>),
    RA4: (ra4, 4, Input<Floating>),
]);

#[cfg(feature = "pic32mx2x4fxxxb")]
port!(PORTB, portb, [
    RB0: (rb0, 0, Input<Analog>, true),
    RB1: (rb1, 1, Input<Analog>, true),
    RB2: (rb2, 2, Input<Analog>, true),
    RB3: (rb3, 3, Input<Analog>, true),
    RB4: (rb4, 4, Input<Floating>),
    RB5: (rb5, 5, Input<Floating>),
    RB7: (rb7, 7, Input<Floating>),
    RB8: (rb8, 8, Input<Floating>),
    RB9: (rb9, 9, Input<Floating>),
    RB13: (rb13, 13, Input<Analog>, true),
    RB14: (rb14, 14, Input<Analog>, true),
    RB15: (rb15, 15, Input<Analog>, true),
]);
