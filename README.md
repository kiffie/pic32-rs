# pic32-rs

Rust crates for PIC32 programming including PIC32 HAL modules

This repository contains code to program PIC32MX microcontrollers with Rust. It
uses the `mipsel-unknown-none` target, which is intended for use with MIPS MCU ("bare metal") targets and is used to generate code for the classical MIPS32r2 ISA having 32 bit wide instructions.

The repository contains the following

* mips-rt: Basic Rust runtime and startup files for MIPS based microcontrollers
* pic32-hal: HAL crate for PIC32 microcontrollers. There are currently HAL modules
for the MIPS core timer, GPIO, interrupt controller, SPI, UART, I2C and USB.
* example applications

Moreover, there are peripheral access crates (PACs) under the repository `pic32-pac`.
There is also a repository `alloc-pic32` to support dynamic memory allocation.

## Compiling

To set up the toolchain, the following commands may be used.

```sh
rustup default nightly
rustup component add rust-src
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

[cargo-binutils](https://github.com/rust-embedded/cargo-binutils) includes
`cargo-objcopy` that can be used to generate Intel HEX
files. When other tools are used to generate HEX files or if your Flash memory programmer can
deal with ELF files then cargo-binutils is not needed.

This code can be compiled with the nightly toolchain using cargo.

See also the _blinky_ example on how to compile a PIC32 application.

A `.cargo/config` file is needed to specify the linker script (e.g.
`32MX150F128B_procdefs.ld`), to specify the target and to build
standard library crates. Below see an example `.cargo/config` file.

```toml
[target.mipsel-unknown-none]
rustflags = ["-C", "link-arg=-T32MX150F128B_procdefs.ld"]

[build]
target = "mipsel-unknown-none"

[unstable]
build-std = ["core", "compiler_builtins", "alloc"]
```

Using the above files, the build can be done with cargo. To save code space, a
release build may make sense.

```sh
cargo build --release
```

To create a Intel hex file, `cargo-objcopy` can be used.

```sh
cargo objcopy --release -- -O ihex somefilename.hex
```

## Details on Linking

To link the final application image, three linker script files are used

* a main file containing the memory map of the used device (needs to be adapted
to the Flash memory size and the RAM size
* a file `pic32_common.ld` included by the main file
* a file `device.x` contained in the peripheral access crate that provides
symbolic names for the interrupt vectors; also included by the main file
