# pic32-rs
Rust crates for PIC32 programming including PIC32 HAL modules

This repository contains code to program PIC32MX microcontrollers with Rust. It
uses the `mipsel-unknown-linux-gnu` target, which is intended for use with
MIPS-based application processors running under Linux. The cores of most PIC32
MCUs support the classical 32 bit wide MIPS ISA. Therefore, this target can also
be used to program these microcontrollers when customized appropriately.

The repository contains the following

* mips-rt: Basic Rust runtime and startup files for MIPS based microcontrollers
* pic32-hal: HAL crate for PIC32 microcontrollers. There are currently HAL modules
for the MIPS core timer, GPIO, interrupt controller, SPI, UART, I2C and USB.
* example applications

Moreover, there are peripheral access crates (PACs) under the repository `pic32-pac`.
There is also a repository `alloc-pic32` to support dynamic memory allocation.

## Compiling

To set up the toolchain, the following commands may be used.

```
rustup default nightly
rustup target install mipsel-unknown-linux-gnu
cargo install cargo-binutils
```

`cargo-binutils` includes `cargo-objcopy` that can be used to generate Intel HEX
files. When other tools are used to generate HEX files or if the programmer can
deal with ELF files then cargo-binutils is not needed.

This code can be compiled with the nighly toolchain using cargo. A JSON file must
be provided to customize the target (mainly to define the ISA revision and to use
FPU emulation for the smaller cores like the MIPS M4K).

See also the _blinky_ example on how to compile a PIC32 application.

A target specification JSON file for the M4K core used in PIC32MX devices looks
like this.

```
{
  "arch": "mips",
  "cpu": "mips32r2",
  "data-layout": "e-m:m-p:32:32-i8:8:32-i16:16:32-i64:64-n32-S64",
  "emit-debug-gdb-scripts": false,
  "env": "",
  "executables": true,
  "features": "+mips32r2,+soft-float,+noabicalls",
  "has-elf-tls": false,
  "has-rpath": true,
  "linker": "rust-lld",
  "linker-flavor": "ld.lld",
  "llvm-target": "mipsel-unknown-linux-gnu",
  "max-atomic-width": 32,
  "os": "none",
  "position-independent-executables": false,
  "panic-strategy": "abort",
  "relocation-model": "static",
  "target-c-int-width": "32",
  "target-endian": "little",
  "target-pointer-width": "32",
  "vendor": ""
}
```

A `.cargo/config` file is needed to specify the linker script (e.g.
`32MX150F128B_procdefs.ld`), to refer to the JSON target definition and to build
standard library crates.

```
[target.mipsel-none]
rustflags = ["-C", "link-arg=-T32MX150F128B_procdefs.ld",
             "-C", "relocation-model=static"]

[build]
target = "mipsel-none.json"

[unstable]
build-std = ["core", "compiler_builtins", "alloc"]
```

Using the above files, the build can be done with cargo. To save code space, a 
release build may make sense.

```
cargo build --release
```

To create a Intel hex file, `cargo-objcopy` can be used.

```
cargo objcopy --release -- -O ihex somefilename.hex
```

## Details on Linking

To link the final application image, three linker script files are used

* a main file containing the memory map of the used device (needs to be adapted
to the Flash memory size and the RAM size
* a file `pic32_common.ld` included by the main file
* a file `device.x` contained in the peripheral access crate that provides
symbolic names for the interrupt vectors; also included by the main file
