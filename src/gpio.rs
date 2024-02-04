use core::{hint::unreachable_unchecked, task::Poll};

use atxtiny_hal::{
    embedded_hal::digital::InputPin,
    gpio::{Edge, GpioRegExt, Input, PXx},
};
use avr_device::attiny1616::{PORTA, PORTB, PORTC};
use avr_hal_generic::prelude::_unwrap_infallible_UnwrapInfallible;
use embassy_sync::waitqueue::AtomicWaker;
use futures_util::Future;

use crate::peripheral_ref::{Peripheral, PeripheralRef};

const PORTA_PIN_COUNT: usize = 8;
const PORTB_PIN_COUNT: usize = 8;
const PORTC_PIN_COUNT: usize = 6;

const NEW_AW: AtomicWaker = AtomicWaker::new();

static WAKERS: [AtomicWaker; PORTA_PIN_COUNT + PORTB_PIN_COUNT + PORTC_PIN_COUNT] =
    [NEW_AW; PORTA_PIN_COUNT + PORTB_PIN_COUNT + PORTC_PIN_COUNT];

fn get_waker(port: u8, pin: u8) -> &'static AtomicWaker {
    unsafe {
        // hah
        WAKERS.get_unchecked((port * PORTA_PIN_COUNT as u8 + pin) as usize)
    }
}

trait GpioInt {
    fn is_pending(&self, n: u8) -> bool;
    fn clear(&self, n: u8);
}

impl<T: GpioRegExt> GpioInt for T {
    fn is_pending(&self, n: u8) -> bool {
        self.interrupt_pending(n)
    }

    fn clear(&self, n: u8) {
        // enabling input buffering disables the interrupt
        self.enable_input_buffer(n);
        self.clear_interrupt_pending(n);
    }
}

fn int_handler(gpio: &dyn GpioInt, port: u8, pin_count: u8) {
    for i in 0..pin_count {
        if gpio.is_pending(i) {
            get_waker(port, i).wake();
            gpio.clear(i);
        }
    }
}

#[avr_device::interrupt(attiny1616)]
unsafe fn PORTA_PORT() {
    int_handler(&*PORTA::PTR as &dyn GpioInt, 0, PORTA_PIN_COUNT as u8);
}

#[avr_device::interrupt(attiny1616)]
unsafe fn PORTB_PORT() {
    int_handler(&*PORTB::PTR as &dyn GpioInt, 1, PORTB_PIN_COUNT as u8);
}

#[avr_device::interrupt(attiny1616)]
unsafe fn PORTC_PORT() {
    int_handler(&*PORTC::PTR as &dyn GpioInt, 2, PORTC_PIN_COUNT as u8);
}

pub struct Pin<Gpio, Index, Mode: 'static>(atxtiny_hal::gpio::Pin<Gpio, Index, Mode>);

impl<Gpio, Index, Mode> Peripheral for Pin<Gpio, Index, Mode> {
    type P = Pin<Gpio, Index, Mode>;

    unsafe fn clone_unchecked(&self) -> Self::P {
        core::mem::transmute_copy(self)
    }
}

impl<Gpio, Index, Mode> Pin<Gpio, Index, Mode> {
    pub fn new(p: atxtiny_hal::gpio::Pin<Gpio, Index, Mode>) -> Self {
        Self(p)
    }
}

impl<Gpio: atxtiny_hal::gpio::marker::Gpio, Index: atxtiny_hal::gpio::marker::Index>
    Pin<Gpio, Index, Input>
{
    pub async fn wait(&mut self, edge: Edge) {
        let is_high = self.0.is_high().unwrap_infallible();
        if match edge {
            Edge::Rising => is_high,
            Edge::Falling => !is_high,
            Edge::RisingFalling => false,
            Edge::LowLevel => !is_high,
        } {
            return;
        }

        InputFuture::new(self.into_ref(), edge).await;
    }

    pub fn wait_high(&mut self) -> impl Future<Output = ()> + '_ {
        self.wait(Edge::Rising)
    }

    pub fn wait_low(&mut self) -> impl Future<Output = ()> + '_ {
        self.wait(Edge::Falling)
    }
}

struct InputFuture<'d, Gpio, Index> {
    pin: PeripheralRef<'d, Pin<Gpio, Index, Input>>,
    // waker: &'static AtomicWaker,
}

impl<'d, Gpio: atxtiny_hal::gpio::marker::Gpio, Index: atxtiny_hal::gpio::marker::Index>
    InputFuture<'d, Gpio, Index>
{
    fn new(mut pin: PeripheralRef<'d, Pin<Gpio, Index, Input>>, edge: Edge) -> Self {
        pin.0.clear_interrupt();

        pin.0.configure_interrupt(edge);

        Self { pin }
    }
}

impl<'d, Gpio: atxtiny_hal::gpio::marker::Gpio, Index: atxtiny_hal::gpio::marker::Index> Future
    for InputFuture<'d, Gpio, Index>
{
    type Output = ();

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let pin_idx = self.pin.0.pin_index();
        let waker = get_waker(self.pin.0.port_index(), pin_idx);

        waker.register(cx.waker());

        if !self.pin.0.is_interrupt_enabled() {
            return Poll::Ready(());
        }

        Poll::Pending
    }
}
