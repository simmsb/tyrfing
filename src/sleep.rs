use avr_device::attiny1616::slpctrl::ctrla::SMODE_A;
use avr_device::attiny1616::SLPCTRL;

#[export_name = "__sleep"]
unsafe fn sleep() {
    SLPCTRL::steal().ctrla().modify(|_, w| w.sen().set_bit());
    avr_device::asm::sleep();
    SLPCTRL::steal().ctrla().modify(|_, w| w.sen().clear_bit());
}

#[macro_export]
macro_rules! with_sleep_mode {
    ($mode:expr, $e:expr) => {{
        let current = crate::sleep::get_sleep_mode();
        crate::sleep::set_sleep_mode($mode);
        let result = $e;
        crate::sleep::set_sleep_mode(current);
        result
    }};
}

pub fn get_sleep_mode() -> SMODE_A {
    unsafe {
        SLPCTRL::steal()
            .ctrla()
            .read()
            .smode()
            .variant()
            .unwrap_unchecked()
    }
}

pub fn set_sleep_mode(mode: SMODE_A) {
    unsafe {
        SLPCTRL::steal()
            .ctrla()
            .modify(|_, w| w.smode().variant(mode));
    }
}
