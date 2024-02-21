use crate::nonatomic::NonAtomicU16;
use atxtiny_hal::gpio::Input;
use atxtiny_hal::prelude::*;
use atxtiny_hal::watchdog::WatchdogTimer;
use avr_device::attiny1616::ADC0;
use avr_hal_generic::prelude::*;

use crate::adc::{self, Temperature, Voltage};

pub static VOLTAGE: Voltage<NonAtomicU16> = Voltage(NonAtomicU16::new(0));
pub static TEMPERATURE: Temperature<NonAtomicU16> = Temperature(NonAtomicU16::new(0));

impl Voltage<NonAtomicU16> {
    fn set(&self, value: Voltage<u16>) {
        self.0.store(value.0);
    }

    pub fn get(&self) -> Voltage<u16> {
        Voltage(self.0.load())
    }
}

impl Temperature<NonAtomicU16> {
    fn set(&self, value: Temperature<u16>) {
        self.0.store(value.0);
    }

    pub fn get(&self) -> Temperature<u16> {
        Temperature(self.0.load())
    }
}

pub struct Smoother(pub u16);

impl Smoother {
    pub fn update(&mut self, value: u16) {
        // self.0 = value;
        let diff = (value / 8) as i16 - (self.0 / 8) as i16;

        self.0 = self.0.wrapping_add_signed(diff);
    }
}

#[embassy_executor::task]
pub async fn watchdock_tickler(
    mut wd: WatchdogTimer,
    p: atxtiny_hal::gpio::PC0<Input>,
    mut adc: adc::Adc<ADC0, adc::Disabled>,
) {
    let mut p = p.into_push_pull_output();
    p.set_high().unwrap_infallible();

    let mut temp_smoother = Smoother(1970);
    let mut volt_smoother = Smoother(1380); // 5.2v (?)

    loop {
        let mut adc_ = adc.enable();
        adc_.run_in_standby(true);

        let (t, v) = crate::with_wakelock!({
            let t = adc_.read_temp().await.smooth_with(&mut temp_smoother);
            let v = adc_.read_voltage().await.smooth_with(&mut volt_smoother);

            (t, v)
        });

        TEMPERATURE.set(t);
        VOLTAGE.set(v);

        adc = adc_.disable();
        wd.feed();

        #[cfg(feature = "logging")]
        crate::serial_println!(
            "Temp: {} ({}), Volts: {} ({})|||",
            t.celcius(),
            t.0,
            v.volts_times_100(),
            v.0
        );

        p.toggle().unwrap_infallible();

        embassy_time::Timer::after_millis(500).await;
    }
}
