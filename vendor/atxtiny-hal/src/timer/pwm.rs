use super::{Instance, WithPwm, Timer, FTimer, Error};

use fugit::TimerDurationU32;

use crate::time::Hertz;

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

/// The portmux returns a `WaveformOutputPinset` for muxed pins to be
/// used as PWM waveform output pins. What pins can be muxed into a waveform
/// output pin depends on the specific chip.
pub trait WaveformOutputPinset<TCA, const CHAN: u8> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Channel {
    C1 = 0,
    C2 = 1,
    C3 = 2,
}

pub struct Ch<const C: u8>;
pub const C1: u8 = 0;
pub const C2: u8 = 1;
pub const C3: u8 = 2;

pub trait Pins<TIM, P> {
    const C1: bool = false;
    const C2: bool = false;
    const C3: bool = false;
    type Channels;

    fn check_used(c: Channel) -> Channel {
        if (c == Channel::C1 && Self::C1)
            || (c == Channel::C2 && Self::C2)
            || (c == Channel::C3 && Self::C3)
        {
            c
        } else {
            panic!("Unused channel")
        }
    }

    fn split() -> Self::Channels;
}

pub struct PwmChannel<TIM, const C: u8> {
    pub(super) _tim: PhantomData<TIM>,
}

pub trait PwmPin<TIM, const C: u8> {}

macro_rules! pins_impl {
    ( $( ( $($PINX:ident),+ ), ( $($ENCHX:ident),+ ); )+ ) => {
        $(
            #[allow(unused_parens)]
            impl<TIM, $($PINX,)+> Pins<TIM, ($(Ch<$ENCHX>),+)> for ($($PINX),+)
            where
                TIM: Instance + WithPwm,
                $($PINX: PwmPin<TIM, $ENCHX>,)+
            {
                $(const $ENCHX: bool = true;)+
                type Channels = ($(PwmChannel<TIM, $ENCHX>),+);
                fn split() -> Self::Channels {
                    ($(PwmChannel::<TIM, $ENCHX>::new()),+)
                }
            }
        )+
    };
}

pins_impl!(
    (P1, P2, P3), (C1, C2, C3);
    (P2, P3), (C2, C3);
    (P1, P3), (C1, C3);
    (P1, P2), (C1, C2);
    (P3), (C3);
    (P2), (C2);
    (P1), (C1);
);

macro_rules! tuples {
    ( $( $trait:ident, ( $($PX:ident),+ ); )+ ) => {
        $(
            impl<TIM, $($PX,)+ const C: u8> $trait<TIM, C> for ($($PX),+)
            where
                $($PX: WaveformOutputPinset<TIM, C>,)+
            {
            }
        )+
    };
}

tuples! {
    WaveformOutputPinset, (P1, P2);
    WaveformOutputPinset, (P1, P2, P3);
}

impl<P, TIM, const C: u8> PwmPin<TIM, C> for P where P: WaveformOutputPinset<TIM, C> {}


pub trait PwmExt<TIM: Instance + WithPwm>
where
    Self: Sized + Instance + WithPwm,
{
    fn pwm<P, PINS, const FREQ: u32>(
        self,
        pins: PINS,
        time: TimerDurationU32<FREQ>,
        mode: Self::GenerationMode,
        clk: TIM::ClockSource,
    ) -> Result<Pwm<Self, P, PINS, FREQ>, Error>
    where
        PINS: Pins<Self, P>;

    fn pwm_hz<P, PINS>(self, pins: PINS, freq: Hertz, mode: TIM::GenerationMode, clk: TIM::ClockSource,) -> Result<PwmHz<Self, P, PINS>, Error>
    where
        PINS: Pins<Self, P>;
}

impl<TIM: Instance + WithPwm> PwmExt<TIM> for TIM
where
    Self: Sized + Instance + WithPwm,
{
    fn pwm<P, PINS, const FREQ: u32>(
        self,
        pins: PINS,
        time: TimerDurationU32<FREQ>,
        mode: Self::GenerationMode,
        clk: TIM::ClockSource,
    ) -> Result<Pwm<TIM, P, PINS, FREQ>, Error>
    where
        PINS: Pins<Self, P>,
    {
        FTimer::<Self, FREQ>::new(self, clk)?.pwm(pins, time, mode)
    }

    fn pwm_hz<P, PINS>(
        self,
        pins: PINS,
        time: Hertz,
        mode: TIM::GenerationMode,
        clk: TIM::ClockSource,
    ) -> Result<PwmHz<TIM, P, PINS>, Error>
    where
        PINS: Pins<Self, P>,
    {
        Timer::new(self, clk).pwm_hz(pins, time, mode)
    }
}

