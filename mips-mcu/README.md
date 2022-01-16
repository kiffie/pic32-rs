# mips_mcu

[![Crates.io](https://img.shields.io/crates/v/mips-mcu.svg)](https://crates.io/crates/mips_mcu)
[![docs.rs](https://img.shields.io/docsrs/mips-mcu.svg)](https://docs.rs/mips_mcu)

Low level access to MIPS MCU cores

This crate includes Rust function to deal with low level aspects related to the
MIPS MCU cores (e.g. the M4K core). Routines requiring special or privileged
instructions are included in a binary library, thereby avoiding inline assembly.
