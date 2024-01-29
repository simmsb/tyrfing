use atxtiny_hal::{
    clkctrl::Clocks,
    embedded_hal::digital::OutputPin,
    gpio::{Output, Pin, Portc, Stateful},
    timer::{
        rtc::{Pit, RTCClockSource}, tcb::{self, TCBClockSource}, Counter
    },
};
use avr_device::attiny1616::{rtc::pitctrla::PERIOD_A, RTC};
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

    fn empty_cb(_x: *mut ()) {}
    const EMPTY_ALARM: Alarm = Alarm {
        callback: empty_cb,
        ctx: core::ptr::null_mut(),
    };

    static mut IN_PROGRESS: bool = false;
    static mut TAKEN: [bool; QUEUE_SIZE] = [false; QUEUE_SIZE];
    static mut ATS: [Time; QUEUE_SIZE] = [0; QUEUE_SIZE];
    static mut ALARMS: [Alarm; QUEUE_SIZE] = [EMPTY_ALARM; QUEUE_SIZE];

    pub fn allocate(_: CriticalSection) -> Option<NonMaxU8> {
        unsafe {
            for i in 0..QUEUE_SIZE {
                if !TAKEN[i] {
                    TAKEN[i] = true;
                    return Some(NonMaxU8(i as u8));
                }
            }

            None
        }
    }

    pub fn mark_in_progress(_: CriticalSection) -> bool {
        unsafe { !core::mem::replace(&mut IN_PROGRESS, true) }
    }

    pub fn mark_finished(_: CriticalSection) {
        unsafe {
            IN_PROGRESS = false;
        }
    }

    pub fn process(ticks_elapsed: Time) {
        unsafe {
            for i in 0..QUEUE_SIZE {
                if !TAKEN[i] {
                    continue;
                }

                if ATS[i] <= ticks_elapsed {
                    let Alarm { callback, ctx } = ALARMS[i];

                    TAKEN[i] = false;

                    callback(ctx);
                }
            }
        }
    }
    pub fn set_at(_: CriticalSection, id: NonMaxU8, v: Time) {
        unsafe { *ATS.get_unchecked_mut(id.0 as usize) = v }
    }

    pub fn set_alarm(_: CriticalSection, id: NonMaxU8, a: Alarm) {
        unsafe { *ALARMS.get_unchecked_mut(id.0 as usize) = a }
    }
}

mod wake_queue {
    use core::task::Waker;

    use avr_device::interrupt::CriticalSection;

    use super::{NonMaxU8, Time, QUEUE_SIZE};

    static mut IN_PROGRESS: bool = false;
    static mut TAKEN: [bool; QUEUE_SIZE] = [false; QUEUE_SIZE];
    static mut ATS: [Time; QUEUE_SIZE] = [0; QUEUE_SIZE];
    const X: Option<Waker> = None;
    static mut WAKERS: [Option<Waker>; QUEUE_SIZE] = [X; QUEUE_SIZE];

    pub fn allocate(_: CriticalSection) -> Option<NonMaxU8> {
        unsafe {
            for i in 0..QUEUE_SIZE {
                if !TAKEN[i] {
                    TAKEN[i] = true;
                    return Some(NonMaxU8(i as u8));
                }
            }

            None
        }
    }

    pub fn mark_in_progress(_: CriticalSection) -> bool {
        unsafe { !core::mem::replace(&mut IN_PROGRESS, true) }
    }

    pub fn mark_finished(_: CriticalSection) {
        unsafe {
            IN_PROGRESS = false;
        }
    }

    pub fn process(ticks_elapsed: Time) {
        unsafe {
            for i in 0..QUEUE_SIZE {
                if !TAKEN[i] {
                    continue;
                }

                if ATS[i] <= ticks_elapsed {
                    let w = WAKERS[i].take();

                    TAKEN[i] = false;

                    if let Some(w) = w {
                        w.wake();
                    }
                }
            }
        }
    }

    pub fn set_at(_: CriticalSection, id: NonMaxU8, v: Time) {
        unsafe { *ATS.get_unchecked_mut(id.0 as usize) = v }
    }

    pub fn set_waker(_: CriticalSection, id: NonMaxU8, w: Option<Waker>) {
        unsafe { *WAKERS.get_unchecked_mut(id.0 as usize) = w }
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
        })
    }
}

#[avr_device::interrupt(attiny1616)]
unsafe fn RTC_PIT() {
    handle_tick()
}

struct InterruptState {
    pub counter: Pit,
    pub led: Option<Pin<Portc, atxtiny_hal::gpio::U<0>, Output<Stateful>>>,
}

static mut INTERRUPT_STATE: MaybeUninit<InterruptState> = MaybeUninit::uninit();

#[inline(always)]
pub unsafe fn handle_tick() {
    let state = unsafe { &mut *INTERRUPT_STATE.as_mut_ptr() };
    let _ = state.led.as_mut().map(|p| p.set_low());
    let (should_process_timers, should_process_wake, ticks_elapsed) =
        avr_device::interrupt::free(|t| {
            TICKS_ELAPSED += TICKS_PER_COUNT;
            (
                timer_queue::mark_in_progress(t),
                wake_queue::mark_in_progress(t),
                TICKS_ELAPSED,
            )
        });

    if should_process_timers {
        timer_queue::process(ticks_elapsed);
    }
    if should_process_wake {
        wake_queue::process(ticks_elapsed);
    }

    avr_device::interrupt::free(|t| {
        timer_queue::mark_finished(t);
        wake_queue::mark_finished(t);
    });
    state.counter.clear_interrupt();
    let _ = state.led.as_mut().map(|p| p.set_high());
}

pub fn init_system_time(
    tc: RTC,
    p: Option<Pin<Portc, atxtiny_hal::gpio::U<0>, Output<Stateful>>>,
) {
    unsafe {
        avr_device::interrupt::enable();
        avr_device::interrupt::free(|_| {
            TICKS_ELAPSED = 0;

            let mut pit = Pit::from_rtc(tc, RTCClockSource::OSCULP32K_1K, PERIOD_A::CYC32);
            pit.enable_interrupt();
            pit.start();

            INTERRUPT_STATE = MaybeUninit::new(InterruptState { counter: pit, led: p });
        });
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: AvrTc0EmbassyTimeDriver = AvrTc0EmbassyTimeDriver{});
embassy_time_queue_driver::timer_queue_impl!(static QUEUE_DRIVER: AvrTc0EmbassyTimeDriver = AvrTc0EmbassyTimeDriver{});
