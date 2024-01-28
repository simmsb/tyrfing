use atxtiny_hal::{
    clkctrl::Clocks,
    embedded_hal::digital::OutputPin,
    gpio::{Output, Pin, Portc, Stateful},
    timer::{
        tcb::{self, TCBClockSource},
        Counter,
    },
};
use avr_device::attiny1616::TCB0;
/// embassy_time implementation for avr
///
/// configure using feature flags
///
/// adjust QUEUE_SIZE so you don't run out of stack
///
/// adjust resolution if timer can't keep up,
/// this is because timer interrupt doesn't finish fast enough
///
///
/// TODO: allow configuration from env
///
/// use macro to define interrupt
/// ```rust
/// define_interrupt!(atmega328p)
/// ```
///
/// you need to initialize timer as well
/// ```rust
/// init_system_time(&mut dp.TC0);
/// ```
/// note you **must** initialize timer before using embassy_time
use core::{mem::MaybeUninit, task::Waker};
use embassy_time_driver::{AlarmHandle, Driver};
use embassy_time_queue_driver::TimerQueue;
use env_int::env_int;
use fugit::TimerDurationU32;

#[cfg(feature = "time_u32")]
pub type Time = u32;
#[cfg(not(feature = "time_u32"))]
pub type Time = u64;

pub static mut TICKS_ELAPSED: Time = 0;

pub struct AvrTc0EmbassyTimeDriver {}

#[allow(dead_code)]
const TICKS_PER_COUNT: Time = 1;

const QUEUE_SIZE: usize = env_int!("AVR_EMBASSY_TIME_QUEUE_SIZE", 4);

#[rustc_layout_scalar_valid_range_start(0)]
#[rustc_layout_scalar_valid_range_end(254)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct NonMaxU8(u8);

#[derive(Copy, Clone)]
struct Alarm {
    callback: fn(*mut ()),
    ctx: *mut (),
}

mod timer_queue {
    use avr_device::interrupt::CriticalSection;

    use super::{Alarm, NonMaxU8, Time, QUEUE_SIZE};

    static mut ALLOCATED: Option<NonMaxU8> = None;

    static mut FREE: Option<NonMaxU8> = Some(unsafe { NonMaxU8(0) });

    fn empty_cb(_x: *mut ()) {}
    const EMPTY_ALARM: Alarm = Alarm {
        callback: empty_cb,
        ctx: core::ptr::null_mut(),
    };

    // AT initialisation:
    //
    // FREE (0) -> NEXTS[0] (1) -> NEXTS[1] (2) -> NEXTS[2] (3) -> NEXTS[3] (None)

    static mut NEXTS: [Option<NonMaxU8>; QUEUE_SIZE] = {
        let mut i = 0;

        let mut nexts = [None; QUEUE_SIZE];

        while i < QUEUE_SIZE - 1 {
            nexts[i] = unsafe { Some(NonMaxU8(i as u8 + 1)) };
            i += 1;
        }

        nexts
    };
    static mut ATS: [Time; QUEUE_SIZE] = [0; QUEUE_SIZE];
    static mut ALARMS: [Alarm; QUEUE_SIZE] = [EMPTY_ALARM; QUEUE_SIZE];

    pub fn allocate(_: CriticalSection) -> Option<NonMaxU8> {
        unsafe {
            FREE.map(|x| {
                FREE = *NEXTS.get_unchecked(x.0 as usize);
                NEXTS[x.0 as usize] = None;
                x
            })
        }
    }

    pub fn free(_: CriticalSection, id: NonMaxU8) {
        unsafe {
            *NEXTS.get_unchecked_mut(id.0 as usize) = FREE;
            *ALARMS.get_unchecked_mut(id.0 as usize) = EMPTY_ALARM;
            FREE = Some(id);
        }
    }

    pub fn push(_: CriticalSection, id: NonMaxU8, timestamp: Time) {
        unsafe {
            let mut next = &mut ALLOCATED;

            while let Some(i) = next {
                let next_at = at(*i);

                if next_at > timestamp {
                    *NEXTS.get_unchecked_mut(id.0 as usize) = Some(*i);
                    break;
                }

                next = NEXTS.get_unchecked_mut(i.0 as usize);
            }

            *next = Some(id);
        }
    }

    pub fn process(start: NonMaxU8, ticks_elapsed: Time) -> Option<NonMaxU8> {
        let mut next = Some(start);

        while let Some(id) = next {
            if at(id) <= ticks_elapsed {
                next = unsafe { *NEXTS.get_unchecked(id.0 as usize) };
                let Alarm { callback, ctx } = alarm(id);

                avr_hal_generic::avr_device::interrupt::free(|t| free(t, id));

                callback(ctx);
            } else {
                return Some(id);
            }
        }

        None
    }

