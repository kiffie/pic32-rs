//! USB function driver for PIC32 devices with Full Speed USB hardware
//!
//! Uses dynamic memory (```alloc``` crate) to allocate USB buffers and buffer
//! descriptor table
//!
//! This module can be enabled with the ```usb-device``` feature.

extern crate alloc;
use alloc::alloc::{alloc, dealloc, Layout};
use alloc::boxed::Box;
use core::cell::RefCell;
use core::pin::Pin;
use core::ptr::{read_volatile, write_volatile};
use core::slice;

use mips_mcu::PhysicalAddress;
use mips_mcu::fmt::virt_to_phys;

use crate::pac::USB;

use usb_device as udev;
use usb_device::bus::PollResult;
use usb_device::bus::UsbBusAllocator;
use usb_device::endpoint::{EndpointAddress, EndpointType};
use usb_device::{Result, UsbDirection, UsbError};

// bit masks for endpoint control registers
const EPREG_EPHSHK_MASK: u8 =   0x01;
const EPREG_EPSTALL_MASK: u8 =  0x02;
const EPREG_EPTXEN_MASK: u8 =   0x04;
const EPREG_EPRXEN_MASK: u8 =   0x08;
const EPREG_EPCONDIS_MASK: u8 = 0x10;

// bit masks and positions for U1STAT register
const U1STAT_ENDPT_MASK : u32 =         0xf0;
const U1STAT_ENDPT_POSITION: usize =    4;
const U1STAT_DIR_MASK: u32 =            0x08;
const U1STAT_DIR_POSITION: usize =      3;
//const U1STAT_PPBI_MASK: u32 =           0x04;
//const U1STAT_PPBI_POSITION: u32 =       2;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct BufferDescriptor {
    flags: u16,
    byte_count: u16,
    buffer_address: PhysicalAddress,
}

// bitmasks for BufferDescriptor flags
const BD_UOWN: u16 =        0x80;
const BD_DATA01: u16 =      0x40;
//const BD_KEEP: u16 =        0x20;
//const BD_NINC: u16 =        0x10;
const BD_DTS: u16 =         0x08;
const BD_STALL: u16 =       0x04;

const BD_PID_POS: usize =   2;
const BD_PID_MSK: u16 =     0x3c;
//const BD_PID_LEN: usize =   4;

impl BufferDescriptor {
    const fn new(flags: u16, byte_count: u16, buffer_address: PhysicalAddress) -> BufferDescriptor {
        BufferDescriptor {
            flags,
            byte_count,
            buffer_address,
        }
    }

    const fn const_default() -> BufferDescriptor {
        Self::new(0, 0, PhysicalAddress::from_usize(0))
    }

    fn flags(&self) -> u16 {
        unsafe { read_volatile(&self.flags) }
    }

    fn set_flags(&mut self, flags: u16) {
        unsafe { write_volatile(&mut self.flags, flags) };
    }

    fn set_byte_count(&mut self, byte_count: u16) {
        unsafe { write_volatile(&mut self.byte_count, byte_count) };
    }

    fn set_buffer_address(&mut self, buffer_address: PhysicalAddress) {
        //debug!("dma_address = {}", buffer_address.as_usize());
        unsafe { write_volatile(&mut self.buffer_address, buffer_address) };
    }
}

const USB_PID_OUT  : u8 = 0x1;
const USB_PID_IN   : u8 = 0x9;
const USB_PID_SETUP: u8 = 0xd;

const N_ENDPOINTS: usize = 16;

#[repr(C, align(512))]
union BufferDescriptorTable {
    flat: [BufferDescriptor; 4 * N_ENDPOINTS],
    ep_dir_ppbi: [[[BufferDescriptor; 2]; 2]; N_ENDPOINTS],
}

impl BufferDescriptorTable {
    const fn new() -> BufferDescriptorTable {
        BufferDescriptorTable {
            flat: [BufferDescriptor::const_default(); 4 * N_ENDPOINTS],
        }
    }

    /// Get a raw pointer to the buffer descriptor table. Used to get an
    /// address that can be passed to the hardware
    fn as_raw(&mut self) -> *mut Self {
        self as *mut Self
    }
}

