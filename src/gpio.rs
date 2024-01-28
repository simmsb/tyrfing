use core::{hint::unreachable_unchecked, task::Poll};

use atxtiny_hal::gpio::{Edge, GpioRegExt, Input, PXx};
use avr_device::attiny1616::{PORTA, PORTB, PORTC};
use embassy_sync::waitqueue::AtomicWaker;
use futures_util::Future;

use crate::peripheral_ref::{Peripheral, PeripheralRef};

const PORTA_PIN_COUNT: usize = 8;
const PORTB_PIN_COUNT: usize = 8;
const PORTC_PIN_COUNT: usize = 6;

const NEW_AW: AtomicWaker = AtomicWaker::new();

static PORTA_WAKERS: [AtomicWaker; PORTA_PIN_COUNT] = [NEW_AW; PORTA_PIN_COUNT];
static PORTB_WAKERS: [AtomicWaker; PORTB_PIN_COUNT] = [NEW_AW; PORTB_PIN_COUNT];
static PORTC_WAKERS: [AtomicWaker; PORTC_PIN_COUNT] = [NEW_AW; PORTC_PIN_COUNT];

trait GpioInt {
    fn is_pending(&self, n: u8) -> bool;
    fn clear(&self, n: u8);
}

impl<T: GpioRegExt> GpioInt for T {
    fn is_pending(&self, n: u8) -> bool {
        self.interrupt_pending(n)
    }

    fn clear(&self, n: u8) {
        self.enable_input_buffer(n);
    }
}

// this function is 248 bytes when compiled, + 3x 100 bytes for each handler
// when inlined, each handler is only 150 bytes
#[inline(always)]
fn int_handler(gpio: &dyn GpioInt, wakers: &[AtomicWaker]) {
    for (i, w) in wakers.iter().enumerate() {
        if gpio.is_pending(i as u8) {
            w.wake();
            gpio.clear(i as u8);
        }
    }
}

#[avr_device::interrupt(attiny1616)]
unsafe fn PORTA_PORT() {
    int_handler(&*PORTA::PTR as &dyn GpioInt, PORTA_WAKERS.as_slice());
}

#[avr_device::interrupt(attiny1616)]
unsafe fn PORTB_PORT() {
    int_handler(&*PORTB::PTR as &dyn GpioInt, PORTB_WAKERS.as_slice());
}

#[avr_device::interrupt(attiny1616)]
unsafe fn PORTC_PORT() {
    int_handler(&*PORTC::PTR as &dyn GpioInt, PORTC_WAKERS.as_slice());
}

pub struct Pin<Mode: 'static>(PXx<Mode>);

impl<Mode> Peripheral for Pin<Mode> {
    type P = Pin<Mode>;

    unsafe fn clone_unchecked(&self) -> Self::P {
        core::mem::transmute_copy(self)
    }
}

impl<Mode> Pin<Mode> {
    pub fn new(p: PXx<Mode>) -> Self {
        Self(p)
    }
}

impl Pin<Input> {
    pub fn wait(&mut self, edge: Edge) -> impl Future<Output = ()> + '_ {
        InputFuture::new(self.into_ref(), edge)
    }
}

struct InputFuture<'d> {
    pin: PeripheralRef<'d, Pin<Input>>,
    waker: &'static AtomicWaker,
}

impl<'d> InputFuture<'d> {
    fn new(mut pin: PeripheralRef<'d, Pin<Input>>, edge: Edge) -> Self {
        pin.0.clear_interrupt();

        pin.0.configure_interrupt(edge);

        let wakers_table = match pin.0.port_index() {
            0 => PORTA_WAKERS.as_slice(),
            1 => PORTB_WAKERS.as_slice(),
            2 => PORTC_WAKERS.as_slice(),
            _ => unsafe { unreachable_unchecked() },
        };

        let waker = unsafe { wakers_table.get_unchecked(pin.0.pin_index() as usize) };

        Self { pin, waker }
    }
}

impl<'d> Future for InputFuture<'d> {
    type Output = ();

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        self.waker.register(cx.waker());

        if !self.pin.0.is_interrupt_enabled() {
            return Poll::Ready(());
        }

        Poll::Pending
    }
}
