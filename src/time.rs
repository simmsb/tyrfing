use atxtiny_hal::{
    embedded_hal::digital::OutputPin,
    gpio::{Output, Pin, Portc, Stateful},
    timer::rtc::{Pit, RTCClockSource},
};
use avr_device::{
    attiny1616::{rtc::pitctrla::PERIOD_A, RTC},
    interrupt::{CriticalSection, Mutex},
};
use core::{
    borrow::BorrowMut,
    cell::{RefCell, UnsafeCell},
    mem::MaybeUninit,
    task::Waker,
};
use embassy_time_driver::{AlarmHandle, Driver};
use embassy_time_queue_driver::TimerQueue;
use fixed::traits::LosslessTryInto;

#[cfg(feature = "time_u32")]
pub type Time = u32;
#[cfg(not(feature = "time_u32"))]
pub type Time = u64;

pub static mut TICKS_ELAPSED: Time = 0;

pub struct AvrTc0EmbassyTimeDriver {}

#[allow(dead_code)]
const TICKS_PER_COUNT: Time = 1;

const QUEUE_SIZE: usize = 10;

#[rustc_layout_scalar_valid_range_start(0)]
#[rustc_layout_scalar_valid_range_end(254)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct NonMaxU8(u8);

#[derive(Copy, Clone)]
struct Alarm {
    callback: fn(*mut ()),
    ctx: *mut (),
}

static mut IN_PROGRESS: bool = false;

pub fn mark_in_progress(_: CriticalSection) -> bool {
    unsafe { !core::ptr::replace(core::ptr::addr_of_mut!(IN_PROGRESS), true) }
}

pub fn mark_finished(_: CriticalSection) {
    unsafe {
        IN_PROGRESS = false;
    }
}

mod timer_queue {
    use avr_device::interrupt::CriticalSection;

    use super::{Alarm, NonMaxU8, Time, QUEUE_SIZE};

    fn empty_cb(_x: *mut ()) {}
    const EMPTY_ALARM: Alarm = Alarm {
        callback: empty_cb,
        ctx: core::ptr::null_mut(),
    };

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

    static mut TAKEN: [bool; QUEUE_SIZE] = [false; QUEUE_SIZE];
    static mut ATS: [Time; QUEUE_SIZE] = [0; QUEUE_SIZE];
    const X: Option<Waker> = None;
    static mut WAKERS: [Option<Waker>; QUEUE_SIZE] = [X; QUEUE_SIZE];

    pub fn allocate(_: CriticalSection, waker: &Waker) -> Option<(NonMaxU8, bool)> {
        unsafe {
            for i in 0..QUEUE_SIZE {
                if TAKEN[i] && WAKERS[i].as_ref().map_or(false, |w| w.will_wake(waker)) {
                    return Some((NonMaxU8(i as u8), true));
                }
            }
            for i in 0..QUEUE_SIZE {
                if !TAKEN[i] {
                    TAKEN[i] = true;
                    return Some((NonMaxU8(i as u8), false));
                }
            }

            None
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

    pub fn set_at(_: CriticalSection, id: NonMaxU8, v: Time, set_to_min: bool) {
        unsafe {
            let at = ATS.get_unchecked_mut(id.0 as usize);
            if set_to_min {
                *at = (*at).min(v);
            } else {
                *at = v;
            }
        }
    }

    pub fn set_waker(_: CriticalSection, id: NonMaxU8, w: Option<Waker>) {
        unsafe { *WAKERS.get_unchecked_mut(id.0 as usize) = w }
    }
}

impl Driver for AvrTc0EmbassyTimeDriver {
    #[inline(always)]
    fn now(&self) -> Time {
        avr_hal_generic::avr_device::interrupt::free(|_| unsafe { TICKS_ELAPSED })
    }

    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
        avr_hal_generic::avr_device::interrupt::free(timer_queue::allocate)
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
            let Some((id, already_present)) = wake_queue::allocate(t, waker) else {
                panic!("queue full");
            };

            wake_queue::set_waker(t, id, Some(waker.clone()));
            wake_queue::set_at(t, id, at, already_present);
        })
    }
}

#[avr_device::interrupt(attiny1616)]
unsafe fn RTC_PIT() {
    handle_tick()
}

struct InterruptState {
    pub counter: Pit,
    pub ticks_per_count: u8,
}

impl InterruptState {
    fn configure_pit(&mut self, config: PitConfig) {
        self.counter.reconfigure(config.clock_source, config.period);
        self.ticks_per_count = config.ticks_per_count;
    }
}

struct PitConfig {
    ticks_per_count: u8,
    clock_source: RTCClockSource,
    period: PERIOD_A,
}

impl PitConfig {
    const fn new(ticks_per_count: u8, clock_source: RTCClockSource, period: PERIOD_A) -> Self {
        Self {
            ticks_per_count,
            clock_source,
            period,
        }
    }
}

const TICK_CONFIG_1024: PitConfig =
    PitConfig::new(1, RTCClockSource::OSCULP32K_32K, PERIOD_A::CYC32);
const TICK_CONFIG_64: PitConfig = PitConfig::new(16, RTCClockSource::OSCULP32K_1K, PERIOD_A::CYC16);

static INTERRUPT_STATE: Mutex<RefCell<Option<InterruptState>>> = Mutex::new(RefCell::new(None));

pub fn enter_sleep_clock() {
    avr_device::interrupt::free(|t| {
        let mut state = INTERRUPT_STATE.borrow(t).borrow_mut();
        state.as_mut().unwrap().configure_pit(TICK_CONFIG_64);
    });
}

pub fn enter_wake_clock() {
    avr_device::interrupt::free(|t| {
        let mut state = INTERRUPT_STATE.borrow(t).borrow_mut();
        state.as_mut().unwrap().configure_pit(TICK_CONFIG_1024);
    });
}

#[inline(always)]
pub unsafe fn handle_tick() {
    let (should_process, ticks_elapsed) = avr_device::interrupt::free(|t| {
        let elapsed = TICKS_ELAPSED
            + INTERRUPT_STATE
                .borrow(t)
                .borrow()
                .as_ref()
                .unwrap()
                .ticks_per_count as u32;
        TICKS_ELAPSED = elapsed;
        (mark_in_progress(t), elapsed)
    });

    if should_process {
        timer_queue::process(ticks_elapsed);
        wake_queue::process(ticks_elapsed);
    }

    avr_device::interrupt::free(|t| {
        mark_finished(t);
        let mut state = INTERRUPT_STATE.borrow(t).borrow_mut();
        state.as_mut().unwrap().counter.clear_interrupt();
    });
}

pub fn init_system_time(tc: RTC) {
    unsafe {
        avr_device::interrupt::enable();
        avr_device::interrupt::free(|t| {
            TICKS_ELAPSED = 0;

            let pitconfig = TICK_CONFIG_64;

            let mut pit = Pit::from_rtc(tc, pitconfig.clock_source, pitconfig.period);
            pit.enable_interrupt();
            pit.start();

            *INTERRUPT_STATE.borrow(t).borrow_mut() = Some(InterruptState {
                counter: pit,
                ticks_per_count: pitconfig.ticks_per_count,
            });
        });
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: AvrTc0EmbassyTimeDriver = AvrTc0EmbassyTimeDriver{});
embassy_time_queue_driver::timer_queue_impl!(static QUEUE_DRIVER: AvrTc0EmbassyTimeDriver = AvrTc0EmbassyTimeDriver{});