    pub fn at(id: NonMaxU8) -> Time {
        unsafe { *ATS.get_unchecked(id.0 as usize) }
    }

    pub fn set_at(_: CriticalSection, id: NonMaxU8, v: Time) {
        unsafe { *ATS.get_unchecked_mut(id.0 as usize) = v }
    }

    pub fn alarm(id: NonMaxU8) -> Alarm {
        unsafe { *ALARMS.get_unchecked(id.0 as usize) }
    }

    pub fn set_alarm(_: CriticalSection, id: NonMaxU8, a: Alarm) {
        unsafe { *ALARMS.get_unchecked_mut(id.0 as usize) = a }
    }

    pub fn take_allocated(_: CriticalSection) -> Option<NonMaxU8> {
        unsafe { ALLOCATED.take() }
    }

    pub fn put_allocated(_: CriticalSection, x: Option<NonMaxU8>) {
        unsafe { ALLOCATED = x }
    }
}

mod wake_queue {
    use core::task::Waker;

    use avr_device::interrupt::CriticalSection;

    use super::{NonMaxU8, Time, QUEUE_SIZE};

    static mut ALLOCATED: Option<NonMaxU8> = None;

    static mut FREE: Option<NonMaxU8> = Some(unsafe { NonMaxU8(0) });

    static mut NEXTS: [Option<NonMaxU8>; QUEUE_SIZE] = {
        let mut i = 0;

        let mut nexts = [None; QUEUE_SIZE];

        while i < QUEUE_SIZE - 1 {
            nexts[i] = unsafe { Some(NonMaxU8(i as u8)) };
            i += 1;
        }

        nexts
    };
    static mut ATS: [Time; QUEUE_SIZE] = [0; QUEUE_SIZE];
    const X: Option<Waker> = None;
    static mut WAKERS: [Option<Waker>; QUEUE_SIZE] = [X; QUEUE_SIZE];

    pub fn allocate(_: CriticalSection) -> Option<NonMaxU8> {
        unsafe {
            FREE.map(|x| {
                FREE = *NEXTS.get_unchecked(x.0 as usize);
                NEXTS[x.0 as usize] = None;
                x
            })
        }
    }

    pub fn free(_: CriticalSection, id: NonMaxU8) {
        unsafe {
            *NEXTS.get_unchecked_mut(id.0 as usize) = FREE;
            *WAKERS.get_unchecked_mut(id.0 as usize) = None;
            FREE = Some(id);
        }
    }

    pub fn push(_: CriticalSection, id: NonMaxU8, timestamp: Time) {
        unsafe {
            let mut next = &mut ALLOCATED;

            while let Some(i) = next {
                let next_at = at(*i);

                if next_at > timestamp {
                    *NEXTS.get_unchecked_mut(id.0 as usize) = Some(*i);
                    break;
                }

                next = NEXTS.get_unchecked_mut(i.0 as usize);
            }

            *next = Some(id);
        }
    }

    pub fn process(start: NonMaxU8, ticks_elapsed: Time) -> Option<NonMaxU8> {
        let mut next = Some(start);

        while let Some(id) = next {
            if at(id) <= ticks_elapsed {
                next = unsafe { *NEXTS.get_unchecked_mut(id.0 as usize) };
                let w = waker(id);

                avr_hal_generic::avr_device::interrupt::free(|t| free(t, id));

                if let Some(w) = w {
                    w.wake();
                }
            } else {
                return Some(id);
            }
        }

        None
    }

    pub fn at(id: NonMaxU8) -> Time {
        unsafe { *ATS.get_unchecked(id.0 as usize) }
    }

    pub fn set_at(_: CriticalSection, id: NonMaxU8, v: Time) {
        unsafe { *ATS.get_unchecked_mut(id.0 as usize) = v }
    }

    pub fn waker(id: NonMaxU8) -> Option<Waker> {
        unsafe { WAKERS.get_unchecked(id.0 as usize).clone() }
    }

    pub fn set_waker(_: CriticalSection, id: NonMaxU8, w: Option<Waker>) {
        unsafe { *WAKERS.get_unchecked_mut(id.0 as usize) = w }
    }

    pub fn take_allocated(_: CriticalSection) -> Option<NonMaxU8> {
        unsafe { ALLOCATED.take() }
    }

