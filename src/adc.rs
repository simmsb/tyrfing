use core::{marker::PhantomData, task::Poll};

use atxtiny_hal::vref::{ADCReferenceVoltage, ReferenceVoltage, VrefExt};
use avr_device::attiny1616::{
    adc0::{
        ctrla::RESSEL_A,
        ctrlb::SAMPNUM_A,
        ctrlc::{PRESC_A, REFSEL_A},
        ctrld::INITDLY_A,
        muxpos::MUXPOS_A,
    },
    ADC0, SIGROW, VREF,
};
use embassy_sync::waitqueue::AtomicWaker;
use futures_util::{Future, FutureExt};

use crate::peripheral_ref::{Peripheral, PeripheralRef};

static ADC0_WAKER: AtomicWaker = AtomicWaker::new();

#[avr_device::interrupt(attiny1616)]
unsafe fn ADC0_RESRDY() {
    let mut adc = ADC0::steal();

    // clear interrupt
    adc.clear_interrupt();

    // disable interrupt
    adc.enable_interrupt(false);

    ADC0_WAKER.wake();
}

pub struct Disabled;
pub struct Enabled;

pub trait ED {}
impl ED for Disabled {}
impl ED for Enabled {}

pub trait AdcExt<INST: AdcRegExt, const IDX: u8> {
    fn constrain(self, ref_voltage: ADCReferenceVoltage<IDX>) -> Adc<INST, Disabled>;
}

pub struct Adc<INST, ED> {
    adc: INST,
    ref_: ADCReferenceVoltage<0>,
    _enabled: PhantomData<ED>,
}

impl AdcExt<ADC0, 0> for ADC0 {
    fn constrain(self, ref_voltage: ADCReferenceVoltage<0>) -> Adc<ADC0, Disabled> {
        Adc {
            adc: self,
            ref_: ref_voltage,
            _enabled: PhantomData,
        }
    }
}

impl<INST, ED> Peripheral for Adc<INST, ED> {
    type P = Adc<INST, ED>;

    unsafe fn clone_unchecked(&self) -> Self::P {
        core::mem::transmute_copy(self)
    }
}

impl<INST: AdcRegExt> Adc<INST, Disabled> {
    pub fn enable(mut self) -> Adc<INST, Enabled> {
        self.adc.enable(true);

        Adc {
            adc: self.adc,
            ref_: self.ref_,
            _enabled: PhantomData,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Temperature(pub u16);

impl Temperature {
    pub fn kelvin_times_64(self) -> u16 {
        let r = self.0 << 4;
        let offset = -((unsafe { SIGROW::steal() }.tempsense1().read().bits() as i8 as i32) << 6);
        let gain = unsafe { SIGROW::steal() }.tempsense0().read().bits() as u32;

        let r = r as u32;
        let r = r.saturating_add_signed(offset);
        let r = r * gain as u32;
        let r = r + 65536 / 8;
        let r = r >> 8;
        r as u16
    }

    pub fn celcius(self) -> i16 {
        (self.kelvin_times_64() >> 6) as i16 - 275
    }

    pub fn smooth_with(self, smoother: &mut crate::sensing::Smoother) -> Self {
        smoother.update(self.0);

        Self(smoother.0)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Voltage(pub u16);

impl Voltage {
    pub fn volts_times_40(self) -> u8 {
        let r = self.0 << 4;
        const NUMERATOR: u32 = (40.0 * 1.5 * 4096.0) as u32;

        (NUMERATOR / (r >> 4) as u32) as u8
    }

    pub fn volts_times_100(self) -> u16 {
        let r = self.volts_times_40() as u16;
        r * 2 + r / 2
    }

    pub fn smooth_with(self, smoother: &mut crate::sensing::Smoother) -> Self {
        smoother.update(self.0);

        Self(smoother.0)
    }
}

impl Adc<ADC0, Enabled> {
    pub fn disable(mut self) -> Adc<ADC0, Disabled> {
        self.adc.enable(false);

        Adc {
            adc: self.adc,
            ref_: self.ref_,
            _enabled: PhantomData,
        }
    }

    pub fn run_in_standby(&mut self, on: bool) {
        self.adc.set_run_standby(on);
    }

    fn read(&mut self) -> impl Future<Output = u16> + '_ {
        AdcFuture::new(self.into_ref())
    }

    pub fn read_voltage(&mut self) -> impl Future<Output = Voltage> + '_ {
        self.adc.set_initdelay(INITDLY_A::DLY16);

        // steal vref, TODO: rework api so ReferenceVoltage holds a ref to it
        let mut vref = unsafe { VREF::steal() }.constrain();
        ADCReferenceVoltage::voltage(&mut vref, ReferenceVoltage::_1V50);

        self.adc.set_muxpos(MUXPOS_A::INTREF);
        self.adc.set_sampnum(SAMPNUM_A::ACC4);

        self.adc.set_c_state(PRESC_A::DIV16, REFSEL_A::VDDREF, true);

        self.adc.start();

        self.read().map(Voltage)
    }

    pub fn read_temp(&mut self) -> impl Future<Output = Temperature> + '_ {
        self.adc.set_initdelay(INITDLY_A::DLY16);

        // steal vref, TODO: rework api so ReferenceVoltage holds a ref to it
        let mut vref = unsafe { VREF::steal() }.constrain();
        ADCReferenceVoltage::voltage(&mut vref, ReferenceVoltage::_1V10);

        self.adc.set_muxpos(MUXPOS_A::TEMPSENSE);
        self.adc.set_sampnum(SAMPNUM_A::ACC4);

        self.adc.set_c_state(PRESC_A::DIV16, REFSEL_A::INTREF, true);

        self.adc.start();

        self.read().map(Temperature)
    }
}

struct AdcFuture<'d, INST> {
    adc: PeripheralRef<'d, Adc<INST, Enabled>>,
}

impl<'d, INST: AdcRegExt> AdcFuture<'d, INST> {
    fn new(mut adc: PeripheralRef<'d, Adc<INST, Enabled>>) -> Self {
        adc.adc.clear_interrupt();
        adc.adc.enable_interrupt(true);

        Self { adc }
    }
}

impl<'d, INST: AdcRegExt> Future for AdcFuture<'d, INST> {
    type Output = u16;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        ADC0_WAKER.register(cx.waker());

