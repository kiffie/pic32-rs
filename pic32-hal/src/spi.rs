///
/// SPI driver for PIC32
///

pub use crate::hal::spi::{Mode, Phase, Polarity};
use crate::pac::{SPI1, SPI2};

use nb;
use crate::clock::Osc;
use crate::time::Hertz;

/// SPI error
#[derive(Debug)]
pub enum Error {
    /// Overrun occurred
    Overrun,
}

pub struct Spi<SPI> {
    spi: SPI
}


macro_rules! spi {
    ($Id:ident, $Spi:ident) => {
        impl Spi<$Spi> {
            pub fn $Id(
                spi: $Spi,
                mode: Mode,
                bitrate: Hertz,
                osc: &Osc
            ) -> Self {
                spi.con.write(|w| unsafe { w.bits(0) }); // turn SPI off
                let brg1 = osc.pb_clock().0 / bitrate.0 / 2;
                let brg = if brg1 > 0 { brg1 -1 } else { brg1 };
                spi.brg.write(|w| unsafe { w.bits(brg) });
                let ckp = match mode.polarity {
                    Polarity::IdleLow => false,
                    Polarity::IdleHigh => true,
                };
                let cke = match mode.phase {
                    Phase::CaptureOnFirstTransition => true,
                    Phase::CaptureOnSecondTransition => false,
                };
                spi.con2.write(|w| unsafe { w.bits(0) });
                spi.con.write(|w| { w
                    .enhbuf().bit(true)
                    .cke().bit(cke)
                    .ckp().bit(ckp)
                    .msten().bit(true)
                    .on().bit(true)
                });
                Spi { spi }
            }

            pub fn free(self) -> $Spi {
                self.spi.con.write(|w| w.on().bit(false)); // turn SPI off
                self.spi
            }
        }

        impl crate::hal::spi::FullDuplex<u8> for Spi<$Spi> {
            type Error = Error;

            fn read(&mut self) -> nb::Result<u8, Error> {
                if self.spi.stat.read().spirbe().bit() {
                    Err(nb::Error::WouldBlock)
                } else if self.spi.stat.read().spirov().bit() {
                    self.spi.statclr.write(|w| w.spirov().bit(true));
                    Err(nb::Error::Other(Error::Overrun))
                } else {
                    let byte = self.spi.buf.read().bits() as u8;
                    Ok(byte)
                }
            }

            fn send(&mut self, byte: u8) -> nb::Result<(), Error> {
                if self.spi.stat.read().spitbf().bit() {
                    Err(nb::Error::WouldBlock)
                } else {
                    self.spi.buf.write(|w| unsafe { w.bits(byte as u32) });
                    Ok(())
                }
            }
        }

        impl crate::hal::blocking::spi::transfer::Default<u8> for Spi<$Spi> {}
        impl crate::hal::blocking::spi::write::Default<u8> for Spi<$Spi> {}
    }
}

spi!(spi1, SPI1);
spi!(spi2, SPI2);

