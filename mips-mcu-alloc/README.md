# mips-mcu-alloc

[![Crates.io](https://img.shields.io/crates/v/mips-mcu-alloc.svg)](https://crates.io/crates/mips-mcu-alloc)
[![docs.rs](https://img.shields.io/docsrs/mips-mcu-alloc.svg)](https://docs.rs/mips-mcu-alloc)

A heap allocator for PIC32 microcontrollers (based on the `alloc-cortex-m` crate)

The heap is placed at a location determined by the linker and automatically extended
to fullfil allocation requests. Automatic heap extension fails if the heap would collide
with the stack.

Memory allocation and heap extension can be traced via logging by activating the `log` feature.

Example:

```rust
#![feature(global_allocator)]
#![feature(alloc_error_handler)]

// Plug in the allocator crate
extern crate alloc;
use alloc::Vec;
use mips_mcu_alloc::MipsMcuHeap;

#[global_allocator]
static ALLOCATOR: MipsMcuHeap = MipsMcuHeap::empty();

#[entry]
fn main() -> ! {
    ALLOCATOR.init();
    let mut xs = Vec::new();
    xs.push(1);
    loop { /* .. */ }
}

#[alloc_error_handler]
fn alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Cannot allocate heap memory: {:?}", layout);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
