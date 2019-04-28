/// I2C driver for PIC32

use vcell::VolatileCell;
use embedded_hal::blocking;

#[repr(C)]
pub struct RegisterBlock {
    pub con:        VolatileCell<u32>,
    pub conclr:     VolatileCell<u32>,
    pub conset:     VolatileCell<u32>,
    pub coninv:     VolatileCell<u32>,
    pub stat:       VolatileCell<u32>,
    pub statclr:    VolatileCell<u32>,
    pub statset:    VolatileCell<u32>,
    pub statinv:    VolatileCell<u32>,
    pub add:        VolatileCell<u32>,
    pub addclr:     VolatileCell<u32>,
    pub addset:     VolatileCell<u32>,
    pub addinv:     VolatileCell<u32>,
    pub msk:        VolatileCell<u32>,
    pub mskclr:     VolatileCell<u32>,
    pub mskset:     VolatileCell<u32>,
    pub mskinv:     VolatileCell<u32>,
    pub brg:        VolatileCell<u32>,
    pub brgclr:     VolatileCell<u32>,
    pub brgset:     VolatileCell<u32>,
    pub brginv:     VolatileCell<u32>,
    pub trn:        VolatileCell<u32>,
    pub trnclr:     VolatileCell<u32>,
    pub trnset:     VolatileCell<u32>,
    pub trninv:     VolatileCell<u32>,
    pub rcv:        VolatileCell<u32>,
}

const CON_SEN_MASK: u32         = 0x00000001;
const CON_RSEN_MASK: u32        = 0x00000002;
const CON_PEN_MASK: u32         = 0x00000004;
const CON_RCEN_MASK: u32        = 0x00000008;
const CON_ACKEN_MASK: u32       = 0x00000010;
const CON_ACKDT_MASK: u32       = 0x00000020;
const CON_DISSLW_MASK: u32      = 0x00000200;
const CON_ON_MASK: u32          = 0x00008000;
const STAT_TRSTAT_MASK: u32     = 0x00004000;
const STAT_ACKSTAT_MASK: u32    = 0x00008000;


// The values of this enum correspond to the base address of the I2C modules
#[allow(dead_code)]
#[repr(usize)]
pub enum HwModule {
    I2C1 = 0xbf80_5000,
    I2C2 = 0xbf80_5100,
}

// The values of this enum correspond to the divisor values mentioned in the
// reference manual
#[repr(u32)]
pub enum Fscl {
    F100KHZ  = 204248,
    F400KHZ  = 872600,
    F1000KHZ = 2525253,
}

pub struct I2c {
    reg_ptr: *const RegisterBlock,
    transaction_ongoing: bool,
}


impl I2c {

    pub const fn new(i2c: HwModule) -> I2c {
        // calculate base address
        let i2c_addr = i2c as usize;
        let reg_ptr = i2c_addr as *const RegisterBlock;
        I2c {
            reg_ptr: reg_ptr,
            transaction_ongoing: false,
        }
    }

    pub fn init(&mut self, pb_clock: u32, fsck: Fscl) {
        let regs : &RegisterBlock = unsafe { &*self.reg_ptr };
        let divisor = fsck as u32;

        self.transaction_ongoing = false;
        let round = if pb_clock % divisor > divisor/2 { 1 } else { 0 };
        let brg = pb_clock/divisor - 2 + round;
        regs.brg.set(brg);
        // disable slew rate control, see PIC32MX1xx/2xxx Silicon Errata, item 17
        regs.con.set(CON_ON_MASK | CON_DISSLW_MASK);
    }

    fn i2c_busy(&self) -> bool {
        let regs : &RegisterBlock = unsafe { &*self.reg_ptr };
        regs.con.get() & 0x1f != 0
    }

    pub fn transmit(&mut self, data: &[u8]) -> Result<(),()> {
        let regs : &RegisterBlock = unsafe { &*self.reg_ptr };
        if !self.transaction_ongoing {
            while self.i2c_busy(){};
            regs.conset.set(CON_SEN_MASK); // generate start condition
            self.transaction_ongoing = true;
        }
        for byte in data {
            while self.i2c_busy(){};
            regs.trn.set(*byte as u32);
            // wait until TX complete
            while regs.stat.get() & STAT_TRSTAT_MASK != 0 {};
            if regs.stat.get() & STAT_ACKSTAT_MASK != 0 { // NACK
                self.stop();
                return Err(());
            }
        }
        Ok(())
    }

    //generate repeated start condition
    pub fn rstart(&self) -> Result<(),()> {
        let regs : &RegisterBlock = unsafe { &*self.reg_ptr };
        if self.transaction_ongoing {
            while self.i2c_busy(){};
            regs.conset.set(CON_RSEN_MASK);
            Ok(())
        }else{
            Err(())
        }
    }

    pub fn stop(&mut self) {
        let regs : &RegisterBlock = unsafe { &*self.reg_ptr };
        while self.i2c_busy() {}
        regs.conset.set(CON_PEN_MASK);
        self.transaction_ongoing = false;
    }

    pub fn receive(&mut self, data: &mut[u8], nack_last: bool) -> Result<(),()> {
        let regs : &RegisterBlock = unsafe { &*self.reg_ptr };
        if !self.transaction_ongoing {
            return Err(());
        }
        let len = data.len();
        for (i, byte) in data.iter_mut().enumerate() {
            while self.i2c_busy(){};
            regs.conset.set(CON_RCEN_MASK);
            while self.i2c_busy(){};
            *byte = regs.rcv.get() as u8;
            if (i == len - 1) && nack_last { // NACK for last byte
                regs.conset.set(CON_ACKDT_MASK);
            }else{
                regs.conclr.set(CON_ACKDT_MASK);
            }
            regs.conset.set(CON_ACKEN_MASK);
        }
        Ok(())
    }
}

impl blocking::i2c::Write for I2c {

    type Error = ();

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.transmit(&[addr<<1])?;
        self.transmit(bytes)?;
        self.stop();
        Ok(())
    }
}

impl blocking::i2c::Read for I2c {

    type Error = ();

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.transmit(&[addr<<1])?;
        self.receive(buffer, true)?;
        self.stop();
        Ok(())
    }
}
