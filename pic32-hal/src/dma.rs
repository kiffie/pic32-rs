//! DMA Controller

use crate::int::InterruptSource;
use crate::pac::{DMAC, DMAC0, DMAC1, DMAC2, DMAC3};
use enumflags2::{bitflags, BitFlags};
use mips_mcu::PhysicalAddress;

/// Interrupt flag or enable bits related to a specific DMA channel
#[bitflags]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum DmaIrq {
    /// Channel Source Done
    CHSD = 0x80,
    /// Channel Source Half Empty
    CHSH = 0x40,
    /// Channel Destination Done
    CHDD = 0x20,
    /// Channel Destination Half Full
    CHDH = 0x10,
    /// Channel Block Transfer Complete
    CHBC = 0x08,
    /// Channel Cell Transfer Complete
    CHCC = 0x04,
    /// Channel Transfer Abort
    CHTA = 0x02,
    ///  Channel Address Error
    CHER = 0x01,
}

/// indicates whether the channel shall be automatically enabled after a block
/// transfer
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum XferMode {
    OneShot,
    Auto,
}

/// DMA Operations
///
/// This trait defines operations that can be carried out by a DMAChannel
pub trait Ops {
    /// Set source address and size of source block in bytes
    fn set_source(&mut self, addr: PhysicalAddress, size: usize);

    /// Set destination address and size of destination block in bytes
    fn set_dest(&mut self, addr: PhysicalAddress, size: usize);

    /// Set cell size, i.e. number of bytes transferred per triggering event
    fn set_cell_size(&mut self, size: usize);

    /// Set start event source for triggering a cell transfer
    fn set_start_event(&mut self, event: Option<InterruptSource>);

    /// Set event for aborting a transfer
    fn set_abort_event(&mut self, event: Option<InterruptSource>);

    /// Set data pattern that aborts a transfer
    fn set_abort_pattern(&mut self, pattern: Option<u8>);

    /// Get source pointer
    fn source_pointer(&self) -> PhysicalAddress;

    /// Get destination pointer
    fn destination_pointer(&self) -> PhysicalAddress;

    /// Enable/disable individual interrupt sources
    ///
    /// This function does not configure the interrupt controller.
    fn irq_enable(&mut self, irq: BitFlags<DmaIrq>);

    /// Get interrupt flags
    fn irq_flags(&self) -> BitFlags<DmaIrq>;

    /// Set interrupt flags
    fn set_irq_flags(&mut self, flags: BitFlags<DmaIrq>);

    /// Clear all interrupt flags
    fn clear_all_irq_flags(&mut self) {
        self.set_irq_flags(BitFlags::<DmaIrq>::default());
    }

    /// Enable DMA channel
    ///
    /// # Safety
    ///
    /// Unsafe because the DMA controller will access the memory blocks
    /// specified as source and destination without any checks
    unsafe fn enable(&mut self, mode: XferMode);

    /// Check if a DMA channel is enabled
    ///
    /// Can be used to poll OneShot channels for completion.
    fn is_enabled(&self) -> bool;

    /// Disable a DMA channel
    fn disable(&mut self);

    /// Force a cell transfer
    fn force(&mut self);
}

pub struct DmaChannel<D> {
    ch: D,
}

macro_rules! dma {
    ($Id:ident, $Dmac:ident) => {
        impl Ops for DmaChannel<$Dmac> {
            fn set_source(&mut self, addr: PhysicalAddress, size: usize) {
                unsafe {
                    self.ch.ssa.write(|w| w.bits(addr.address() as u32));
                    self.ch.ssiz.write(|w| w.bits(size as u32));
                }
            }

            fn set_dest(&mut self, addr: PhysicalAddress, size: usize) {
                unsafe {
                    self.ch.dsa.write(|w| w.bits(addr.address() as u32));
                    self.ch.dsiz.write(|w| w.bits(size as u32));
                }
            }

            fn set_cell_size(&mut self, size: usize) {
                unsafe {
                    self.ch.csiz.write(|w| w.bits(size as u32));
                }
            }

            fn set_start_event(&mut self, event: Option<InterruptSource>) {
                match event {
                    Some(e) => {
                        self.ch
                            .econ
                            .modify(|_, w| unsafe { w.chsirq().bits(e as u8).sirqen().bit(true) });
                    }
                    None => {
                        self.ch.econclr.write(|w| w.sirqen().bit(true));
                    }
                }
            }

            fn set_abort_event(&mut self, event: Option<InterruptSource>) {
                match event {
                    Some(e) => {
                        self.ch
                            .econ
                            .modify(|_, w| unsafe { w.chairq().bits(e as u8).airqen().bit(true) });
                    }
                    None => {
                        self.ch.econclr.write(|w| w.airqen().bit(true));
                    }
                }
            }

            fn set_abort_pattern(&mut self, pattern: Option<u8>) {
                match pattern {
                    Some(p) => {
                        self.ch.dat.write(|w| unsafe { w.dchpdat().bits(p) });
                        self.ch.econset.write(|w| w.paten().bit(true));
                    }
                    None => {
                        self.ch.econclr.write(|w| w.paten().bit(true));
                    }
                }
            }

            fn source_pointer(&self) -> PhysicalAddress {
                PhysicalAddress::from_usize(self.ch.sptr.read().bits() as usize)
            }

            fn destination_pointer(&self) -> PhysicalAddress {
                PhysicalAddress::from_usize(self.ch.dptr.read().bits() as usize)
            }

            fn irq_enable(&mut self, irq: BitFlags<DmaIrq>) {
                self.ch
                    .int
                    .write(|w| unsafe { w.bits((irq.bits() as u32) << 16) });
            }

            fn irq_flags(&self) -> BitFlags<DmaIrq> {
                let int = self.ch.int.read().bits();
                unsafe { BitFlags::from_bits_unchecked(int as u8) }
            }

            fn set_irq_flags(&mut self, flags: BitFlags<DmaIrq>) {
                self.ch.int.modify(|r, w| unsafe {
                    w.bits((r.bits() & 0xffff_ff00) | flags.bits() as u32)
                });
            }

            unsafe fn enable(&mut self, mode: XferMode) {
                match mode {
                    XferMode::OneShot => self.ch.contclr.write(|w| w.chaen().bit(true)),
                    XferMode::Auto => self.ch.contset.write(|w| w.chaen().bit(true)),
                }
                self.ch.contset.write(|w| w.chen().bit(true));
            }

            fn is_enabled(&self) -> bool {
                self.ch.cont.read().chen().bit()
            }

            fn disable(&mut self) {
                self.ch.contclr.write(|w| w.chen().bit(true));
                while self.ch.cont.read().chbusy().bit() {}
            }

            fn force(&mut self) {
                self.ch.econset.write(|w| w.cforce().bit(true));
            }
        }

        impl DmaChannel<$Dmac> {
            pub fn $Id(ch: $Dmac) -> Self {
                // make sure that the DMA controller is enabled
                unsafe {
                    (*DMAC::ptr()).dmaconset.write(|w| w.on().bit(true));
                }
                DmaChannel { ch }
            }
        }
    };
}

dma!(channel0, DMAC0);
dma!(channel1, DMAC1);
dma!(channel2, DMAC2);
dma!(channel3, DMAC3);
