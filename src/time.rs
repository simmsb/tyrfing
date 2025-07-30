use atxtiny_hal::timer::rtc::{Pit, RTCClockSource};
use avr_device::{
    avr32dd20::{rtc::pitctrla::PERIOD_A, RTC},
    interrupt::{CriticalSection, Mutex},
};
use core::{
    cell::{Cell, RefCell},
    task::Waker,
};
use embassy_time_driver::{AlarmHandle, Driver};
use embassy_time_queue_driver::TimerQueue;

#[cfg(feature = "time_u32")]
pub type Time = u32;
#[cfg(not(feature = "time_u32"))]
pub type Time = u64;

pub static TICKS_ELAPSED: Mutex<Cell<Time>> = Mutex::new(Cell::new(0));

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

static IN_PROGRESS: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

pub fn mark_in_progress(cs: CriticalSection) -> bool {
    !IN_PROGRESS.borrow(cs).replace(true)
}

pub fn mark_finished(cs: CriticalSection) {
    IN_PROGRESS.borrow(cs).set(false);
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
        unsafe {
            *(&raw mut ATS)
                .as_mut()
                .unwrap()
                .get_unchecked_mut(id.0 as usize) = v
        }
    }

    pub fn set_alarm(_: CriticalSection, id: NonMaxU8, a: Alarm) {
        unsafe {
            *(&raw mut ALARMS)
                .as_mut()
                .unwrap()
                .get_unchecked_mut(id.0 as usize) = a
        }
    }
}

mod wake_queue {
    use core::{cell::Cell, task::Waker};

    use avr_device::interrupt::{CriticalSection, Mutex};

    use super::{Time, QUEUE_SIZE};

    struct Entry {
        at: Time,
        waker: Waker,
    }

    static ENTRIES: [Mutex<Cell<Option<Entry>>>; QUEUE_SIZE] =
        [const { Mutex::new(Cell::new(None)) }; QUEUE_SIZE];

    pub fn allocate(cs: CriticalSection, at: Time, waker: &Waker) {
        for entry in &ENTRIES {
            let entry = entry.borrow(cs);
            let e = entry.replace(None);

            if let Some(mut e) = e {
                // waker is the same, simply store back the earliest time
                if e.waker.will_wake(waker) {
                    e.at = at.min(e.at);

                    entry.set(Some(e));
                    return;
                } else {
                    entry.set(Some(e));
                }
            }
        }

        for entry in &ENTRIES {
            let entry = entry.borrow(cs);
            let e = entry.replace(None);

            if e.is_some() {
                entry.set(e);
                continue;
            }

            entry.set(Some(Entry {
                at,
                waker: waker.clone(),
            }));

            return;
        }

        panic!("queue full");
    }

    pub fn process(ticks_elapsed: Time) {
        for entry in &ENTRIES {
            let w = avr_device::interrupt::free(|cs| {
                let entry = entry.borrow(cs);
                let e = entry.replace(None);

                if let Some(e) = e {
                    if e.at <= ticks_elapsed {
                        return Some(e.waker);
                    } else {
                        entry.set(Some(e));
                    }
                }

                None
            });

            if let Some(w) = w {
                w.wake();
            }
        }
    }
}

impl Driver for AvrTc0EmbassyTimeDriver {
    #[inline(always)]
    fn now(&self) -> Time {
        avr_hal_generic::avr_device::interrupt::free(|cs| TICKS_ELAPSED.borrow(cs).get())
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
            wake_queue::allocate(t, at, waker);
        })
    }
}

#[avr_device::interrupt(avr32dd20)]
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
        let elapsed = TICKS_ELAPSED.borrow(t).get()
            + INTERRUPT_STATE
                .borrow(t)
                .borrow()
                .as_ref()
                .unwrap()
                .ticks_per_count as u32;
        TICKS_ELAPSED.borrow(t).set(elapsed);
        (mark_in_progress(t), elapsed)
    });

    if should_process {
        timer_queue::process(ticks_elapsed);
        wake_queue::process(ticks_elapsed);
    }

    avr_device::interrupt::free(|t| {
        if should_process {
            mark_finished(t);
        }
        let mut state = INTERRUPT_STATE.borrow(t).borrow_mut();
        state.as_mut().unwrap().counter.clear_interrupt();
    });
}

pub fn init_system_time(tc: RTC) {
    unsafe {
        avr_device::interrupt::enable();
        avr_device::interrupt::free(|t| {
            TICKS_ELAPSED.borrow(t).set(0);

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
