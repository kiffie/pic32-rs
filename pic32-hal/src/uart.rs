//! UART driver

use core::convert::Infallible;
use core::fmt;
use core::marker::PhantomData;

use crate::clock::Osc;
use crate::pac::{UART1, UART2};
use crate::pps::{input, output, IsConnected, MappedPin};

use embedded_hal_0_2::prelude::*;
use embedded_io::{ReadReady, WriteReady};
use nb::block;

/// Uart
pub struct Uart<UART, RX, TX> {
    _uart: PhantomData<UART>,
    rx: RX,
    tx: TX,
}

/// Uart receiver
pub struct Rx<UART> {
    _uart: PhantomData<UART>,
}

/// Uart transmitter
pub struct Tx<UART> {
    _uart: PhantomData<UART>,
}

/// UART read errors
#[derive(Debug)]
pub enum ReadError {
    /// RX FIFO overrun
    Overrun,

    /// Parity error
    Parity,

    /// Framing error
    Framing,

    /// Break detected
    Break,
}

impl embedded_io::Error for ReadError {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

macro_rules! uart_impl {
    ($Id:ident, $Uart:ident, $Rx:ty, $Tx:ty) => {
        impl<RX, TX> Uart<$Uart, MappedPin<RX, $Rx>, MappedPin<TX, $Tx>> {
            pub fn $Id(
                uart: $Uart,
                osc: &Osc,
                baudrate: u32,
                rx: MappedPin<RX, $Rx>,
                tx: MappedPin<TX, $Tx>,
            ) -> Uart<$Uart, MappedPin<RX, $Rx>, MappedPin<TX, $Tx>>
            where
                MappedPin<RX, $Rx>: IsConnected,
                MappedPin<TX, $Tx>: IsConnected,
            {
                let brg = osc.pb_clock().0 / (4 * baudrate) - 1;
                let has_rx = rx.is_connected();
                let has_tx = tx.is_connected();
                unsafe {
                    uart.mode.write(|w| w.bits(0));
                    uart.mode.write(|w| w.brgh().bit(true));
                    uart.sta.write(|w| {
                        w.urxen()
                            .bit(has_rx)
                            .utxen()
                            .bit(has_tx)
                            .urxisel()
                            .bits(0b10)
                    });
                    uart.brg.write(|w| w.bits(brg));
                    uart.modeset.write(|w| w.on().bit(true));
                }
                Uart {
                    _uart: PhantomData,
                    rx,
                    tx,
                }
            }

            pub fn free(self) -> (MappedPin<RX, $Rx>, MappedPin<TX, $Tx>) {
                unsafe { (*$Uart::ptr()).modeclr.write(|w| w.on().bit(true)) };
                (self.rx, self.tx)
            }

            pub fn split(self) -> (Tx<$Uart>, Rx<$Uart>) {
                (Tx { _uart: PhantomData }, Rx { _uart: PhantomData })
            }
        }

        impl embedded_hal_0_2::serial::Write<u8> for Uart<$Uart, $Rx, $Tx> {
            type Error = ();

            fn flush(&mut self) -> nb::Result<(), Self::Error> {
                let mut tx: Tx<$Uart> = Tx { _uart: PhantomData };
                tx.flush()
            }

            fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
                let mut tx: Tx<$Uart> = Tx { _uart: PhantomData };
                tx.write(byte)
            }
        }

        impl embedded_hal_0_2::serial::Write<u8> for Tx<$Uart> {
            type Error = ();

            fn flush(&mut self) -> nb::Result<(), Self::Error> {
                let trmt = unsafe { (*$Uart::ptr()).sta.read().trmt().bit() };
                if trmt {
                    Err(nb::Error::WouldBlock)
                } else {
                    Ok(())
                }
            }

            fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
                let utxbf = unsafe { (*$Uart::ptr()).sta.read().utxbf().bit() };
                if utxbf {
                    Err(nb::Error::WouldBlock)
                } else {
                    unsafe {
                        (*$Uart::ptr()).txreg.write(|w| w.txreg().bits(byte as u16));
                    }
                    Ok(())
                }
            }
        }

        impl embedded_hal_0_2::serial::Read<u8> for Uart<$Uart, $Rx, $Tx> {
            type Error = ();

            fn read(&mut self) -> nb::Result<u8, Self::Error> {
                let mut rx: Rx<$Uart> = Rx { _uart: PhantomData };
                rx.read()
            }
        }

        impl embedded_hal_0_2::serial::Read<u8> for Rx<$Uart> {
            type Error = ();

            fn read(&mut self) -> nb::Result<u8, Self::Error> {
                let data_avail = unsafe { (*$Uart::ptr()).sta.read().urxda().bit() };
                let result = if !data_avail {
                    Err(nb::Error::WouldBlock)
                } else {
                    unsafe { Ok((*$Uart::ptr()).rxreg.read().rxreg().bits() as u8) }
                };
                let overrun = unsafe { (*$Uart::ptr()).sta.read().oerr().bit() };
                if overrun {
                    unsafe { (*$Uart::ptr()).staclr.write(|w| w.oerr().bit(true)) }
                }
                result
            }
        }

