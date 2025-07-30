use core::{ops::RangeInclusive, task::Poll};

use atxtiny_hal::gpio::{Edge, GpioRegExt, Input};
use avr_device::avr32dd20::{PORTA, PORTC, PORTD, PORTF};
use embassy_sync::waitqueue::AtomicWaker;
use futures_util::Future;

use crate::peripheral_ref::{Peripheral, PeripheralRef};

const PORTA_PIN_COUNT: usize = 8;
const PORTC_PIN_COUNT: usize = 3;
const PORTD_PIN_COUNT: usize = 4;
const PORTF_PIN_COUNT: usize = 2;

static PORTA_WAKERS: [AtomicWaker; PORTA_PIN_COUNT] =
    [const { AtomicWaker::new() }; PORTA_PIN_COUNT];
static PORTC_WAKERS: [AtomicWaker; PORTC_PIN_COUNT] =
    [const { AtomicWaker::new() }; PORTC_PIN_COUNT];
static PORTD_WAKERS: [AtomicWaker; PORTD_PIN_COUNT] =
    [const { AtomicWaker::new() }; PORTD_PIN_COUNT];
static PORTF_WAKERS: [AtomicWaker; PORTF_PIN_COUNT] =
    [const { AtomicWaker::new() }; PORTF_PIN_COUNT];

// Helper trait used for its vtable
trait GpioInt {
    fn is_pending(&self, n: u8) -> bool;
    fn clear(&self, n: u8);
    fn pin_range(&self) -> RangeInclusive<u8>;
    fn wakers(&self) -> &[AtomicWaker];
}

trait GpioIntMeta: GpioInt {
    const PIN_RANGE: RangeInclusive<u8>;
    const WAKERS: &[AtomicWaker];
}

impl<T: GpioRegExt + GpioIntMeta> GpioInt for T {
    fn is_pending(&self, n: u8) -> bool {
        // we need this proxy method as GpioRegExt isn't object safe
        self.interrupt_pending(n)
    }

    fn clear(&self, n: u8) {
        // enabling input buffering disables the interrupt
        self.enable_input_buffer(n);
        self.clear_interrupt_pending(n);
    }

    fn pin_range(&self) -> RangeInclusive<u8> {
        return Self::PIN_RANGE;
    }

    fn wakers(&self) -> &[AtomicWaker] {
        return Self::WAKERS;
    }
}

impl GpioIntMeta for atxtiny_hal::pac::porta::RegisterBlock {
    const PIN_RANGE: RangeInclusive<u8> = 0..=7;
    const WAKERS: &[AtomicWaker] = &PORTA_WAKERS;
}

impl GpioIntMeta for atxtiny_hal::pac::portc::RegisterBlock {
    const PIN_RANGE: RangeInclusive<u8> = 1..=3;
    const WAKERS: &[AtomicWaker] = &PORTC_WAKERS;
}

impl GpioIntMeta for atxtiny_hal::pac::portd::RegisterBlock {
    const PIN_RANGE: RangeInclusive<u8> = 4..=7;
    const WAKERS: &[AtomicWaker] = &PORTD_WAKERS;
}

impl GpioIntMeta for atxtiny_hal::pac::portf::RegisterBlock {
    const PIN_RANGE: RangeInclusive<u8> = 6..=7;
    const WAKERS: &[AtomicWaker] = &PORTF_WAKERS;
}

fn int_handler(gpio: &dyn GpioInt) {
    let offset = *gpio.pin_range().start();
    for pin in gpio.pin_range() {
        if gpio.is_pending(pin) {
            gpio.wakers()[(pin - offset) as usize].wake();
            gpio.clear(pin);
        }
    }
}

#[avr_device::interrupt(avr32dd20)]
unsafe fn PORTA_PORT() {
    int_handler(&*PORTA::PTR as &dyn GpioInt);
}

#[avr_device::interrupt(avr32dd20)]
unsafe fn PORTC_PORT() {
    int_handler(&*PORTC::PTR as &dyn GpioInt);
}

#[avr_device::interrupt(avr32dd20)]
unsafe fn PORTD_PORT() {
    int_handler(&*PORTD::PTR as &dyn GpioInt);
}

#[avr_device::interrupt(avr32dd20)]
unsafe fn PORTF_PORT() {
    int_handler(&*PORTF::PTR as &dyn GpioInt);
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

    pub fn pin(&mut self) -> &mut atxtiny_hal::gpio::Pin<Gpio, Index, Mode> {
        &mut self.0
    }
}

impl<Gpio: atxtiny_hal::gpio::marker::Gpio, Index: atxtiny_hal::gpio::marker::Index>
    Pin<Gpio, Index, Input>
{
    pub fn wait(&mut self, edge: Edge) -> impl Future<Output = ()> + '_ {
        InputFuture::new(self.into_ref(), edge)
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

struct StaticGpioMeta {
    wakers: &'static [AtomicWaker],
    pin_range: RangeInclusive<u8>,
}

const fn gpio_meta<T: GpioIntMeta>() -> StaticGpioMeta {
    let wakers = T::WAKERS;
    let pin_range = T::PIN_RANGE;

    StaticGpioMeta { wakers, pin_range }
}

static PORT_MAP: [Option<StaticGpioMeta>; 6] = unsafe {
    [
        Some(gpio_meta::<atxtiny_hal::pac::porta::RegisterBlock>()),
        None,
        Some(gpio_meta::<atxtiny_hal::pac::portc::RegisterBlock>()),
        Some(gpio_meta::<atxtiny_hal::pac::portd::RegisterBlock>()),
        None,
        Some(gpio_meta::<atxtiny_hal::pac::portf::RegisterBlock>()),
    ]
};

impl<'d, Gpio: atxtiny_hal::gpio::marker::Gpio, Index: atxtiny_hal::gpio::marker::Index> Future
    for InputFuture<'d, Gpio, Index>
{
    type Output = ();

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let pin_idx = self.pin.0.pin_index();
        let meta = PORT_MAP[self.pin.0.port_index() as usize].as_ref().unwrap();
        let waker = &meta.wakers[(pin_idx - meta.pin_range.start()) as usize];

        waker.register(cx.waker());

        if !self.pin.0.is_interrupt_enabled() {
            return Poll::Ready(());
        }

        Poll::Pending
    }
}
