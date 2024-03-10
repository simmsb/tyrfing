use avr_device::attiny1616::slpctrl::ctrla::SMODE_A;
use avr_device::attiny1616::SLPCTRL;
use portable_atomic::AtomicU8;

#[export_name = "__sleep"]
unsafe fn sleep() {
    SLPCTRL::steal().ctrla().modify(|_, w| w.sen().set_bit());
    avr_device::asm::sleep();
    SLPCTRL::steal().ctrla().modify(|_, w| w.sen().clear_bit());
}

static COFFEE: AtomicU8 = AtomicU8::new(0);

pub struct Mug(bool);

impl Default for Mug {
    fn default() -> Self {
        Self::new()
    }
}

impl Mug {
    pub fn new() -> Self {
        Self(false)
    }

    pub fn caffeinate(&mut self) {
        if !self.0 {
            caffeinate();
            self.0 = true;
        }
    }

    pub fn decaffeinate(&mut self) {
        if self.0 {
            decaffeinate();
            self.0 = false;
        }
    }
}

fn caffeinate() {
    let v = COFFEE.fetch_add(1, portable_atomic::Ordering::SeqCst);
    if v == 0 {
        // use idle instead of standby, we're never wakelocked for long (only
        // during ADC measurements) unless the torch is on
        set_sleep_mode(SMODE_A::IDLE);
    }
}

fn decaffeinate() {
    let v = COFFEE.fetch_sub(1, portable_atomic::Ordering::SeqCst);
    if v == 1 {
        set_sleep_mode(SMODE_A::PDOWN);
    }
}

#[macro_export]
macro_rules! with_wakelock {
    ($e:expr) => {{
        let mut m = $crate::sleep::Mug::new();
        m.caffeinate();
        let result = $e;
        m.decaffeinate();
        result
    }};
}

pub fn set_sleep_mode(mode: SMODE_A) {
    unsafe {
        SLPCTRL::steal()
            .ctrla()
            .modify(|_, w| w.smode().variant(mode));
    }
}
