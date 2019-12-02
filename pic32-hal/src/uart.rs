/// UART driver for PIC32

use core::fmt::Write;
use vcell::VolatileCell;
use nb;
use crate::clock;

#[repr(C)]
struct RegisterBlock {
    mode:       VolatileCell<u32>,
    modeclr:    VolatileCell<u32>,
    modeset:    VolatileCell<u32>,
    modeinv:    VolatileCell<u32>,
    sta:        VolatileCell<u32>,
    staclr:     VolatileCell<u32>,
    staset:     VolatileCell<u32>,
    stainv:     VolatileCell<u32>,
    txreg:      VolatileCell<u32>,
    _reserved0: [u8; 12usize],
    rxreg:      VolatileCell<u32>,      // offset: 0x30
    _reserved1: [u8; 12usize],
    brg:        VolatileCell<u32>,      // offset: 0x40
    brgclr:     VolatileCell<u32>,
    brgset:     VolatileCell<u32>,
    brginv:     VolatileCell<u32>,
}


const MODE_BRGH_MASK: u32       = 0x00000008;
const MODE_ON_MASK: u32         = 0x00008000;

const STA_URXEN_MASK: u32       = 0x00001000;
const STA_UTXEN_MASK: u32       = 0x00000400;
const STA_UTXBF_MASK: u32       = 0x00000200;
const STA_TRMT_MASK: u32        = 0x00000100;
const STA_URXISEL_POSITION: u32 = 0x00000006;
const STA_URXDA_MASK :u32       = 0x00000001;


// The values of this enum correspond to the base address of the UARTS
#[allow(dead_code)]
#[repr(usize)]
pub enum HwModule {
    UART1 = 0xbf80_6000,
    UART2 = 0xbf80_6200,
    UART3 = 0xbf80_6400,
    UART4 = 0xbf80_6600,
    UART5 = 0xbf80_6800,
}

pub struct Uart{
    reg_ptr: *const RegisterBlock,
}

pub struct Rx {
    reg_ptr: *const RegisterBlock,
}
unsafe impl core::marker::Send for Rx { }

pub struct Tx {
    reg_ptr: *const RegisterBlock,
}
unsafe impl core::marker::Send for Tx { }
unsafe impl core::marker::Sync for Tx { }


impl Uart {

    pub const fn new(uart: HwModule) -> Uart {
        // calculate base address of UART
        let uart_addr = uart as usize;
        let reg_ptr = uart_addr as *const RegisterBlock;
        Uart {
            reg_ptr: reg_ptr,
        }
    }

    pub fn init(&self, baud_rate: u32){
        let regs : &RegisterBlock = unsafe { &*self.reg_ptr };
        regs.mode.set(0);
        let brg = clock::pb_clock()/(4*baud_rate)-1;
        regs.mode.set(MODE_BRGH_MASK);
        regs.sta.set(STA_URXEN_MASK |
                     STA_UTXEN_MASK |
                     (0b10 << STA_URXISEL_POSITION));
        regs.brg.set(brg);
        regs.modeset.set(MODE_ON_MASK);
    }

    pub fn split(self) -> (Tx, Rx) {
        (Tx { reg_ptr: self.reg_ptr}, Rx { reg_ptr: self.reg_ptr } )
    }
}

impl Tx {
    pub fn try_write_byte(&self, byte: u8) -> bool {

        let regs : &RegisterBlock = unsafe { &*self.reg_ptr };
        let utxbf = (regs.sta.get() & STA_UTXBF_MASK) != 0;
        if utxbf {
            false
        }else{
            regs.txreg.set(byte as u32);
            true
        }
    }
}

impl Drop for Tx {
    fn drop(&mut self){
        let regs : &RegisterBlock = unsafe { &*self.reg_ptr };
        regs.staclr.set(STA_UTXEN_MASK);
        //turn module of if receiver is disabled as well
        if regs.sta.get() & STA_URXEN_MASK == 0 {
            regs.modeclr.set(MODE_ON_MASK);
        }
    }
}

impl Drop for Rx {
    fn drop(&mut self) {
        let regs : &RegisterBlock = unsafe { &*self.reg_ptr };
        regs.staclr.set(STA_URXEN_MASK);
        //turn module of if transmitter is disabled as well
        if regs.sta.get() & STA_UTXEN_MASK == 0 {
            regs.modeclr.set(MODE_ON_MASK);
        }
    }
}

impl embedded_hal::serial::Read<u8> for Rx {
    type Error = ();

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let regs : &RegisterBlock = unsafe { &*self.reg_ptr };
        let urxda = (regs.sta.get() & STA_URXDA_MASK) != 0;
        if urxda {
            let byte = regs.rxreg.get() as u8;
            Ok(byte)
        }else{
            Err(nb::Error::WouldBlock)
        }
    }
}

impl embedded_hal::serial::Write<u8> for Tx {
    type Error = ();


    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        let regs : &RegisterBlock = unsafe { &*self.reg_ptr };
        let trmt = (regs.sta.get() & STA_TRMT_MASK) != 0;
        if trmt {
            Err(nb::Error::WouldBlock)
        }else{
            Ok(())
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        if self.try_write_byte(byte) {
            Ok(())
        }else{
            Err(nb::Error::WouldBlock)
        }
    }
}


impl Write for Tx {

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            while !self.try_write_byte(b) {};
        }
        Ok(())
    }

}