    pub fn put_allocated(_: CriticalSection, x: Option<NonMaxU8>) {
        unsafe { ALLOCATED = x }
    }
}

impl Driver for AvrTc0EmbassyTimeDriver {
    #[inline(always)]
    fn now(&self) -> Time {
        avr_hal_generic::avr_device::interrupt::free(|_| unsafe {
            // including the current count uses about 100 bytes?
            // let cnt = avr_device::attiny1616::TCB0::steal().cnt().read().bits() / DIVIDER as u16;
            TICKS_ELAPSED // + cnt as Time
        })
    }

    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
        avr_hal_generic::avr_device::interrupt::free(|t| timer_queue::allocate(t))
            .map(|n| AlarmHandle::new(n.0))
    }

    fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
        avr_hal_generic::avr_device::interrupt::free(|t| {
            timer_queue::set_alarm(t, unsafe { NonMaxU8(alarm.id()) }, Alarm { callback, ctx })
        })
    }

    fn set_alarm(&self, alarm: AlarmHandle, timestamp: Time) -> bool {
        unsafe {
            avr_hal_generic::avr_device::interrupt::free(|t| {
                timer_queue::set_at(t, NonMaxU8(alarm.id()), timestamp);
                timer_queue::push(t, NonMaxU8(alarm.id()), timestamp);
            });
        }
        true
    }
}

impl TimerQueue for AvrTc0EmbassyTimeDriver {
    // #[inline(never)]
    fn schedule_wake(&'static self, at: Time, waker: &Waker) {
        avr_device::interrupt::free(|t| {
            let Some(id) = wake_queue::allocate(t) else {
                panic!("queue full");
            };

            wake_queue::set_waker(t, id, Some(waker.clone()));
            wake_queue::set_at(t, id, at);
            wake_queue::push(t, id, at);
        })
    }
}

#[avr_device::interrupt(attiny1616)]
unsafe fn TCB0_INT() {
    __tcb0_top()
}

// const CLOCK: u64 = 10_000_000;
const CLOCK: u32 = 32_000;

struct InterruptState {
    pub counter: Counter<TCB0, CLOCK>,
    pub led: Option<Pin<Portc, atxtiny_hal::gpio::U<0>, Output<Stateful>>>,
}

static mut INTERRUPT_STATE: MaybeUninit<InterruptState> = MaybeUninit::uninit();

#[inline(always)]
pub unsafe fn __tcb0_top() {
    let state = unsafe { &mut *INTERRUPT_STATE.as_mut_ptr() };
    let _ = state.led.as_mut().map(|p| p.set_low());
    let (timer_queue_next, wake_queue_next, ticks_elapsed) = avr_device::interrupt::free(|t| {
        TICKS_ELAPSED += TICKS_PER_COUNT;
        (
            timer_queue::take_allocated(t),
            wake_queue::take_allocated(t),
            TICKS_ELAPSED,
        )
    }); //minimize critical section

    let timer_queue_next =
        timer_queue_next.and_then(|next| timer_queue::process(next, ticks_elapsed));
    let wake_queue_next = wake_queue_next.and_then(|next| wake_queue::process(next, ticks_elapsed));

    avr_device::interrupt::free(|t| {
        timer_queue::put_allocated(t, timer_queue_next);
        wake_queue::put_allocated(t, wake_queue_next);
    });
    state.counter.clear_event(tcb::Event::CaptureCompare);
    let _ = state.led.as_mut().map(|p| p.set_high());
}

pub fn init_system_time(
    tc: TCB0,
    clocks: Clocks,
    p: Option<Pin<Portc, atxtiny_hal::gpio::U<0>, Output<Stateful>>>,
) {
    unsafe {
        avr_device::interrupt::enable();
        avr_device::interrupt::free(|_| {
            TICKS_ELAPSED = 0;

            let timer =
                atxtiny_hal::timer::FTimer::<_, CLOCK>::new(tc, TCBClockSource::Peripheral(clocks))
                    .unwrap();
            let mut c = timer.counter();
            c.enable_interrupt(tcb::Interrupt::CaptureCompare);
            c.start(TimerDurationU32::Hz(32)).unwrap();

            INTERRUPT_STATE = MaybeUninit::new(InterruptState { counter: c, led: p });
        });
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: AvrTc0EmbassyTimeDriver = AvrTc0EmbassyTimeDriver{});
embassy_time_queue_driver::timer_queue_impl!(static QUEUE_DRIVER: AvrTc0EmbassyTimeDriver = AvrTc0EmbassyTimeDriver{});
