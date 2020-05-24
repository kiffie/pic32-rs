//! DMA Controller

use crate::int::InterruptSource;
use crate::pac::{DMAC, DMAC0, DMAC1, DMAC2, DMAC3};
use enumflags2::BitFlags;
use mips_rt::PhysicalAddress;

pub struct DmaChannelBuilder {
    start_event: Option<InterruptSource>,
    abort_event: Option<InterruptSource>,
    abort_pattern: Option<u8>,
    irq_ena: BitFlags<DmaIrq>, // IRQ Enable flags to be written into DCHxINT register
}

impl DmaChannelBuilder {
    pub fn new() -> DmaChannelBuilder {
        DmaChannelBuilder {
            start_event: None,
            abort_event: None,
            abort_pattern: None,
            irq_ena: BitFlags::empty(),
        }
    }

    pub fn start_event(mut self, event: InterruptSource) -> DmaChannelBuilder {
        self.start_event = Some(event);
        self
    }

    pub fn abort_event(mut self, event: InterruptSource) -> DmaChannelBuilder {
        self.abort_event = Some(event);
        self
    }

    pub fn abort_pattern(mut self, pattern: u8) -> DmaChannelBuilder {
        self.abort_pattern = Some(pattern);
        self
    }

    pub fn irq_enable(mut self, irq: BitFlags<DmaIrq>) -> DmaChannelBuilder {
        self.irq_ena = irq;
        self
    }
}

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

pub struct DmaChannel<D> {
    ch: D,
}

macro_rules! dma {
    ($Id:ident, $Dmac:ident) => {
        impl DmaChannelBuilder {
            pub fn $Id(self, ch: $Dmac) -> DmaChannel<$Dmac> {
                // make sure that the DMA controller is enabled
                unsafe {
                    (*DMAC::ptr()).dmaconset.write(|w| w.on().bit(true));
                }

                if let Some(event) = self.start_event {
                    ch.econ
                        .modify(|_, w| unsafe { w.chsirq().bits(event as u8).sirqen().bit(true) });
                }
                if let Some(event) = self.abort_event {
                    ch.econ
                        .modify(|_, w| unsafe { w.chairq().bits(event as u8).airqen().bit(true) });
                }
                if let Some(pattern) = self.abort_pattern {
                    ch.dat.write(|w| unsafe { w.dchpdat().bits(pattern) });
                    ch.econset.write(|w| w.paten().bit(true));
                }
                ch.int
                    .write(|w| unsafe { w.bits((self.irq_ena.bits() as u32) << 16) });
                //ch.con.write(|w| w.chen().bit(false));

                DmaChannel { ch }
            }
        }

        impl DmaChannel<$Dmac> {
            /// Enable DMA channel using raw PhysicalAddress and usize values
            ///
            /// Unsafe because the DMA controller will access the memory blocks
            /// specified as source and destination without any checks
            pub unsafe fn enable_raw(
                &self,
                mode: XferMode,
                source: PhysicalAddress,
                source_size: usize,
                destination: PhysicalAddress,
                destination_size: usize,
                cell_size: usize,
            ) {
                self.ch.ssa.write(|w| w.bits(source.address() as u32));
                self.ch.dsa.write(|w| w.bits(destination.address() as u32));
                self.ch.ssiz.write(|w| w.bits(source_size as u32));
                self.ch.dsiz.write(|w| w.bits(destination_size as u32));
                self.ch.csiz.write(|w| w.bits(cell_size as u32));
                match mode {
                    XferMode::OneShot => self.ch.conclr.write(|w| w.chaen().bit(true)),
                    XferMode::Auto => self.ch.conset.write(|w| w.chaen().bit(true)),
                }
                self.ch.conset.write(|w| w.chen().bit(true));
            }
        }
    };
}

dma!(build_channel0, DMAC0);
dma!(build_channel1, DMAC1);
dma!(build_channel2, DMAC2);
dma!(build_channel3, DMAC3);

/// indicates whether the channel shall be automatically enabled after a block
/// transfer
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum XferMode {
    OneShot,
    Auto,
}
