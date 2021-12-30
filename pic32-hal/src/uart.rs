//! UART driver

use core::fmt;
use core::marker::PhantomData;

use crate::clock::Osc;
use crate::pps::{input, output, MappedPin, IsConnected};
use crate::pac::{UART1, UART2};

use embedded_hal::prelude::*;
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
                    rx: rx,
                    tx: tx,
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

        impl embedded_hal::serial::Write<u8> for Uart<$Uart, $Rx, $Tx> {
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

        impl embedded_hal::serial::Write<u8> for Tx<$Uart> {
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

        impl embedded_hal::serial::Read<u8> for Uart<$Uart, $Rx, $Tx> {
            type Error = ();

            fn read(&mut self) -> nb::Result<u8, Self::Error> {
                let mut rx: Rx<$Uart> = Rx { _uart: PhantomData };
                rx.read()
            }
        }

        impl embedded_hal::serial::Read<u8> for Rx<$Uart> {
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
    };
}

uart_impl!(uart1, UART1, input::U1rx, output::U1tx);
uart_impl!(uart2, UART2, input::U2rx, output::U2tx);

impl<UART> fmt::Write for Tx<UART>
where
    Tx<UART>: embedded_hal::serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = s.as_bytes().iter().map(|c| block!(self.write(*c))).last();
        Ok(())
    }
}