        impl embedded_io::ErrorType for Tx<$Uart> {
            type Error = Infallible;
        }

        impl embedded_io::WriteReady for Tx<$Uart> {
            fn write_ready(&mut self) -> Result<bool, Self::Error> {
                let utxbf = unsafe { (*$Uart::ptr()).sta.read().utxbf().bit() };
                Ok(!utxbf)
            }
        }

        impl embedded_io::Write for Tx<$Uart> {
            fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
                while !self.write_ready()? {}
                let mut len = 0;
                for byte in buf {
                    if !self.write_ready()? {
                        break;
                    }
                    unsafe {
                        (*$Uart::ptr())
                            .txreg
                            .write(|w| w.txreg().bits(*byte as u16));
                    }
                    len += 1;
                }
                Ok(len)
            }

            fn flush(&mut self) -> Result<(), Self::Error> {
                loop {
                    let trmt = unsafe { (*$Uart::ptr()).sta.read().trmt().bit() };
                    if !trmt {
                        break;
                    }
                }
                Ok(())
            }
        }

        impl embedded_io::ErrorType for Rx<$Uart> {
            type Error = ReadError;
        }

        impl embedded_io::ReadReady for Rx<$Uart> {
            fn read_ready(&mut self) -> Result<bool, Self::Error> {
                let data_avail = unsafe { (*$Uart::ptr()).sta.read().urxda().bit() };
                Ok(data_avail)
            }
        }

        impl embedded_io::Read for Rx<$Uart> {
            fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
                if buf.is_empty() {
                    return Ok(0);
                }

                // check for FIFO overrun
                if unsafe { (*$Uart::ptr()).sta.read().oerr().bit() } {
                    // clear overrun error condition and RX FIFO
                    unsafe {
                        (*$Uart::ptr()).staclr.write(|w| w.oerr().bit(true));
                    }
                    return Err(ReadError::Overrun);
                }

                // wait until data is available
                while !self.read_ready()? {}

                // read as many bytes as possible without blocking
                let mut len = 0;
                for bufbyte in buf {
                    if !self.read_ready()? {
                        break;
                    }
                    // check for framing error or break skipping the erroneous byte
                    if unsafe { (*$Uart::ptr()).sta.read().ferr().bit() } {
                        let word = unsafe { (*$Uart::ptr()).rxreg.read().bits() };
                        if word == 0 {
                            return Err(ReadError::Break);
                        } else {
                            return Err(ReadError::Framing);
                        }
                    }
                    // check for parity error skipping the erroneous byte
                    if unsafe { (*$Uart::ptr()).sta.read().perr().bit() } {
                        let _skip = unsafe { (*$Uart::ptr()).rxreg.read().bits() };
                        return Err(ReadError::Parity);
                    }
                    *bufbyte = unsafe { (*$Uart::ptr()).rxreg.read().bits() } as u8;
                    len += 1;
                }
                Ok(len)
            }
        }

        impl<RX, TX> embedded_io::ErrorType for Uart<$Uart, RX, TX> {
            type Error = ReadError;
        }

        impl<RX, TX> embedded_io::WriteReady for Uart<$Uart, RX, TX> {
            fn write_ready(&mut self) -> Result<bool, Self::Error> {
                let mut tx: Tx<$Uart> = Tx { _uart: PhantomData };
                Ok(tx.write_ready().unwrap_or(false))
            }
        }

        impl<RX, TX> embedded_io::Write for Uart<$Uart, RX, TX> {
            fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
                let mut tx: Tx<$Uart> = Tx { _uart: PhantomData };
                Ok(embedded_io::Write::write(&mut tx, buf).unwrap_or(0))
            }

            fn flush(&mut self) -> Result<(), Self::Error> {
                let mut tx: Tx<$Uart> = Tx { _uart: PhantomData };
                let _ = embedded_io::Write::flush(&mut tx);
                Ok(())
            }
        }

        impl<RX, TX> embedded_io::ReadReady for Uart<$Uart, RX, TX> {
            fn read_ready(&mut self) -> Result<bool, Self::Error> {
                let mut rx: Rx<$Uart> = Rx { _uart: PhantomData };
                rx.read_ready()
            }
        }

        impl<RX, TX> embedded_io::Read for Uart<$Uart, RX, TX> {
            fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
                let mut rx: Rx<$Uart> = Rx { _uart: PhantomData };
                embedded_io::Read::read(&mut rx, buf)
            }
        }
    };
}

uart_impl!(uart1, UART1, input::U1rx, output::U1tx);
uart_impl!(uart2, UART2, input::U2rx, output::U2tx);

impl<UART> fmt::Write for Tx<UART>
where
    Tx<UART>: embedded_hal_0_2::serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = s.as_bytes().iter().map(|c| block!(self.write(*c))).last();
        Ok(())
    }
}
