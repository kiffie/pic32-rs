//! Timer 1 Interrupt Service Routine

use alloc::boxed::Box;
use core::cell::RefCell;
use critical_section::Mutex;
use mips_rt::interrupt;
use pic32_hal::pac::{
    self,
    interrupt::{self, Interrupt, InterruptSource},
};
use pic32_hal::{
    int::{Int, IPL1, ISL2},
    timer::{timer_a::*, Clocking},
};

type Handler = Mutex<RefCell<Option<Box<dyn FnMut() + Send + 'static>>>>;

static HANDLER: Handler = Mutex::new(RefCell::new(None));

/// Handle of a Time 1 Interrupt Service Routine
///
/// This is a singleton because it consumes the Timer 1 PAC object
pub struct Timer1Isr {
    _timer: Timer<TimerSynchronous>,
}

impl Timer1Isr {
    /// create a UsbIsr handle
    pub fn new(tmr1: pac::TMR1) -> Self {
        let period = (48_000_000u32 / 256 / 1000) as u16;
        let timer = Timer::<TimerSynchronous>::timer1_synchronous(
            tmr1,
            Clocking::Pbclock,
            ClockPrescale::Prescale256,
            period,
            false,
        );
        Self { _timer: timer }
    }

    /// start interrupt processing
    pub fn start<F: FnMut() + Send + 'static>(&mut self, int: &Int, isr: F) {
        int.set_ipl(Interrupt::TIMER_1, IPL1);
        int.set_isl(Interrupt::TIMER_1, ISL2);
        int.ei(InterruptSource::TIMER_1);
        int.set_if(InterruptSource::TIMER_1);
        critical_section::with(|cs| {
            HANDLER.borrow(cs).replace(Some(Box::new(isr)));
        });
    }

    // pub fn tmr_mut(&mut self) -> &mut Timer<TimerSynchronous> {
    //     &mut self._timer
    // }
}

impl Drop for Timer1Isr {
    fn drop(&mut self) {
        let p: pac::Peripherals = unsafe { pac::Peripherals::steal() };
        let int = Int::new(p.INT);
        int.di(InterruptSource::TIMER_1);
    }
}

#[interrupt]
fn TIMER_1() {
    // Accessing the handler directly is safe because we know that there will be
    // no recursive ISR calls and that the main program is using the Mutex
    // safely.
    let ptr = &HANDLER as *const Handler as *mut Handler;
    // let r = ptr.cast_mut();
    let mut handler_cell = unsafe { (*ptr).get_mut() }.borrow_mut();
    let mut handler_option = handler_cell.as_deref_mut();

    if let Some(ref mut handler) = handler_option {
        handler();
    }
    let p: pac::Peripherals = unsafe { pac::Peripherals::steal() };
    let int = Int::new(p.INT);
    int.clear_if(InterruptSource::TIMER_1);
}
