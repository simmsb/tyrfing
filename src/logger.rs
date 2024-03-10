#[cfg(feature = "logging")]
pub static SERIAL: avr_device::interrupt::Mutex<
    core::cell::RefCell<
        Option<
            atxtiny_hal::serial::Serial<
                avr_device::attiny1616::USART0,
                atxtiny_hal::serial::UartPinset<
                    avr_device::attiny1616::USART0,
                    atxtiny_hal::gpio::Pin<
                        atxtiny_hal::gpio::Porta,
                        atxtiny_hal::gpio::U<2>,
                        atxtiny_hal::gpio::Input,
                    >,
                    atxtiny_hal::gpio::Pin<
                        atxtiny_hal::gpio::Porta,
                        atxtiny_hal::gpio::U<1>,
                        atxtiny_hal::gpio::Output<atxtiny_hal::gpio::Stateless>,
                    >,
                >,
            >,
        >,
    >,
> = avr_device::interrupt::Mutex::new(core::cell::RefCell::new(None));

#[macro_export]
macro_rules! serial_println {
    ($($arg:expr),*) => {{
        #[cfg(feature = "logging")]
        avr_device::interrupt::free(|t| {
            let s = &mut *$crate::logger::SERIAL.borrow(t).borrow_mut();
            if let Some(w) = s.as_mut() {
                let _ = ::ufmt::uwriteln!(w, $($arg),*);
                use embedded_io::Write;
                let _ = w.flush();
            }
        })
    }}
}