struct EndpointControlBlock {
    next_odd: bool,         // next BD to use is odd BD
    data01: bool,           // data toggle flag for next transaction
    stalled: bool,
    armed_ctr: u8,          // number of armed BD (0..=2)
    complete_ctr: u8,       // number of completed BD (0..=2), always 0 for IN
    next_complete_odd: bool,// next completed BD to process
    ep_type: EndpointType,
    ep_size: u16,           // max. transaction size (== size of ep_buf[.])
    ep_buf: [*mut u8; 2],   // transaction buffers
    bd: *mut [BufferDescriptor; 2],
}

impl EndpointControlBlock {
    fn alloc(
        ep_size: u16,
        ep_type: EndpointType,
        ep_addr: EndpointAddress,
        bd: *mut [BufferDescriptor; 2],
    ) -> Result<EndpointControlBlock> {
        let b0 = unsafe {
            alloc(
                Layout::from_size_align(ep_size as usize, 1)
                    .map_err(|_| UsbError::EndpointOverflow)?,
            )
        };
        if b0.is_null() {
            return Err(UsbError::EndpointOverflow);
        }
        let b1 = unsafe {
            alloc(
                Layout::from_size_align(ep_size as usize, 1)
                    .map_err(|_| UsbError::EndpointOverflow)?,
            )
        };
        if b1.is_null() {
            return Err(UsbError::EndpointOverflow);
        }
        let bd_pair: &mut [BufferDescriptor; 2] = unsafe { &mut *bd };
        bd_pair[0].set_flags(0);
        bd_pair[0].set_buffer_address(virt_to_phys(b0));
        bd_pair[1].set_flags(0);
        bd_pair[1].set_buffer_address(virt_to_phys(b1));

        // initialize endpoint control register
        let ep = ep_addr.index();
        let mut epreg = unsafe { UsbBus::read_epreg(ep) };
        if ep_addr.is_in() {
            epreg |= EPREG_EPTXEN_MASK;
        } else {
            epreg |= EPREG_EPRXEN_MASK;
        }
        epreg |= match ep_type {
            EndpointType::Control => EPREG_EPHSHK_MASK,
            EndpointType::Isochronous => EPREG_EPCONDIS_MASK,
            EndpointType::Bulk | EndpointType::Interrupt => EPREG_EPCONDIS_MASK | EPREG_EPHSHK_MASK,
        };
        unsafe { UsbBus::write_epreg(ep, epreg) };
        Ok(EndpointControlBlock {
            next_odd: false,
            data01: false,
            stalled: false,
            armed_ctr: 0,
            complete_ctr: 0,
            next_complete_odd: false,
            ep_type,
            ep_size,
            ep_buf: [b0, b1],
            bd,
        })
    }

    /// Test if an endpoint can be armed.
    /// Returns the endpoint buffer as a mutable slice if it can be armed.
    /// Otherwise, None is returned.
    fn can_arm(&mut self) -> Option<&mut [u8]> {
        let bd: &mut BufferDescriptor = unsafe { &mut (*self.bd)[self.next_odd as usize] };
        if bd.flags() & BD_UOWN == 0 {
            unsafe {
                Some(slice::from_raw_parts_mut(
                    self.ep_buf[self.next_odd as usize],
                    self.ep_size as usize,
                ))
            }
        } else {
            None
        }
    }

    fn arm_generic(&mut self, len: usize, stall: bool) -> Result<usize> {
        let bd: &mut BufferDescriptor = unsafe { &mut (*self.bd)[self.next_odd as usize] };
        if len > self.ep_size as usize {
            return Err(UsbError::BufferOverflow);
        }
        if self.armed_ctr + self.complete_ctr >= 2 {
            return Err(UsbError::WouldBlock);
        }
        if self.stalled {
            if stall {
                return Err(UsbError::InvalidState);
            }
            self.stalled = false;
            bd.set_flags(0);
        }
        bd.set_buffer_address(virt_to_phys(self.ep_buf[self.next_odd as usize]));
        bd.set_byte_count(len as u16);
        bd.set_flags( BD_UOWN | 
                      if self.data01 { BD_DATA01 } else { 0 } |
                      if self.ep_type == EndpointType::Isochronous { 0 } else { BD_DTS } |
                      if stall { BD_STALL } else { 0 } );
        if stall {
            self.stalled = true;
        } else {
            self.next_odd = !self.next_odd;
            self.armed_ctr += 1;
        }
        if self.ep_type != EndpointType::Isochronous {
            self.data01 = !self.data01;
        }
        Ok(len)
    }

//     fn stall(&mut self) {
//         if self.ep_addr.is_in() {
//             self.arm_generic(0, true);
//         }
//     }

