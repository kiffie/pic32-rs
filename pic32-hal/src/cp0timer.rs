/// Timer and delay function using the MIPS CP0 counter and compare registers

pub mod future;

use core::cell::{RefCell};
use core::time::Duration;
use core::mem::transmute;
use mips_rt::interrupt;
use mips_rt::interrupt::Mutex;

use tinylog::{debug, trace};
const TL_LOGLEVEL: tinylog::Level = tinylog::Level::Error;

use sysconfig::SYS_CLOCK;

const MAX_TIMERS: usize = 16;

use crate::pac;

pub type Time = u64;

const TICKS_PER_US: u64 = SYS_CLOCK/2000000;
const TICKS_PER_MS: u64 = (TICKS_PER_US*1000);
const TICKS_PER_SECOND: u64 = (TICKS_PER_US*1000000);

//Maximum delay used to schedule next IRQ
const MAX_DELAY: u64 = 0xffff_ffff / 2;

//Minimum delay to avoid race condition (counter is incrementing continuously)
const MIN_DELAY: u64 = 100; // 200 CPU cycles

pub const fn time_from_micros(us: usize) -> Time {
    TICKS_PER_US * us as Time
}

pub const fn time_from_millis(ms: usize) -> Time {
    TICKS_PER_MS * ms as Time
}

pub const fn time_from_secs(seconds: usize) -> Time {
    TICKS_PER_SECOND * seconds as Time
}

/// calculated a Time in future based on a Duration
pub const fn time_from_duration(d: Duration) -> Time {
    TICKS_PER_US * d.as_micros() as Time
}

#[derive(Clone, Copy)]
struct TimerCbInfo {
    when: Time,
    isr_cb: fn(Time, *const ()),
    context: *const (),
}

struct State {
    is_initialized: bool,
    timer_high: u32,
    last_timer_low: u32,
    timers: [Option<TimerCbInfo>; MAX_TIMERS],
}

static mut STATE: Mutex<RefCell<State>> = Mutex::new(RefCell::new(State {
    is_initialized: false,
    timer_high: 0,
    last_timer_low: 0,
    timers: [None; MAX_TIMERS],
}));

// does not work with MIPS16
fn write_to_compare(v: u32){
    unsafe { asm!("mtc0   $0, $$11" : : "r"(v) : : "volatile"); }
}

// does not work with MIPS16
fn read_count() -> u32 {
    let mut count;
    unsafe { asm!("mfc0 $0,$$9" : "=r"(count) : : : "volatile")};
    count
}

/// Initialize the timer if not yet done
fn init() {
    interrupt::free(|cs| {
        let mut state = unsafe { STATE.borrow(cs).borrow_mut() };
        if !state.is_initialized {
            // initialize timer
            let p = unsafe { pac::Peripherals::steal() };
            p.INT.iec0set.write(|w| { w.ctie().bit(true) } );
            p.INT.ipc0.write(|w| unsafe { w.ctip().bits(1) } );
            write_to_compare(((state.now() + MAX_DELAY) & 0xffff_ffff) as u32 );
            state.is_initialized = true;
            debug!("CP0 timer initialized");
        }
    });
}


impl State {
    /// update high u32 word of timer and return current Time
    fn now(&mut self) -> Time {
        let timer_low = read_count();
        if timer_low < self.last_timer_low { // detect CP0 timer overflow
            self.timer_high += 1;
        }
        self.last_timer_low = timer_low;
        ((self.timer_high as u64) << 32) | (timer_low as u64)
    }
}

// CPO timer ISR
#[no_mangle]
pub extern "C" fn _vector_0_fn() {
    trace!("!ISR");
    
    for i in 0..MAX_TIMERS {
        // move callback info to ot if timer expired
        let ot = interrupt::free(|cs| {
            let mut state = unsafe { STATE.borrow(cs).borrow_mut() };
            if let Some(ref t) = state.timers[i] {
                if t.when <= state.now() {
                    state.timers[i].take()
                }else{
                    None
                }
            }else{
                None
            }
        });
        // do the callback outside of the critical section
        if let Some(t) = ot {
            (t.isr_cb)(t.when, t.context);
        }
    }
    // prepare next ISR invocation
    interrupt::free(|cs| {
        let mut state = unsafe { STATE.borrow(cs).borrow_mut() };
        let mut earliest = u64::max_value();
        for timer in &state.timers {
            if let Some(timer) = timer {
                if timer.when < earliest {
                    earliest = timer.when;
                }
            }
        }
        let now = state.now();
        let mut delay = if earliest > now { earliest - now } else { 0 };
        if delay > MAX_DELAY {
            delay = MAX_DELAY;
        }
        if delay < MIN_DELAY {
            delay = MIN_DELAY;
        }
        write_to_compare(((now + delay) & 0xffff_ffff ) as u32 );
    });

    let p = unsafe { pac::Peripherals::steal() };
    p.INT.ifs0clr.write(|w| { w.ctif().bit(true) } );
}

pub struct Timer {}

impl Timer{

    pub fn new() -> Timer {
        init();
        Timer { }
    }

    pub fn now(&self) -> u64 {
        interrupt::free(|cs| {
            let mut state = unsafe { STATE.borrow(cs).borrow_mut() };
            state.now()
        })
    }

    pub fn now_secs(&self) -> u64 {
        (self.now() as u64) / TICKS_PER_SECOND
    }

    pub fn now_millis(&self) -> u64 {
        self.now() / TICKS_PER_MS
    }

    pub fn delay_secs(&self, seconds: usize) {
        let when = self.now() + time_from_secs(seconds);
        while self.now() < when {}
    }

    pub fn delay_millis(&self, ms: usize) {
        let when = self.now() + time_from_millis(ms);
        while self.now() < when {}
    }

    pub fn delay_micros(&self, us: usize) {
        let when = self.now() + time_from_micros(us);
        while self.now() < when {}
    }



    /// call isr_hook not earlier than at the time indicated with when
    /// isr_cb may be called from an ISR context
    /// isr_cb is called exactly once if now >= when
    /// context is a raw pointer to be passed to isr_cb
    pub fn at<T>(&mut self, when: Time,
                 isr_cb: fn(Time, *const T), context: *const T) -> Result<(),()>
    {
        let now = self.now();
        if when <= now {
            isr_cb(when, context);
            return Ok(());
        }
        interrupt::free(|cs| {
            let mut state = unsafe { STATE.borrow(cs).borrow_mut() };
            //find a free slot
            let mut ndx = None;
            for i in 0..state.timers.len() {
                if state.timers[i].is_none() {
                    ndx = Some(i);
                    break;
                }
            }
            if let Some(ndx) = ndx {
                state.timers[ndx] = Some(TimerCbInfo {
                    when: when,
                    isr_cb: unsafe {transmute::<fn(Time, *const T),fn(Time, *const ())>(isr_cb)},
                    context: context as *const (),
                });
                debug!("now  = {:#x}", now);
                debug!("when = {:#x}", when);
                // trigger an IRQ (this has no effect when already in ISR context)
                let p = unsafe { pac::Peripherals::steal() };
                p.INT.ifs0set.write(|w| { w.ctif().bit(true) } );
                Ok(())
            }else{
                debug!("all timer slots used");
                Err(())
            }
        })
    }
}
