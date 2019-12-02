// future for cp0timer


use core::future::Future;
use core::task::{Context, Poll, Waker};
use core::pin::Pin;
use core::time::Duration;

extern crate alloc;
use alloc::sync::Arc;
use core::cell::RefCell;

use mips_rt::interrupt;
use mips_rt::interrupt::Mutex;

use crate::cp0timer;
use crate::cp0timer::{Time, time_from_duration};

type StateMutex = Mutex<RefCell<State>>;

pub struct TimerFuture {
    state: Arc<StateMutex>,
}

struct State {
    completed: bool,
    waker: Option<Waker>,
}

fn timer_handler(_when: Time, context: *const StateMutex){
    let state_arc = unsafe { Arc::from_raw(context)};
    interrupt::free(|cs|{
        let mut state = state_arc.borrow(cs).borrow_mut();
        state.completed = true;
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    });
}

/// Create a timer future. Returns Err(()) in case of lack of resources
impl TimerFuture {
    pub fn new(duration: Duration) -> Result<Self, ()> {
        let future = TimerFuture {
            state: Arc::new(Mutex::new(RefCell::new(State {
                completed: false,
                waker: None
            })))};
        let mut timer = cp0timer::Timer::new();
        let state_ptr = Arc::into_raw(future.state.clone());
        match timer.at(time_from_duration(duration) + timer.now(), timer_handler, state_ptr){
            Ok(_) => Ok(future),
            Err(_) => {
                unsafe { let _dealloc_arc = Arc::from_raw(state_ptr); };
                Err(())
            }
        }
    }
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        interrupt::free(|cs|{
            let mut state = self.state.borrow(cs).borrow_mut();
            if state.completed {
                Poll::Ready(())
            }else{
                state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        })
    }
}
