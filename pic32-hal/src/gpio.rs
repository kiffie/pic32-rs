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
            use core::convert::Infallible;

            use embedded_hal_0_2::digital::v2 as eh02;
            use embedded_hal::digital as eh;
            use crate::pac::$PORTX;

            #[allow(unused_imports)]
            use super::Analog;
            use super::{
                Floating, GpioExt, Input, OpenDrain, Output,
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
                                _ = $has_ansel; // dummy statement to satisfy macro processor
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
                                _ = $has_ansel; // dummy statement to satisfy macro processor
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
                                _ = $has_ansel; // dummy statement to satisfy macro processor
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
                                _ = $has_ansel; // dummy statement to satisfy macro processor
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
                                _ = $has_ansel; // dummy statement to satisfy macro processor
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
                            _ = $has_ansel; // dummy statement to satisfy macro processor
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

                impl<MODE> eh02::OutputPin for $PXi<Output<MODE>> {

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

                impl<MODE> eh02::StatefulOutputPin for $PXi<Output<MODE>> {

                    fn is_set_high(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).lat.read().bits() & (1 << $i) != 0 })
                    }

                    fn is_set_low(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).lat.read().bits() & (1 << $i) == 0 })
                    }
                }

                impl<MODE> eh02::ToggleableOutputPin for $PXi<Output<MODE>> {

                    type Error = ();

                    fn toggle(&mut self) -> Result<(), Self::Error> {
                        unsafe { (*$PORTX::ptr()).latinv.write(|w| w.bits(1 << $i)) };
                        Ok(())
                    }
                }

                impl<MODE> eh02::InputPin for $PXi<Input<MODE>> {

                    type Error = ();

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).port.read().bits() & (1 << $i) != 0 })
                    }

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).port.read().bits() & (1 << $i) == 0 })
                    }
                }

                impl eh02::InputPin for $PXi<Output<OpenDrain>> {

                    type Error = ();

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).port.read().bits() & (1 << $i) != 0 })
                    }

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).port.read().bits() & (1 << $i) == 0 })
                    }
                }

                impl<MODE> eh::ErrorType for $PXi<MODE> {
                    type Error = Infallible;
                }

                impl<MODE> eh::InputPin for $PXi<Input<MODE>> {

                    fn is_high(&mut self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).port.read().bits() & (1 << $i) != 0 })
                    }

                    fn is_low(&mut self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).port.read().bits() & (1 << $i) == 0 })
                    }
                }

                impl eh::InputPin for $PXi<Output<OpenDrain>> {

                    fn is_high(&mut self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).port.read().bits() & (1 << $i) != 0 })
                    }

                    fn is_low(&mut self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).port.read().bits() & (1 << $i) == 0 })
                    }
                }

                impl<MODE> eh::OutputPin for $PXi<Output<MODE>> {

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        unsafe { (*$PORTX::ptr()).latclr.write(|w| w.bits(1 << $i)) }
                        Ok(())
                    }

                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        unsafe { (*$PORTX::ptr()).latset.write(|w| w.bits(1 << $i)) }
                        Ok(())
                    }
                }

                impl<MODE> eh::StatefulOutputPin for $PXi<Output<MODE>> {

                    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).lat.read().bits() & (1 << $i) != 0 })
                    }

                    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                        Ok(unsafe { (*$PORTX::ptr()).lat.read().bits() & (1 << $i) == 0 })
                    }

                    fn toggle(&mut self) -> Result<(), Self::Error> {
                        unsafe { (*$PORTX::ptr()).latinv.write(|w| w.bits(1 << $i)) };
                        Ok(())
                    }
                }
            )+
        }
    }
}

include!("gpio_tables.rs");
