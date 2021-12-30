# pic32-hal

![Crates.io](https://img.shields.io/crates/v/pic32-hal)
![docs.rs](https://img.shields.io/docsrs/pic32-hal)

A hardware abstraction layer for some PIC32 microcontrollers.

This crate provides a thin low-level API on top of the register access API implemented by the PAC crate. The following HAL functionality is available

* basic clock control, including reference clock generator
* GPIO
* UART
* USB
* access to the MIPS core timer
* DMA channels
* I2C peripheral
* SPI peripheral
* interrupt controller
* Peripheral Pin Select (PPS)

Some of the modules implement the [embedded-hal](https://crates.io/crates/embedded-hal) API so that device drivers (e. g. for displays) using this API can access PIC32 peripherals.

Everything here is work in progress. Examples can be found in the [git repository](https://github.com/kiffie/pic32-rs/tree/master/examples).
