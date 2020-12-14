//! UART driver

use core::marker::PhantomData;
use crate::clock::Osc;

use crate::pac::{UART1, UART2};

/// Uart
pub struct Uart<UART> {
    _uart: PhantomData<UART>,
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
    ($Id:ident, $Uart:ident) => {
        impl Uart<$Uart> {
            pub fn $Id(uart: $Uart, osc: &Osc, baudrate: u32) -> Uart<$Uart> {
                let brg = osc.pb_clock().0 / (4 * baudrate) - 1;

                unsafe {
                    uart.mode.write(|w| w.bits(0));
                    uart.mode.write(|w| w.brgh().bit(true));
                    uart.sta
                        .write(|w| w.urxen().bit(true).utxen().bit(true).urxisel().bits(0b10));
                    uart.brg.write(|w| w.bits(brg));
                    uart.modeset.write(|w| w.on().bit(true));
                }
                Uart { _uart: PhantomData }
            }

            pub fn split(self) -> (Tx<$Uart>, Rx<$Uart>) {
                (
                    Tx {
                        _uart: PhantomData,
                    },
                    Rx {
                        _uart: PhantomData,
                    },
                )
            }
        }

        impl embedded_hal::serial::Write<u8> for Uart<$Uart> {
            type Error = ();

            fn flush(&mut self) -> nb::Result<(), Self::Error> {
                let mut tx: Tx<$Uart> = Tx {
                    _uart: PhantomData,
                };
                tx.flush()
            }

            fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
                let mut tx: Tx<$Uart> = Tx {
                    _uart: PhantomData,
                };
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

        impl embedded_hal::serial::Read<u8> for Uart<$Uart> {
            type Error = ();

            fn read(&mut self) -> nb::Result<u8, Self::Error> {
                let mut rx: Rx<$Uart> = Rx {
                    _uart: PhantomData,
                };
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

uart_impl!(uart1, UART1);
uart_impl!(uart2, UART2);
