//! Support for the Fixed Mapping Translation (FMT) of the MIPS M4K core

use crate::PhysicalAddress;

/// Convert a virtual to a physical address given as `usize` values
#[inline(never)]
fn virt_to_phys_usize(virt: usize) -> usize {
    if virt >= 0x80000000usize {
        virt & 0x1fff_ffff
    } else {
        virt + 0x4000_0000
    }
}

/// Calculate a physical address for a raw pointer
pub fn virt_to_phys<T>(ptr: *mut T) -> PhysicalAddress {
    let virt: usize = ptr as usize;
    PhysicalAddress {
        addr: virt_to_phys_usize(virt),
    }
}