impl<TIM: Instance + WithPwm, const C: u8> PwmChannel<TIM, C> {
    pub(crate) fn new() -> Self {
        Self {
            _tim: core::marker::PhantomData,
        }
    }
}

// FIXME: implement this, needs access to timer though
impl<TIM: Instance + WithPwm, const C: u8> PwmChannel<TIM, C> {
    #[inline]
    pub fn disable(&mut self) {
        TIM::enable_channel(C, false);
    }

    #[inline]
    pub fn enable(&mut self) {
        TIM::enable_channel(C, true);
    }

    #[inline]
    pub fn get_duty(&self) -> TIM::CompareValue {
        TIM::read_compare_value(C)
    }

    #[inline]
    pub fn set_duty(&mut self, duty: TIM::CompareValue) {
        TIM::set_compare_value(C, duty);
    }

    #[inline]
    pub fn get_max_duty(&self) -> u32 {
        TIM::read_period().into() + 1
    }
}

pub struct PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    timer: Timer<TIM>,
    _pins: PhantomData<(P, PINS)>,
}

impl<TIM, P, PINS> Deref for PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    type Target = Timer<TIM>;
    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TIM, P, PINS> DerefMut for PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

impl<TIM, P, PINS> PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    pub fn split(self) -> PINS::Channels {
        PINS::split()
    }

    pub fn release(mut self) -> Timer<TIM> {
        self.tim.disable_counter();
        self.timer
    }
}

impl<TIM, P, PINS> PwmHz<TIM, P, PINS>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    #[inline]
    pub fn enable(&mut self, channel: Channel) {
        TIM::enable_channel(PINS::check_used(channel) as u8, true)
    }

    #[inline]
    pub fn disable(&mut self, channel: Channel) {
        TIM::enable_channel(PINS::check_used(channel) as u8, false)
    }

    #[inline]
    pub fn get_duty(&self, channel: Channel) -> TIM::CompareValue {
        TIM::read_compare_value(PINS::check_used(channel) as u8)
    }

    #[inline]
    pub fn set_duty(&mut self, channel: Channel, duty: TIM::CompareValue) {
        // FIXME: throw error if > than current period?
        TIM::set_compare_value(PINS::check_used(channel) as u8, duty);
    }

    pub fn get_period(&self) -> Hertz {
        let clk = self.clk;
        let psc = self.tim.read_prescaler() as u32;
        let per = TIM::read_period().into() + 1;

        TIM::get_input_clock_rate(clk) / (psc * (per + 1))
    }

    pub fn set_period(&mut self, period: Hertz) -> Result<(), Error> {
        let clk = self.clk;
        let (period, prescaler) = self.tim.calculate_period_and_prescaler::<TIM>(clk, period)?;
        self.tim.set_prescaler(prescaler);
        self.tim.set_period(period)?;
        self.tim.trigger_update();
        Ok(())
    }

    pub fn get_max_duty(&self) -> u32 {
        TIM::read_period().into() + 1
    }

    #[inline]
    pub fn disable_counter(&mut self) {
        self.tim.disable_counter();
    }

    #[inline]
    pub fn enable_counter(&mut self) {
        self.tim.enable_counter();
    }

    #[inline]
    pub fn reset_count(&mut self) {
        self.tim.reset_count();
    }
}


pub struct Pwm<TIM, P, PINS, const FREQ: u32>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    timer: FTimer<TIM, FREQ>,
    _pins: PhantomData<(P, PINS)>,
}

impl<TIM, P, PINS, const FREQ: u32> Deref for Pwm<TIM, P, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    type Target = FTimer<TIM, FREQ>;

    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl<TIM, P, PINS, const FREQ: u32> DerefMut for Pwm<TIM, P, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

impl<TIM, P, PINS, const FREQ: u32> Pwm<TIM, P, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    pub fn split(self) -> PINS::Channels {
        PINS::split()
    }

    pub fn release(mut self) -> FTimer<TIM, FREQ> {
        self.tim.disable_counter();
        self.timer
    }
}

