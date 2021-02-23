//! I2C driver for PIC32

use crate::pac::{I2C1, I2C2};
use crate::time::Hertz;
use crate::dma;
use crate::int::InterruptSource;
use embedded_hal::blocking;
use mips_mcu::PhysicalAddress;
use mips_mcu::fmt::virt_to_phys;

/// I2C clock frequency specifier
/// The values of this enum correspond to the divisor values mentioned in the
/// reference manual
#[repr(u32)]
pub enum Fscl {
    F100KHZ = 204248,
    F400KHZ = 872600,
    F1000KHZ = 2525253,
}

/// I2C Error type
#[derive(Debug, Clone)]
pub enum Error {
    TransactionFailed,
    InvalidState,
}

/// An I2C driver for the PIC32. Contains primitives `transmit()`, `receive()`,
/// `rstart()`, `stop()` that can be called one after another to build a complex
/// I2C transaction. A Transaction must be started with `transmit()` and
/// concluded with `stop()`
pub struct I2c<I2C> {
    i2c: I2C,
    transaction_ongoing: bool,
}

/// Primitive I2C Operations
pub trait Ops {
    fn transmit(&mut self, data: &[u8]) -> Result<(), Error>;

    /// Initiate DMA transmission
    ///
    /// # Safety
    ///
    /// Caller must make sure that the physical address block specified by
    /// `addr` and `len` refers to a valid memory block during the whole
    /// DMA transfer.
    unsafe fn transmit_dma<D: dma::Ops>(&mut self, dma: &D, addr: PhysicalAddress, len: usize) -> Result<(), Error>;
    fn rstart(&self) -> Result<(), Error>;
    fn stop(&mut self);
    fn receive(&mut self, data: &mut [u8], nack_last: bool) -> Result<(), Error>;
}

macro_rules! i2c_impl {
    ($Id:ident, $I2c:ident) => {
        impl I2c<$I2c> {
            /// Create a new I2C object
            pub fn $Id(i2c: $I2c, pb_clock: Hertz, fscl: Fscl) -> I2c<$I2c> {
                let divisor = fscl as u32;
                let round = if pb_clock.0 % divisor > divisor / 2 { 1 } else { 0 };
                let brg = pb_clock.0 / divisor - 2 + round;
                unsafe {
                    i2c.brg.write(|w| w.brg().bits(brg as u16));
                    // disable slew rate control, see PIC32MX1xx/2xxx Silicon Errata, item 17
                    i2c.cont.write(|w| w.on().bit(true).disslw().bit(true));
                }
                I2c {
                    i2c,
                    transaction_ongoing: false,
                }
            }

            /// Destroy I2C object and return i2c HAL object
            pub fn free(self) -> $I2c {
                self.i2c
            }

            /// Returns true if busy
            fn i2c_busy(&self) -> bool {
                (self.i2c.cont.read().bits() & 0x1f) != 0
            }
        }

        impl Ops for I2c<$I2c> {
            /// Transmit data over the bus. Generate a START condition if called
            /// first during a transaction.
            fn transmit(&mut self, data: &[u8]) -> Result<(), Error> {
                if !self.transaction_ongoing {
                    while self.i2c_busy() {}
                    // generate start condition
                    self.i2c.contset.write(|w| w.sen().bit(true));
                    self.transaction_ongoing = true;
                }
                for byte in data {
                    while self.i2c_busy() {}
                    unsafe { self.i2c.trn.write(|w| w.trn().bits(*byte)) };
                    // wait until TX complete
                    while self.i2c.stat.read().trstat().bit() {}
                    // check for NACK
                    if self.i2c.stat.read().ackstat().bit() {
                        self.stop();
                        return Err(Error::TransactionFailed);
                    }
                }
                Ok(())
            }

            unsafe fn transmit_dma<D: dma::Ops>(&mut self, dma: &D, addr: PhysicalAddress, len: usize) -> Result<(), Error> {
                if !self.transaction_ongoing {
                    while self.i2c_busy() {}
                    // generate start condition
                    self.i2c.contset.write(|w| w.sen().bit(true));
                    self.transaction_ongoing = true;
                }
                dma.set_source(addr, len);
                let trn = &self.i2c.trn as *const _ as *mut u32;
                dma.set_dest(virt_to_phys(trn), 1);
                dma.set_cell_size(1);
                dma.set_start_event(Some(InterruptSource::I2C1_MASTER));
                dma.enable(dma::XferMode::OneShot);
                dma.force();
                Ok(())
            }

            /// Generate a repeated start condition
            /// A transaction must be started before calling this functions
            fn rstart(&self) -> Result<(), Error> {
                if self.transaction_ongoing {
                    while self.i2c_busy() {}
                    self.i2c.contset.write(|w| w.rsen().bit(true));
                    Ok(())
                } else {
                    Err(Error::InvalidState)
                }
            }

            /// Generate a stop condition and terminate the I2C transfer
            fn stop(&mut self) {
                while self.i2c_busy() {}
                self.i2c.contset.write(|w| w.pen().bit(true));
                self.transaction_ongoing = false;
            }

            /// Receive data.len() bytes. nack_last determines whether a NACK shall
            /// be created after the reception of the last byte
            /// A transaction must be started before calling this function
            fn receive(&mut self, data: &mut [u8], nack_last: bool) -> Result<(), Error> {
                if !self.transaction_ongoing {
                    return Err(Error::InvalidState);
                }
                let len = data.len();
                for (i, byte) in data.iter_mut().enumerate() {
                    while self.i2c_busy() {}
                    self.i2c.contset.write(|w| w.rcen().bit(true));
                    while self.i2c_busy() {}
                    *byte = self.i2c.rcv.read().rcv().bits();
                    if (i == len - 1) && nack_last {
                        // NACK for last byte
                        self.i2c.contset.write(|w| w.ackdt().bit(true));
                    } else {
                        self.i2c.contclr.write(|w| w.ackdt().bit(true));
                    }
                    self.i2c.contset.write(|w| w.acken().bit(true));
                }
                Ok(())
            }
        }

        impl blocking::i2c::Write for I2c<$I2c> {
            type Error = Error;

            fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
                self.transmit(&[addr << 1])?;
                self.transmit(bytes)?;
                self.stop();
                Ok(())
            }
        }

        impl blocking::i2c::Read for I2c<$I2c> {
            type Error = Error;

            fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
                self.transmit(&[(addr << 1) | 0x01])?;
                self.receive(buffer, true)?;
                self.stop();
                Ok(())
            }
        }

        impl blocking::i2c::WriteRead for I2c<$I2c> {
            type Error = Error;

            fn write_read(
                &mut self,
                addr: u8,
                bytes: &[u8],
                buffer: &mut [u8],
            ) -> Result<(), Self::Error> {
                self.transmit(&[addr << 1])?;
                self.transmit(bytes)?;
                self.rstart()?;
                self.transmit(&[(addr << 1) | 0x01])?;
                self.receive(buffer, true)?;
                self.stop();
                Ok(())
            }
        }
    };
}

i2c_impl!(i2c1, I2C1);
i2c_impl!(i2c2, I2C2);
