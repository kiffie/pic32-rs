//! SPI driver (SPI master)

use crate::pac::{SPI1, SPI2};
use core::{cmp::max, slice};
use embedded_hal::spi::{ErrorKind, ErrorType, SpiBus};
pub use embedded_hal_0_2::spi::{Mode, Phase, Polarity};

/// SPI error
pub type Error = ErrorKind;

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
            pub fn $Id(spi: $Spi, proto: Proto, clock_div: u32) -> Self {
                spi.con1.write(|w| unsafe { w.bits(0) }); // first turn SPI off
                let brg1 = clock_div / 2;
                let brg = if brg1 > 0 { brg1 - 1 } else { brg1 };
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
                        spi.con1.write(|w| {
                            w.enhbuf()
                                .bit(true)
                                .cke()
                                .bit(cke)
                                .ckp()
                                .bit(ckp)
                                .msten()
                                .bit(true)
                                .on()
                                .bit(true)
                        });
                    }
                    Proto::AudioI2s(frame_format) => {
                        spi.con2.write(|w| unsafe {
                            w.ignrov()
                                .bit(true)
                                .igntur()
                                .bit(true)
                                .auden()
                                .bit(true)
                                .audmod()
                                .bits(0b00) // I2S mode
                        });
                        spi.con1.write(|w| unsafe {
                            w.mclksel()
                                .bit(true) // use reference clock
                                .enhbuf()
                                .bit(true)
                                .mode32()
                                .bit(frame_format.mode32())
                                .mode16()
                                .bit(frame_format.mode16())
                                .ckp()
                                .bit(true)
                                .stxisel()
                                .bits(0b11) // IRQ when buffer not full
                                .srxisel()
                                .bits(0b01) // IRQ when buffer is not empty
                                .msten()
                                .bit(true)
                                .on()
                                .bit(true)
                        });
                    }
                }
                Spi { spi }
            }

            pub fn free(self) -> $Spi {
                self.spi.con1.write(|w| w.on().bit(false)); // turn SPI off
                self.spi
            }
        }

        impl embedded_hal_0_2::spi::FullDuplex<u8> for Spi<$Spi> {
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

        impl embedded_hal_0_2::blocking::spi::transfer::Default<u8> for Spi<$Spi> {}
        impl embedded_hal_0_2::blocking::spi::write::Default<u8> for Spi<$Spi> {}

        impl ErrorType for Spi<$Spi> {
            type Error = Error;
        }

        impl SpiBus<u8> for Spi<$Spi> {
            fn read(&mut self, bytes: &mut [u8]) -> Result<(), Self::Error> {
                self.transfer(bytes, &[])
            }

            fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
                self.transfer(&mut [], bytes)
            }

            fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
                let xfer_len = max(read.len(), write.len());
                let mut rd_ctr = xfer_len;
                let mut wr_ctr = xfer_len;
                let mut rd = read.iter_mut().fuse();
                let mut wr = write.iter().fuse();
                while rd_ctr > 0 || wr_ctr > 0 {
                    // write to FIFO
                    if wr_ctr > 0 && !self.spi.stat.read().spitbf().bit() {
                        let byte = *wr.next().unwrap_or(&0);
                        self.spi.buf.write(|w| unsafe { w.bits(byte as u32) });
                        wr_ctr -= 1;
                    }
                    // read from FIFO
                    if rd_ctr > 0 && !self.spi.stat.read().spirbe().bit() {
                        let byte = self.spi.buf.read().bits() as u8;
                        if let Some(b) = rd.next() {
                            *b = byte;
                        }
                        rd_ctr -= 1;
                    }
                }
                Ok(())
            }

            fn transfer_in_place(&mut self, bytes: &mut [u8]) -> Result<(), Self::Error> {
                let ptr = bytes.as_mut_ptr();
                let len = bytes.len();
                // This unsafe code works because transfer() always reads from a
                // given byte position before writing back to it.
                let read = unsafe { slice::from_raw_parts_mut(ptr, len) };
                let write = unsafe { slice::from_raw_parts(ptr, len) };
                self.transfer(read, write)
            }

            fn flush(&mut self) -> Result<(), Self::Error> {
                Ok(())
            }
        }
    };
}

spi!(spi1, SPI1);
spi!(spi2, SPI2);
