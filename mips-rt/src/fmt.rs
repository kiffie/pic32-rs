//! Support for the Fixed Mapping Translation (FMT) of the MIPS M4K core

use crate::PhysicalAddress;

/// Calculate a physical address
#[inline(never)]
pub fn virt_to_phys<T>(ptr: *mut T) -> PhysicalAddress {
    let virt: usize = ptr as usize;
    if virt >= 0x80000000usize {
        PhysicalAddress { addr: virt & 0x1fff_ffff }
    } else {
        PhysicalAddress { addr: virt + 0x4000_0000 }
    }
}