impl<TIM, P, PINS, const FREQ: u32> Pwm<TIM, P, PINS, FREQ>
where
    TIM: Instance + WithPwm,
    PINS: Pins<TIM, P>,
{
    #[inline]
    pub fn enable(&mut self, channel: Channel) {
        TIM::enable_channel(PINS::check_used(channel) as u8, true)
    }

    #[inline]
    pub fn disable(&mut self, channel: Channel) {
        TIM::enable_channel(PINS::check_used(channel) as u8, false)
    }

    #[inline]
    pub fn get_duty(&self, channel: Channel) -> TIM::CompareValue {
        TIM::read_compare_value(PINS::check_used(channel) as u8)
    }

    #[inline]
    pub fn get_duty_time(&self, channel: Channel) -> TimerDurationU32<FREQ> {
        TimerDurationU32::from_ticks(TIM::read_compare_value(PINS::check_used(channel) as u8).into())
    }

    #[inline]
    pub fn set_duty(&mut self, channel: Channel, duty: TIM::CompareValue) {
        // FIXME: throw error if > than current period?
        TIM::set_compare_value(PINS::check_used(channel) as u8, duty);
    }

    #[inline]
    pub fn set_duty_time(&mut self, channel: Channel, duty: TimerDurationU32<FREQ>) -> Result<(), Error> {
        // FIXME: throw error if > than current period?
        Ok(TIM::set_compare_value(PINS::check_used(channel) as u8, duty.ticks().try_into().map_err(|_| Error::ImpossiblePeriod)?))
    }

    pub fn get_period(&self) -> TimerDurationU32<FREQ> {
        TimerDurationU32::from_ticks(TIM::read_period().into() + 1)
    }

    pub fn set_period(&mut self, period: TimerDurationU32<FREQ>) -> Result<(), Error> {
        let period = (period.ticks() - 1).try_into().map_err(|_| Error::ImpossiblePeriod)?;
        self.tim.set_period(period)?;
        self.tim.trigger_update();
        Ok(())
    }

    pub fn get_max_duty(&self) -> u32 {
        TIM::read_period().into() + 1
    }

    #[inline]
    pub fn disable_counter(&mut self) {
        self.tim.disable_counter();
    }

    #[inline]
    pub fn enable_counter(&mut self) {
        self.tim.enable_counter();
    }

    #[inline]
    pub fn reset_count(&mut self) {
        self.tim.reset_count();
    }
}

impl<TIM: Instance + WithPwm> Timer<TIM> {
    pub fn pwm_hz<P, PINS>(
        mut self,
        _pins: PINS,
        freq: Hertz,
        mode: TIM::GenerationMode,
    ) -> Result<PwmHz<TIM, P, PINS>, Error>
    where
        PINS: Pins<TIM, P>,
    {
        self.tim.disable_counter();
        self.tim.reset_count();
        self.tim.set_pwm_mode(mode);
        self.tim.clear_overflow();

        let (period, prescaler) = self.tim.calculate_period_and_prescaler::<TIM>(self.clk, freq)?;
        self.tim.set_prescaler(prescaler);
        self.tim.set_period(period)?;
        self.tim.trigger_update();

        self.tim.enable_counter();

        Ok(PwmHz {
            timer: self,
            _pins: PhantomData,
        })
    }
}

impl<TIM: Instance + WithPwm> Timer<TIM> {
    pub fn pwm_custom<P, PINS>(
        mut self,
        _pins: PINS,
        prescaler: u16,
        period: TIM::CounterValue,
        mode: TIM::GenerationMode,
    ) -> Result<PwmHz<TIM, P, PINS>, Error>
    where
        PINS: Pins<TIM, P>,
    {
        self.tim.disable_counter();
        self.tim.reset_count();
        self.tim.set_pwm_mode(mode);
        self.tim.clear_overflow();

        let prescaler = TIM::get_valid_prescalers(self.clk).iter()
                            .find(|e| **e == prescaler)
                            .ok_or(Error::ImpossiblePrescaler)?;
        self.tim.set_prescaler(*prescaler);
        self.tim.set_period(period)?;
        self.tim.trigger_update();

        self.tim.enable_counter();

        Ok(PwmHz {
            timer: self,
            _pins: PhantomData,
        })
    }
}

impl<TIM: Instance + WithPwm, const FREQ: u32> FTimer<TIM, FREQ> {
    pub fn pwm<P, PINS>(
        mut self,
        _pins: PINS,
        time: TimerDurationU32<FREQ>,
        mode: TIM::GenerationMode,
    ) -> Result<Pwm<TIM, P, PINS, FREQ>, Error>
    where
        PINS: Pins<TIM, P>,
    {
        // We are an FTimer, so at this point the clock source and prescaler
        // are already set up based on the target frequency in FREQ

        self.tim.disable_counter();
        self.tim.reset_count();
        self.tim.set_pwm_mode(mode);
        self.tim.clear_overflow();

        let period = (time.ticks() - 1).try_into().map_err(|_| Error::ImpossiblePeriod)?;
        self.tim.set_period(period)?;
        self.tim.trigger_update();

        self.tim.enable_counter();

        Ok(Pwm {
            timer: self,
            _pins: PhantomData,
        })
    }
}
