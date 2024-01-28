#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;

use atxtiny_hal::timer::{FTimer, Counter, tca::Interrupt, tca::Event};
use atxtiny_hal::gpio::{Pin, Gpiox, Ux, Output, Stateful};

use core::mem::MaybeUninit;

struct InterruptState {
    pub counter: Counter<pac::TCA0, 312500>,
    pub led: Pin<Gpiox, Ux, Output<Stateful>>,
}

static mut INTERRUPT_STATE: MaybeUninit<InterruptState> = MaybeUninit::uninit();

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Constrain a few peripherals into our HAL types
    let clkctrl = dp.CLKCTRL.constrain();

    // Configure our clocks
    let clocks = clkctrl.freeze();

    // Split the PORTB peripheral into its pins
    let b = dp.PORTB.split();

    // Grab a pin for an LED
    let led = b.pb6.into_push_pull_output();

    // Create a timer with a fixed frequency using TCA0
    // If the frequency cannot be met given the constrained prescalers of the
    // passed counter in conjunction with the clock supplying the timer peripheral
    // an error is returned.
    let t = FTimer::<_, 312500>::new(dp.TCA0, clocks).unwrap();

    // Use the now configured fixed frequency timer to create a counter
    let mut c = t.counter();

    // Enable the overflow interrupt
    c.enable_interrupt(Interrupt::Overflow);

    // Start the counter with a timeout of 100ms
    // If the timeout cannot be met given the fixed frequency, start() returns
    // an Error
    c.start(100.millis()).unwrap();

    unsafe {
        // SAFETY: Interrupts are not enabled at this point so we can safely write the global
        // variable here.  A memory barrier afterwards ensures the compiler won't reorder this
        // after any operation that enables interrupts.
        INTERRUPT_STATE = MaybeUninit::new(InterruptState {
            counter: c,
            led: led.downgrade().downgrade()
        });

        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

    // Enable the interrupts globally
    unsafe { avr_device::interrupt::enable() };

    loop { }
}

#[avr_device::interrupt(attiny817)]
fn TCA0_LUNF_OVF() {
    let state = unsafe {
        // SAFETY: We _know_ that interrupts will only be enabled after the global was
        // initialized so this ISR will never run when the state is still uninitialized.
        // Also this is the only interrupt accessing this state, so we don't need to
        // lock it with a Mutex or similar.
        &mut *INTERRUPT_STATE.as_mut_ptr()
    };

    // Clear the interrupt so it isn't triggered immediately after returning from this ISR
    state.counter.clear_event(Event::Overflow);

    // Toggle the LED
    state.led.toggle().unwrap();
}
