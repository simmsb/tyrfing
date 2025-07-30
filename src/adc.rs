use core::{marker::PhantomData, task::Poll};

use atxtiny_hal::{
    pac::adc0::{ctrla::CONVMODE_A, ctrld::SAMPDLY_A},
    vref::{ADCReferenceVoltage, ReferenceVoltage, VrefExt},
};
use avr_device::avr32dd20::{
    adc0::{ctrla::RESSEL_A, ctrlb::SAMPNUM_A, ctrlc::PRESC_A, ctrld::INITDLY_A, muxpos::MUXPOS_A},
    ADC0, SIGROW, VREF,
};
use embassy_sync::waitqueue::AtomicWaker;
use futures_util::{Future, FutureExt};

use crate::peripheral_ref::{Peripheral, PeripheralRef};

static ADC0_WAKER: AtomicWaker = AtomicWaker::new();

#[avr_device::interrupt(avr32dd20)]
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
pub struct Temperature<T>(pub T);

impl Temperature<u16> {
    pub fn from_raw(val: u16) -> Self {
        let val = val as u32;
        let offset = unsafe { SIGROW::steal().tempsense1().read().bits() as u32 };
        let gain = unsafe { SIGROW::steal().tempsense0().read().bits() as u32 };

        let r = (offset << 4) - val;
        let r = r * gain;
        let r = r + 65536 / 8;

        Self((r >> 10) as u16)
    }

    pub const fn from_celcius(val: i16) -> Self {
        Temperature(((val + 275) as u16) * 64)
    }

    pub fn celcius(self) -> i16 {
        (self.0 / 64) as i16 - 275
    }

    pub fn smooth_with(self, smoother: &mut crate::sensing::Smoother) -> Self {
        smoother.update(self.0);

        Self(smoother.0)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Voltage<T>(pub T);

impl Voltage<u16> {
    pub const fn volts_times_50(self) -> u8 {
        ((self.0 + 64) / 128) as u8
    }

    pub const fn volts_to_adc_output(volts: f32) -> Voltage<u16> {
        Voltage((volts * 128.0 - 64.0) as u16)
    }

    pub const fn volts_times_100(self) -> u16 {
        let r = self.volts_times_50() as u16;
        r * 2
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

    pub fn read_voltage(&mut self) -> impl Future<Output = Voltage<u16>> + '_ {
        // steal vref, TODO: rework api so ReferenceVoltage holds a ref to it
        let mut vref = unsafe { VREF::steal() }.constrain();
        ADCReferenceVoltage::voltage(&mut vref, ReferenceVoltage::_1V024);

        self.adc.set_convmode(CONVMODE_A::SINGLEENDED);
        self.adc.set_resolution(RESSEL_A::_12BIT);
        self.adc.set_leftadj(true);
        self.adc.set_freerun(true);
        self.adc.set_run_standby(true);
        self.adc.set_muxpos(MUXPOS_A::TEMPSENSE);
        self.adc.set_sampnum(SAMPNUM_A::ACC16);
        self.adc.set_prescaler(PRESC_A::DIV4);

        self.adc.start();

        self.read().map(Voltage)
    }

    pub fn read_temp(&mut self) -> impl Future<Output = Temperature<u16>> + '_ {
        self.adc.set_initdelay(INITDLY_A::DLY64);
        self.adc.set_sampdelay(SAMPDLY_A::DLY10);

        // steal vref, TODO: rework api so ReferenceVoltage holds a ref to it
        let mut vref = unsafe { VREF::steal() }.constrain();
        ADCReferenceVoltage::voltage(&mut vref, ReferenceVoltage::_2V048);

        self.adc.set_convmode(CONVMODE_A::SINGLEENDED);
        self.adc.set_resolution(RESSEL_A::_12BIT);
        self.adc.set_freerun(true);
        self.adc.set_run_standby(true);
        self.adc.set_muxpos(MUXPOS_A::VDDDIV10);
        self.adc.set_sampnum(SAMPNUM_A::ACC16);
        self.adc.set_prescaler(PRESC_A::DIV4);
        self.adc.set_samplen(32);
        self.adc.start();

        self.read().map(Temperature::from_raw)
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
    type InitDelay;
    type SampDelay;
    type ConvMode;

    fn enable(&mut self, value: bool);
    fn start(&mut self);
    fn read(&self) -> u16;
    fn ready(&self) -> bool;

    fn set_freerun(&mut self, value: bool);
    fn set_run_standby(&mut self, value: bool);
    fn set_resolution(&mut self, variant: Self::Resolution);
    fn set_convmode(&mut self, variant: Self::ConvMode);
    fn set_leftadj(&mut self, on: bool);

    fn enable_interrupt(&mut self, enabled: bool);
    fn is_interrupt_enabled(&self) -> bool;
    fn clear_interrupt(&mut self);

    fn set_muxpos(&mut self, variant: Self::MuxPos);
    fn set_sampnum(&mut self, variant: Self::SampNum);
    fn set_samplen(&mut self, n: u8);

    fn set_prescaler(&mut self, variant: Self::PreScaler);

    fn set_initdelay(&mut self, variant: Self::InitDelay);
    fn set_sampdelay(&mut self, variant: Self::SampDelay);
}

impl AdcRegExt for ADC0 {
    type Resolution = RESSEL_A;
    type MuxPos = MUXPOS_A;
    type SampNum = SAMPNUM_A;
    type PreScaler = PRESC_A;
    type InitDelay = INITDLY_A;
    type SampDelay = SAMPDLY_A;
    type ConvMode = CONVMODE_A;

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

    fn set_convmode(&mut self, variant: Self::ConvMode) {
        self.ctrla().modify(|_, w| w.convmode().variant(variant))
    }

    fn set_leftadj(&mut self, on: bool) {
        self.ctrla().modify(|_, w| w.leftadj().variant(on))
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

    fn set_samplen(&mut self, value: u8) {
        self.sampctrl().modify(|_, w| w.samplen().variant(value))
    }

    fn set_prescaler(&mut self, variant: Self::PreScaler) {
        self.ctrlc().modify(|_, w| w.presc().variant(variant))
    }

    fn set_initdelay(&mut self, variant: Self::InitDelay) {
        self.ctrld().modify(|_, w| w.initdly().variant(variant))
    }

    fn set_sampdelay(&mut self, variant: Self::SampDelay) {
        self.ctrld().modify(|_, w| w.sampdly().variant(variant))
    }
}
