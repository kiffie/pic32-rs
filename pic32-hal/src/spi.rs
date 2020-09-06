//! SPI driver (SPI master)

pub use crate::hal::spi::{Mode, Phase, Polarity};
use crate::pac::{SPI1, SPI2};

use nb;

/// SPI error
#[derive(Debug)]
pub enum Error {
    /// Overrun occurred
    Overrun,
}

pub enum Proto {
     Spi(Mode),
     AudioI2s(AudioFrameFormat),
}

/// Length of audio frame and length of sample/subframe
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AudioFrameFormat {
    /// 32 bit frame, 16 bit samples
    F32S16,
    /// 64 bit frame, 16 bit samples
    F64S16,
    /// 64 bit frame, 24 bit samples
    F64S24,
    /// 64 bit frame, 32 bit samples
    F64S32,
}

impl AudioFrameFormat {
    fn mode16(self) -> bool {
        match self {
            AudioFrameFormat::F32S16 => false,
            AudioFrameFormat::F64S16 => true,
            AudioFrameFormat::F64S24 => true,
            AudioFrameFormat::F64S32 => false,
        }
    }

    fn mode32(self) -> bool {
        match self {
            AudioFrameFormat::F32S16 => false,
            AudioFrameFormat::F64S16 => false,
            AudioFrameFormat::F64S24 => true,
            AudioFrameFormat::F64S32 => true,
        }
    }
}

pub struct Spi<SPI> {
    spi: SPI,
}

macro_rules! spi {
    ($Id:ident, $Spi:ident) => {
        impl Spi<$Spi> {
            /// create an SPI instance
            ///
            /// clock_div: clock divisor to initialize the BRG with. Only even
            /// values can be configured, i.e. the LSB will be ignored. When a
            /// value < 2 is given it will be replaced with 2.
            /// `BRG = MAX(clock_div / 2 - 1, 0)`
            pub fn $Id(
                spi: $Spi,
                proto: Proto,
                clock_div: u32,
            ) -> Self {
                spi.con1.write(|w| unsafe { w.bits(0) }); // first turn SPI off
                let brg1 = clock_div / 2;
                let brg = if brg1 > 0 { brg1 -1 } else { brg1 };
                spi.brg.write(|w| unsafe { w.bits(brg) });
                match proto {
                    Proto::Spi(mode) => {
                        let ckp = match mode.polarity {
                            Polarity::IdleLow => false,
                            Polarity::IdleHigh => true,
                        };
                        let cke = match mode.phase {
                            Phase::CaptureOnFirstTransition => true,
                            Phase::CaptureOnSecondTransition => false,
                        };
                        spi.con2.write(|w| unsafe { w.bits(0) });
                        spi.con1.write(|w| { w
                            .enhbuf().bit(true)
                            .cke().bit(cke)
                            .ckp().bit(ckp)
                            .msten().bit(true)
                            .on().bit(true)
                        });
                    },
                    Proto::AudioI2s(frame_format) => {
                        spi.con2.write(|w| unsafe { w
                            .ignrov().bit(true)
                            .igntur().bit(true)
                            .auden().bit(true)
                            .audmod().bits(0b00) // I2S mode
                        });
                        spi.con1.write(|w| unsafe { w
                            .mclksel().bit(true) // use reference clock
                            .enhbuf().bit(true)
                            .mode32().bit(frame_format.mode32())
                            .mode16().bit(frame_format.mode16())
                            .ckp().bit(true)
                            .stxisel().bits(0b11) // IRQ when buffer not full
                            .msten().bit(true)
                            .on().bit(true)
                        });
                    },
                }
                Spi { spi }
            }

            pub fn free(self) -> $Spi {
                self.spi.con1.write(|w| w.on().bit(false)); // turn SPI off
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

