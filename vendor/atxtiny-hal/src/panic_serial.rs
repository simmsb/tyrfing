//! # Serial port panic handler

use ufmt::uWrite;
use core::panic::PanicInfo;

/// Called internally by the panic handler.
pub fn _print_panic<W: uWrite>(w: &mut W, info: &PanicInfo) {
    if let Some(location) = info.location() {
        _ = ufmt::uwrite!(w, "Panic at {}:{}:{}", location.file(), location.line(), location.column());
        if !cfg!(feature="fullpanic") {
            _ = uWrite::write_str(w, "\r\n");
        }
    }

    if cfg!(feature="fullpanic") {
        if let Some(message) = info.message().as_str() {
            _ = w.write_str(": ");
            _ = w.write_str(message);
            _ = w.write_str("\r\n");
        }
    }
}

/// Implements the panic handler. You need to call this for the package to work.
///
/// This macro defines the panic handler, as well as a function called `share_serial_port_with_panic`.
/// That function takes an argument of the given `$type` and returns a `&'static mut $type`.
///
#[macro_export]
macro_rules! impl_panic_handler {
    ($type:ty) => {
        static mut PANIC_PORT: Option<$type> = None;

        #[inline(never)]
        #[panic_handler]
        fn panic(info: &::core::panic::PanicInfo) -> ! {
            unsafe { avr_device::interrupt::disable() };

            if let Some(panic_port) = unsafe { PANIC_PORT.as_mut() } {
                _ = panic_port.flush();
                ::atxtiny_hal::panic_serial::_print_panic(panic_port, info);
            }
            loop {
                ::core::sync::atomic::compiler_fence(::core::sync::atomic::Ordering::SeqCst);
            }
        }

        pub fn share_serial_port_with_panic(port: $type) -> &'static mut $type {
            unsafe {
                PANIC_PORT = Some(port);
                PANIC_PORT.as_mut().unwrap()
            }
        }
    };
}