        if !self.adc.adc.is_interrupt_enabled() {
            let value = self.adc.adc.read();
            return Poll::Ready(value);
        }

        Poll::Pending
    }
}

pub trait AdcRegExt {
    type Resolution;
    type MuxPos;
    type SampNum;
    type PreScaler;
    type RefSel;
    type InitDelay;

    fn enable(&mut self, value: bool);
    fn start(&mut self);
    fn read(&self) -> u16;
    fn ready(&self) -> bool;

    fn set_freerun(&mut self, value: bool);
    fn set_run_standby(&mut self, value: bool);
    fn set_resolution(&mut self, variant: Self::Resolution);

    fn enable_interrupt(&mut self, enabled: bool);
    fn is_interrupt_enabled(&self) -> bool;
    fn clear_interrupt(&mut self);

    fn set_muxpos(&mut self, variant: Self::MuxPos);
    fn set_sampnum(&mut self, variant: Self::SampNum);

    fn set_c_state(&mut self, prescaler: Self::PreScaler, refsel: Self::RefSel, sampcap: bool);
    fn set_prescaler(&mut self, variant: Self::PreScaler);
    fn set_refsel(&mut self, variant: Self::RefSel);
    fn set_sampcap(&mut self, value: bool);

    fn set_initdelay(&mut self, variant: Self::InitDelay);
}

impl AdcRegExt for ADC0 {
    type Resolution = RESSEL_A;
    type MuxPos = MUXPOS_A;
    type SampNum = SAMPNUM_A;
    type PreScaler = PRESC_A;
    type RefSel = REFSEL_A;
    type InitDelay = INITDLY_A;

    fn enable(&mut self, value: bool) {
        self.ctrla().modify(|_, w| w.enable().variant(value))
    }

    fn start(&mut self) {
        self.command().modify(|_, w| w.stconv().set_bit())
    }

    fn read(&self) -> u16 {
        self.res().read().bits()
    }

    fn ready(&self) -> bool {
        self.command().read().stconv().bit_is_set()
    }

    fn set_freerun(&mut self, value: bool) {
        self.ctrla().modify(|_, w| w.freerun().variant(value))
    }

    fn set_run_standby(&mut self, value: bool) {
        self.ctrla().modify(|_, w| w.runstby().variant(value))
    }

    fn set_resolution(&mut self, variant: Self::Resolution) {
        self.ctrla().modify(|_, w| w.ressel().variant(variant))
    }

    fn enable_interrupt(&mut self, enabled: bool) {
        self.intctrl().modify(|_, w| w.resrdy().variant(enabled))
    }

    fn is_interrupt_enabled(&self) -> bool {
        self.intctrl().read().resrdy().bit_is_set()
    }

    fn clear_interrupt(&mut self) {
        self.intflags().modify(|_, w| w.resrdy().set_bit());
    }

    fn set_muxpos(&mut self, variant: Self::MuxPos) {
        self.muxpos().modify(|_, w| w.muxpos().variant(variant))
    }

    fn set_sampnum(&mut self, variant: Self::SampNum) {
        self.ctrlb().modify(|_, w| w.sampnum().variant(variant))
    }

    fn set_prescaler(&mut self, variant: Self::PreScaler) {
        self.ctrlc().modify(|_, w| w.presc().variant(variant))
    }

    fn set_refsel(&mut self, variant: Self::RefSel) {
        self.ctrlc().modify(|_, w| w.refsel().variant(variant))
    }

    fn set_sampcap(&mut self, value: bool) {
        self.ctrlc().modify(|_, w| w.sampcap().variant(value))
    }

    fn set_c_state(&mut self, prescaler: Self::PreScaler, refsel: Self::RefSel, sampcap: bool) {
        self.ctrlc().modify(|_, w| {
            w.presc()
                .variant(prescaler)
                .refsel()
                .variant(refsel)
                .sampcap()
                .variant(sampcap)
        })
    }

    fn set_initdelay(&mut self, variant: Self::InitDelay) {
        self.ctrld().modify(|_, w| w.initdly().variant(variant))
    }
}
