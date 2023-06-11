//! Peripheral Pin Select (PPS)

use crate::gpio;
use crate::pac::PPS;

/// Extension trait to split a PPS peripheral into Parts corresponding to
/// virtual ports
pub trait PpsExt {
    /// The type to split the PPS into
    type Parts;

    /// Splits the PPS peripheral into virtual ports
    fn split(self) -> Self::Parts;
}

/// Virtual input and output ports created from the PPS peripheral struct
pub struct Parts {
    pub inputs: input::Inputs,
    pub outputs: output::Outputs,
}

/// Phyiscal pin P that can be mapped to a virtual pin V
pub trait MapPin<P, V> {
    /// Map this physical pin to `virt_pin`
    fn map_pin(self, virt_pin: V) -> MappedPin<P, V>;
}

/// Dummy to be used instead of a physical pin to leave a virtual pin of a
/// peripheral unconnected
pub struct NoPin;

impl NoPin {
    // Create a new NoPin
    #[allow(clippy::new_without_default)]
    pub fn new() -> NoPin {
        NoPin
    }
}

/// Move a virtual pin into a respective `MappedPin` mapped to a `NoPin`
#[macro_export]
macro_rules! pps_no_pin {
    ($VPIN:expr) => { NoPin::new().map_pin($VPIN) }
}

impl<V> MapPin<NoPin, V> for NoPin {
    fn map_pin(self, virt_pin: V) -> MappedPin<NoPin, V> {
        MappedPin {
            phys: self,
            virt: virt_pin,
        }
    }
}

/// Indicates whether a Mapped pin is connected to a physical pin or to a NoPin
pub trait IsConnected {
    /// true if connected to a physical pin
    const IS_CONNECTED: bool = true;

    fn is_connected(&self) -> bool {
        Self::IS_CONNECTED
    }
}

impl<V> IsConnected for MappedPin<NoPin, V> {
    const IS_CONNECTED: bool = false;
}

/// Physical pin mapped to a particular virtual pin
pub struct MappedPin<P, V> {
    phys: P,
    virt: V,
}

macro_rules! pps_tables {
    { {$( $IDTYPE:ident, $IDNAME:ident );+ ;}
      {$( $ODTYPE:ident, $ODNAME:ident );+ ;}
      {$( $VINTYPE:ident, $INREG:ident { $( $INPTYPE:path, $INVAL:expr; )+ } )+ }
      {$( $OPTYPE:path, $OREG:ident   { $( $VOUTTYPE:ident, $OVAL:expr; )+ } )+ } }
    => {

    /// Virtual input pins
    pub mod input {
        $(
            pub struct $IDTYPE {
                pub(super) _dummy: ()
            }
        )+

        pub struct Inputs {
            $(
                pub $IDNAME: $IDTYPE,
            )+
        }
    }

    /// Virtual output pins
    pub mod output {
        $(
            pub struct $ODTYPE {
                pub(super) _dummy: (),
            }
        )+

        pub struct Outputs {
            $(
                pub $ODNAME: $ODTYPE,
            )+
        }
    }

    impl PpsExt for PPS {

        type Parts = Parts;

        /// Splits the PPS peripheral into virtual ports
        fn split(self) -> Self::Parts {
            Parts {
                inputs: input::Inputs {
                    $(
                        $IDNAME: input::$IDTYPE { _dummy: () },
                    )+
                },
                outputs: output::Outputs {
                    $(
                        $ODNAME: output::$ODTYPE { _dummy: () },
                    )+
                }
            }
        }
    }

    $(
        $(
            impl<MODE> MapPin<$INPTYPE, input::$VINTYPE> for $INPTYPE {

                fn map_pin(self, virt_pin: input::$VINTYPE) -> MappedPin<$INPTYPE, input::$VINTYPE> {
                    unsafe {(*PPS::ptr()).$INREG.write(|w| w.$INREG().bits($INVAL))};
                    MappedPin{ phys: self, virt: virt_pin }
                }
            }

            impl<MODE> IsConnected for MappedPin<$INPTYPE, input::$VINTYPE> {}

            impl<MODE> MappedPin<$INPTYPE, input::$VINTYPE> {

                pub fn unmap_pin(self) -> ($INPTYPE, input::$VINTYPE) {
                    (self.phys, self.virt)
                }
            }
        )+
    )+

    $(
        $(
            impl<MODE> MapPin<$OPTYPE, output::$VOUTTYPE> for $OPTYPE {

                fn map_pin(self, virt_pin: output::$VOUTTYPE) -> MappedPin<$OPTYPE, output::$VOUTTYPE> {
                    unsafe {(*PPS::ptr()).$OREG.write(|w| w.$OREG().bits($OVAL))};
                    MappedPin { phys: self, virt: virt_pin}
                }
            }

            impl <MODE> IsConnected for MappedPin<$OPTYPE, output::$VOUTTYPE> {}

            impl<MODE> MappedPin<$OPTYPE, output::$VOUTTYPE> {

                pub fn unmap_pin(self) -> ($OPTYPE, output::$VOUTTYPE) {
                    unsafe { (*PPS::ptr()).$OREG.write(|w| w.$OREG().bits(0)) };
                    (self.phys, self.virt)
                }
            }
        )+
    )+
}}

include!("pps_tables.rs");