    /// Cancel all pending (submitted and completed) transactions.
    /// Must be called with transactions disabled and the h/w FIFO flushed,
    /// e.g. when processing a SETUP transaction.
    fn cancel(&mut self) {
        self.next_complete_odd =
            (self.next_complete_odd as usize ^ self.complete_ctr as usize) & 0x01 != 0;
        self.complete_ctr = 0;
        self.next_odd = (self.next_odd as usize ^ self.armed_ctr as usize) & 0x01 != 0;
        self.armed_ctr = 0;
        unsafe {
            (*self.bd)[0].set_flags(0);
            (*self.bd)[1].set_flags(0);
        }
    }

    fn clear_completed(&mut self) {
        self.next_complete_odd =
            (self.next_complete_odd as usize ^ self.complete_ctr as usize) & 0x01 != 0;
        self.complete_ctr = 0;
    }

    fn write(&mut self, buf: &[u8]) -> udev::Result<usize> {
        let usb_buf = self.can_arm().ok_or(UsbError::WouldBlock)?;
        if usb_buf.len() < buf.len() {
            return Err(UsbError::BufferOverflow);
        }
        usb_buf[..buf.len()].copy_from_slice(buf);
        self.arm_generic(buf.len(), false)
    }

    fn read(&mut self, buf: &mut [u8]) -> udev::Result<usize> {
        if self.complete_ctr == 0 {
            return Err(UsbError::WouldBlock);
        }
        let bd: &mut BufferDescriptor = unsafe { &mut (*self.bd)[self.next_complete_odd as usize] };

        let byte_count = bd.byte_count as usize;
        if byte_count > buf.len() {
            return Err(UsbError::BufferOverflow);
        }
        let ptr = self.ep_buf[self.next_complete_odd as usize];
        let slice = unsafe { slice::from_raw_parts(ptr, byte_count) };
        buf[..byte_count].copy_from_slice(slice);
        self.complete_ctr -= 1;

        self.arm_generic(self.ep_size as usize, false)?;
        self.next_complete_odd = !self.next_complete_odd;
        Ok(byte_count)
    }
}

impl Drop for EndpointControlBlock {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ep_buf[0],
                    Layout::from_size_align_unchecked(self.ep_size as usize, 1));
            dealloc(self.ep_buf[1],
                    Layout::from_size_align_unchecked(self.ep_size as usize, 1));
        }
    }
}

type Ecb = [[Option<EndpointControlBlock>; 2]; N_ENDPOINTS];

struct UsbInner {
    bdt: Pin<Box<BufferDescriptorTable>>,
    usb: USB,
    ecb: Ecb,
    pr_out: u16,
    pr_su: u16,
}

/// Usb bus driver to be used with the usb-device crate.
pub struct UsbBus(RefCell<UsbInner>);

unsafe impl Sync for UsbBus {}

impl UsbBus {
    /// Create a new UsbBus. Uses the heap for allocating the various buffers.
    pub fn new(usb: USB) -> UsbBusAllocator<Self> {
        usb.u1con.write(unsafe { |w| w.bits(0) }); // first turn USB off
        //turn off VUSB, disable special USB OTG functions
        usb.u1otgcon.write(unsafe { |w| w.bits(0) });
        usb.u1pwrc.write(|w| w.usbpwr().bit(true));

        // disable all endpoints
        for i in 0..=15 {
            unsafe { Self::write_epreg(i, 0) };
        }

        // create Buffer Descriptor Table (BDT) and inform the hardware about it
        let mut bdt = Box::pin(BufferDescriptorTable::new());
        let dma_addr = virt_to_phys(bdt.as_raw()).address() as u32;
        usb.u1bdtp3.write(unsafe { |w| w.bits(dma_addr >> 24) });
        usb.u1bdtp2.write(unsafe { |w| w.bits(dma_addr >> 16) });
        usb.u1bdtp1.write(unsafe { |w| w.bits(dma_addr >> 8) });

        let bus = UsbBus(RefCell::new(UsbInner {
            bdt,
            usb,
            ecb: Ecb::default(),
            pr_out: 0,
            pr_su: 0,
        }));
        UsbBusAllocator::new(bus)
    }

