//! A heap allocator for microcontollers with MIPS core such as PIC32 controllers
//!
//! The heap is placed at a location determined by the linker and automatically extended
//! to fullfil allocation requests. Automatic heap extension fails if the heap would collide
//! with the stack.
//!
//! # Example
//!
//! ```
//! #![feature(global_allocator)]
//! #![feature(alloc_error_handler)]
//!
//! // Plug in the allocator crate
//! extern crate alloc;
//!
//! use alloc::Vec;
//! use mips_mcu_alloc::MipsMcuHeap;
//!
//! #[global_allocator]
//! static ALLOCATOR: MipsMcuHeap = MipsMcuHeap::empty();
//!
//! entry!(main);
//!
//! #[entry]
//! fn main() -> ! {
//!     ALLOCATOR.init();
//!
//!     let mut xs = Vec::new();
//!     xs.push(1);
//!
//!     loop { /* .. */ }
//! }
//!
//! #[alloc_error_handler]
//! fn alloc_error(layout: core::alloc::Layout) -> ! {
//!     panic!("Cannot allocate heap memory: {:?}", layout);
//! }
//!
//! ```

#![no_std]
#![feature(asm_experimental_arch)]

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::cell::RefCell;
use core::ptr::{self, NonNull};

use linked_list_allocator::Heap;
use mips_rt::heap_start;
use critical_section::{self, Mutex};

/// Heap extension is performed stepwise. This constant defines the size of one extension step.
const EXTEND_INCREMENT: usize = 1024;

pub struct MipsMcuHeap {
    heap: Mutex<RefCell<Heap>>,
}

impl MipsMcuHeap {
    /// Crate a new UNINITIALIZED heap allocator
    ///
    /// You must initialize this heap using the
    /// [`init`](struct.CortexMHeap.html#method.init) method before using the allocator.
    pub const fn empty() -> MipsMcuHeap {
        MipsMcuHeap {
            heap: Mutex::new(RefCell::new(Heap::empty())),
        }
    }

    /// Initialize heap with heap start location from linker and a defined initial size
    pub fn init(&self) {
        let bottom = heap_start() as *mut u8;
        critical_section::with(|cs| {
            unsafe {
                self.heap
                    .borrow(cs)
                    .borrow_mut()
                    .init(bottom, EXTEND_INCREMENT)
            };
        });
    }

    /// Returns an estimate of the amount of bytes in use.
    pub fn used(&self) -> usize {
        critical_section::with(|cs| self.heap.borrow(cs).borrow_mut().used())
    }

    /// Returns an estimate of the amount of bytes available.
    pub fn free(&self) -> usize {
        critical_section::with(|cs| self.heap.borrow(cs).borrow_mut().free())
    }

    /// Returns the start (bottom) of the heap
    pub fn bottom(&self) -> *mut u8 {
        critical_section::with(|cs| self.heap.borrow(cs).borrow_mut().bottom())
    }

    /// Returns the end (top) of the heap
    pub fn top(&self) -> *mut u8 {
        critical_section::with(|cs| self.heap.borrow(cs).borrow_mut().top())
    }
}

unsafe impl GlobalAlloc for MipsMcuHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // try to allocate and successively extend by EXTEND_INCREMENT until memory is exhausted
        loop {
            if let Ok(p) = critical_section::with(|cs| {
                self.heap
                    .borrow(cs)
                    .borrow_mut()
                    .allocate_first_fit(layout)
            }) {
                break p.as_ptr();
            } else {
                // this must be a u8 pointer
                let new_top: *mut u8 =
                    critical_section::with(|cs| self.heap.borrow(cs).borrow_mut().top())
                        .add(EXTEND_INCREMENT);
                // avoid collision with stack
                if new_top < stack_pointer() {
                    critical_section::with(|cs| {
                        self.heap.borrow(cs).borrow_mut().extend(EXTEND_INCREMENT)
                    });
                } else {
                    break ptr::null_mut();
                }
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        critical_section::with(|cs| {
            self.heap
                .borrow(cs)
                .borrow_mut()
                .deallocate(NonNull::new_unchecked(ptr), layout)
        });
    }
}

fn stack_pointer() -> *mut u8 {
    let sp: *mut u8;
    unsafe {
        asm!(".set noat",
             "move {0}, $29", out(reg) sp);
    }
    sp
}
