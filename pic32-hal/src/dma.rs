//! DMA Controller

use crate::int::InterruptSource;
use crate::pac::{DMAC, DMAC0, DMAC1, DMAC2, DMAC3};
use enumflags2::BitFlags;
use mips_rt::PhysicalAddress;

/// Interrupt flag or enable bits related to a specific DMA channel
#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
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

/// DMA Operations
///
/// This trait defines operations that can be carried out by a DMAChannel
pub trait Ops {
    fn set_source(&self, addr: PhysicalAddress, size: usize);

    fn set_dest(&self, addr: PhysicalAddress, size: usize);

    fn set_cell_size(&self, size: usize);

    fn set_start_event(&self, event: Option<InterruptSource>);

    fn set_abort_event(&self, event: Option<InterruptSource>);

    fn set_abort_pattern(&self, pattern: Option<u8>);

    fn irq_enable(&self, irq: BitFlags<DmaIrq>);

    /// Enable DMA channel
    ///
    /// # Safety
    ///
    /// Unsafe because the DMA controller will access the memory blocks
    /// specified as source and destination without any checks
    unsafe fn enable(&self, mode: XferMode);

    /// Check if a DMA channel is enabled
    ///
    /// Can be used to poll OneShot channels for completion.
    fn is_enabled(&self) -> bool;

    /// Disable a DMA channel
    fn disable(&self);

    /// Force a cell transfer
    fn force(&self);
}

pub struct DmaChannel<D> {
    ch: D,
}

macro_rules! dma {
    ($Id:ident, $Dmac:ident) => {
        impl Ops for DmaChannel<$Dmac> {
            fn set_source(&self, addr: PhysicalAddress, size: usize) {
                unsafe {
                    self.ch.ssa.write(|w| w.bits(addr.address() as u32));
                    self.ch.ssiz.write(|w| w.bits(size as u32));
                }
            }

            fn set_dest(&self, addr: PhysicalAddress, size: usize) {
                unsafe {
                    self.ch.dsa.write(|w| w.bits(addr.address() as u32));
                    self.ch.dsiz.write(|w| w.bits(size as u32));
                }
            }

            fn set_cell_size(&self, size: usize) {
                unsafe {
                    self.ch.csiz.write(|w| w.bits(size as u32));
                }
            }

            fn set_start_event(&self, event: Option<InterruptSource>) {
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

            fn set_abort_event(&self, event: Option<InterruptSource>) {
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

            fn set_abort_pattern(&self, pattern: Option<u8>) {
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

            fn irq_enable(&self, irq: BitFlags<DmaIrq>) {
                self.ch
                    .int
                    .write(|w| unsafe { w.bits((irq.bits() as u32) << 16) });
            }

            unsafe fn enable(&self, mode: XferMode) {
                match mode {
                    XferMode::OneShot => self.ch.contclr.write(|w| w.chaen().bit(true)),
                    XferMode::Auto => self.ch.contset.write(|w| w.chaen().bit(true)),
                }
                self.ch.contset.write(|w| w.chen().bit(true));
            }

            fn is_enabled(&self) -> bool {
                self.ch.cont.read().chen().bit()
            }

            fn disable(&self) {
                self.ch.contclr.write(|w| w.chen().bit(true));
                while self.ch.cont.read().chbusy().bit() {}
            }

            fn force(&self) {
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

/// indicates whether the channel shall be automatically enabled after a block
/// transfer
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum XferMode {
    OneShot,
    Auto,
}