    /// write to endpoint control register
    unsafe fn write_epreg(ndx: usize, val: u8) {
        let epregs = &((*USB::ptr()).u1ep0) as *const _ as usize;
        let epreg = (epregs + 4 * 4 * ndx) as *mut u32;
        write_volatile(epreg, val as u32);
    }

    /// read endpoint control register
    unsafe fn read_epreg(ndx: usize) -> u8 {
        let epregs = &((*USB::ptr()).u1ep0) as *const _ as usize;
        let epreg = (epregs + 4 * 4 * ndx) as *mut u32;
        read_volatile(epreg) as u8
    }
}

impl Drop for UsbBus {
    fn drop(&mut self) {
        let usb = &self.0.borrow_mut().usb;
        usb.u1ie.write(unsafe { |w| w.bits(0) });
        usb.u1pwrc.write(unsafe { |w| w.bits(0) });
    }
}

impl usb_device::bus::UsbBus for UsbBus {
    /// Allocate an unidirectional endpoint. To create a control endpoint, which
    /// is always bidirectional, this method needs to be called twice with
    /// different values for ep_dir.
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        ep_size: u16,
        _interval: u8,
    ) -> Result<EndpointAddress> {
        let mut inner = self.0.borrow_mut();
        let addr = if let Some(a) = ep_addr {
            // consistency check for ep_dir
            if a.direction() != ep_dir {
                return Err(UsbError::InvalidEndpoint);
            }
            // range check
            let ep = a.index();
            if ep >= N_ENDPOINTS {
                return Err(UsbError::EndpointOverflow);
            }
            // check if endpoint is already in use
            let dir = (a.direction() as u8 >> 7) as usize;
            if inner.ecb[ep][dir].is_some() {
                return Err(UsbError::InvalidEndpoint);
            }
            a
        } else {
            // find a free endpoint starting with EP1
            let dir = (ep_dir as u8 >> 7) as usize;
            let mut addr = None;
            for ep in 1..N_ENDPOINTS {
                if inner.ecb[ep][dir].is_none() {
                    addr = Some(EndpointAddress::from_parts(ep, ep_dir));
                    break;
                }
            }
            match addr {
                Some(a) => a,
                None => return Err(UsbError::EndpointOverflow),
            }
        };
        let ep = addr.index();
        let dir = addr.direction() as usize >> 7;

        // initialize buffer descriptors and endpoint control block
        let bd_pair = unsafe { &mut inner.bdt.ep_dir_ppbi[ep][dir] };
        let ecb = EndpointControlBlock::alloc(ep_size, ep_type, addr, bd_pair)?;
        inner.ecb[ep][dir] = Some(ecb);
        //if ep_type == EndpointType::Control && addr.is_out() {
        if addr.is_out() {
            let ecb = inner.ecb[ep][dir].as_mut().unwrap();
            ecb.arm_generic(ep_size as usize, false)?;
            //ecb.arm_generic(ep_size as usize, false)?;
        }
        Ok(addr)
    }

    fn enable(&mut self) {
        let inner = self.0.borrow();

        // Enable interrupts required to call the poll function from an ISR
        // To use the interrupts, the interrupt controller must be configured
        // as well.
        inner.usb.u1ie.write(|w| w
            .trnie().bit(true)
            .stallie().bit(true)
            .urstie_detachie().bit(true));
        inner.usb.u1con.write(|w| w.usben_sofen().bit(true));
    }

    fn reset(&self) {
        let mut inner = self.0.borrow_mut();
        if let Some(ref mut ecb) = inner.ecb[0][0] {
            ecb.clear_completed();
        }
    }

    fn set_device_address(&self, addr: u8) {
        let inner = self.0.borrow();
        inner
            .usb
            .u1addr
            .write(|w| unsafe { w.bits((addr & 0x7f) as u32) });
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> udev::Result<usize> {
        let ep = ep_addr.index();
        if ep >= N_ENDPOINTS {
            return Err(UsbError::InvalidEndpoint);
        }
        let dir = ep_addr.is_in() as usize;
        let mut inner = self.0.borrow_mut();
        let ecb = inner.ecb[ep][dir]
            .as_mut()
            .ok_or(UsbError::InvalidEndpoint)?;
        ecb.write(buf)
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> udev::Result<usize> {
        let ep = ep_addr.index();
        if ep >= N_ENDPOINTS || ep_addr.direction() != UsbDirection::Out {
            return Err(UsbError::InvalidEndpoint);
        }
        let mut inner = self.0.borrow_mut();
        let ecb = inner.ecb[ep][0].as_mut().ok_or(UsbError::InvalidEndpoint)?;
        let len = ecb.read(buf)?;
        inner.pr_out &= !(1 << ep);
        inner.pr_su &= !(1 << ep);
        Ok(len)
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        if stalled {
            let ep = ep_addr.index();
            unsafe {
                Self::write_epreg(ep, Self::read_epreg(ep) | EPREG_EPSTALL_MASK);
            }
        }
    }

    fn is_stalled(&self, _ep_addr: EndpointAddress) -> bool {
        false
    }

    fn suspend(&self) {}

    fn resume(&self) {}

    fn poll(&self) -> PollResult {
        let mut inner = self.0.borrow_mut();
        let mut pr_in = 0u16;
        let u1eir = inner.usb.u1eir.read().bits();
        if u1eir != 0 {
            inner.usb.u1eir.write(|w| unsafe { w.bits(u1eir) });
        }
        while inner.usb.u1ir.read().trnif().bit() {
            let u1stat = inner.usb.u1stat.read().bits(); // copy status
            inner.usb.u1ir.write(|w| w.trnif().bit(true)); // clear IRQ flag
            let ep = ((u1stat & U1STAT_ENDPT_MASK) >> U1STAT_ENDPT_POSITION) as usize;
            let dir = ((u1stat & U1STAT_DIR_MASK) >> U1STAT_DIR_POSITION) as usize;
            let bdt_index = (u1stat >> 2) as usize;
            let bd_flags = unsafe { inner.bdt.flat[bdt_index].flags() };
            let pid = ((bd_flags & BD_PID_MSK) >> BD_PID_POS) as u8;
            //debug!("pid = {}, ep = {}, dir = {}", pid, ep, dir);
            match pid {
                USB_PID_OUT => {
                    if let Some(ref mut ecb) = inner.ecb[ep][dir] {
                        ecb.complete_ctr += 1;
                        ecb.armed_ctr -= 1;
                    }
                    inner.pr_out |= 1 << ep;
                }
                USB_PID_IN => {
                    pr_in |= 1 << ep;
                    if let Some(ref mut ecb) = inner.ecb[ep][dir] {
                        ecb.armed_ctr -= 1;
                    }
                }
                USB_PID_SETUP => {
                    // stop any previously submitted IN transactions
                    if let Some(ref mut ecb_in) = inner.ecb[ep as usize][1] {
                        ecb_in.cancel();
                    }
                    let ecb_out = inner.ecb[ep][0].as_mut().unwrap();
                    // delete already received but undelivered OUT transactions
                    ecb_out.clear_completed();

                    ecb_out.complete_ctr += 1;
                    ecb_out.armed_ctr -= 1;
                    inner.pr_su |= 1 << ep;
                    inner.ecb[ep as usize][0].as_mut().unwrap().data01 = true;
                    inner.ecb[ep as usize][1].as_mut().unwrap().data01 = true;
                    // the USB modules automatically disables USB transactions
                    // after a SETUP transaction. Enable them again.
                    inner.usb.u1conclr.write(|w| w.pktdis_tokbusy().bit(true));
                }
                _ => {
                }
            }
        }
        if inner.usb.u1ir.read().urstif_detachif().bit() {
            inner.usb.u1addr.write(unsafe { |w| w.bits(0) });
            inner.usb.u1ir.write(|w| w.urstif_detachif().bit(true));
            return PollResult::Reset;
        }
        if inner.usb.u1ir.read().stallif().bit() {
            inner.usb.u1ep0clr.write(|w| w.epstall().bit(true));
            inner.usb.u1ir.write(|w| w.stallif().bit(true));
        }
        if inner.pr_out != 0 || pr_in != 0 || inner.pr_su != 0 {
            PollResult::Data {
                ep_out: inner.pr_out,
                ep_in_complete: pr_in,
                ep_setup: inner.pr_su,
            }
        } else {
            PollResult::None
        }
    }
}
