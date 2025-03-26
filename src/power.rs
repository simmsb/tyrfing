use crate::{
    nonatomic::{NonAtomicBool, NonAtomicU8},
    power_levels::PathLevel,
};
use atxtiny_hal::{
    dac::{Dac, Enabled},
    embedded_hal::digital::OutputPin,
    gpio::{Input, Output, Pin, Porta, Portb, Stateless, U},
    vref::{DACReferenceVoltage, ReferenceVoltage, VrefExt},
};
use avr_device::attiny1616::{DAC0, VREF};
use avr_hal_generic::prelude::*;

use crate::{
    adc::{Temperature, Voltage},
    sleep::Mug,
};

static DESIRED_LEVEL: NonAtomicU8 = NonAtomicU8::new(0);
static GRADUAL_LEVEL: NonAtomicU8 = NonAtomicU8::new(0);
static TURBOED: NonAtomicBool = NonAtomicBool::new(false);

static POKE_POWER_CONTROLLER: embassy_sync::signal::Signal<
    embassy_sync::blocking_mutex::raw::ThreadModeRawMutex,
    (),
> = embassy_sync::signal::Signal::new();

pub fn poke_power_controller() {
    POKE_POWER_CONTROLLER.signal(());
}

pub async fn blink(blinks: u8) {
    // TODO: calculate blink level off current level
    let current_level = DESIRED_LEVEL.load();

    for _ in 0..blinks {
        set_level(40);
        embassy_time::Timer::after_millis(100).await;
        set_level(current_level);
        embassy_time::Timer::after_millis(100).await;
    }
}

pub fn set_level(value: u8) {
    DESIRED_LEVEL.store(value);
    GRADUAL_LEVEL.store(value);
}

pub fn set_level_gradual(value: u8) {
    GRADUAL_LEVEL.store(value);
}

pub struct PowerPaths {
    _vref: DACReferenceVoltage<0>,
    dac: Dac<DAC0, Enabled>,
    path1: Pin<Porta, U<7>, Output<Stateless>>,
    path2: Pin<Portb, U<5>, Output<Stateless>>,
    path3: Pin<Portb, U<4>, Output<Stateless>>,
    buck: Pin<Portb, U<3>, Output<Stateless>>,
    fet: Pin<Portb, U<2>, Output<Stateless>>,
    wakelock: Mug,
    buck_is_on: bool,
}

impl PowerPaths {
    pub fn new(
        vref: DACReferenceVoltage<0>,
        mut dac: Dac<DAC0, Enabled>,
        path1: atxtiny_hal::gpio::PA7<Input>,
        path2: atxtiny_hal::gpio::PB5<Input>,
        path3: atxtiny_hal::gpio::PB4<Input>,
        buck: atxtiny_hal::gpio::PB3<Input>,
        fet: atxtiny_hal::gpio::PB2<Input>,
    ) -> Self {
        dac.run_in_standby(true);
        let mut s = Self {
            _vref: vref,
            dac,
            path1: path1.into_stateless_push_pull_output(),
            path2: path2.into_stateless_push_pull_output(),
            path3: path3.into_stateless_push_pull_output(),
            buck: buck.into_stateless_push_pull_output(),
            fet: fet.into_stateless_push_pull_output(),
            wakelock: Mug::new(),
            buck_is_on: false,
        };
        s.off();
        s
    }

    fn off(&mut self) {
        self.fet.set_low().unwrap_infallible();
        self.buck.set_low().unwrap_infallible();
        self.path1.set_low().unwrap_infallible();
        self.path2.set_low().unwrap_infallible();
        self.path3.set_low().unwrap_infallible();
        self.buck_is_on = false;
    }

    #[cfg(feature = "has_fet")]
    fn turbo_level(&mut self) {
        self.buck.set_low().unwrap_infallible();
        self.fet.set_high().unwrap_infallible();
        self.path1.set_high().unwrap_infallible();
        self.path2.set_high().unwrap_infallible();
        self.path3.set_high().unwrap_infallible();
        self.buck_is_on = false;
    }

    fn buck_level(&mut self) {
        self.fet.set_low().unwrap_infallible();
        self.buck.set_high().unwrap_infallible();
        self.buck_is_on = true;
    }

    fn set_vref(&mut self, vref: ReferenceVoltage) {
        let mut vref_p = unsafe { VREF::steal() }.constrain();
        DACReferenceVoltage::voltage(&mut vref_p, vref);
    }

    fn set_path(&mut self, path: PathLevel) {
        let (one, two, three) = match path {
            PathLevel::One => (true, false, false),
            PathLevel::Two => (true, true, false),
            PathLevel::Three => (true, true, true),
        };

        self.path1.set_state(one.into()).unwrap_infallible();
        self.path2.set_state(two.into()).unwrap_infallible();
        self.path3.set_state(three.into()).unwrap_infallible();
    }

    async fn set_power_level(&mut self, level: u8) {
        if level == 0 {
            self.off();
            self.wakelock.decaffeinate();
        } else if level == 255 {
            #[cfg(feature = "has_fet")]
            self.turbo_level();
            self.wakelock.caffeinate();
        } else {
            self.wakelock.caffeinate();
            let config = crate::power_levels::OUTPUT_LEVELS.load_at(level as usize);

            if !self.buck_is_on {
                self.buck_level();
                embassy_time::Timer::after_millis(8).await;
            }

            self.dac.dac_set_value(config.dac_level);
            self.set_vref(config.path.reference());
            self.set_path(config.path.path_level());
        }
    }
}

const INSTANT_STOP_TEMP: Temperature<u16> = Temperature::kelvin_times_64_from_celcius(60);
const MAX_TEMP: Temperature<u16> = Temperature::kelvin_times_64_from_celcius(50);
const MIN_VOLTS: Voltage<u16> = Voltage::volts_to_adc_output(3.0);
const INSTANT_STOP_VOLTS: Voltage<u16> = Voltage::volts_to_adc_output(2.6);

fn delta(desired_level: u8, gradual_level: u8) -> u8 {
    let abs_diff = desired_level.abs_diff(gradual_level);

    if abs_diff < 10 {
        1
    } else if abs_diff < 30 {
        3
    } else {
        abs_diff / 8
    }
}

#[embassy_executor::task]
pub async fn power_controller(mut paths: PowerPaths) {
    let mut previous_level = 0u8;

    let mut accumulated_over_temp = 0u32;
    let mut accumulated_under_volts = 0u32;
    let mut tick_this_time = true;

    loop {
        if tick_this_time {
            let gradual_level = GRADUAL_LEVEL.load();
            let desired_level = DESIRED_LEVEL.load();

            let delta = delta(desired_level, gradual_level);

            let desired_level = if desired_level < gradual_level {
                desired_level + delta
            } else if desired_level > gradual_level {
                desired_level - delta
            } else {
                desired_level
            };
            DESIRED_LEVEL.store(desired_level);
        }

        let mut actual_level = DESIRED_LEVEL.load();

        let volts = crate::sensing::VOLTAGE.get();

        if volts < INSTANT_STOP_VOLTS {
            actual_level = 0;
        }

        let temp = crate::sensing::TEMPERATURE.get().kelvin_times_64();

        if temp > INSTANT_STOP_TEMP.0 {
            actual_level = 0;
        }

        if tick_this_time {
            if TURBOED.replace(false) {
                accumulated_over_temp = 0;
                accumulated_under_volts = 0;
            }

            let temp_diff = (temp as i32) - (MAX_TEMP.0 as i32);

            accumulated_over_temp = accumulated_over_temp.saturating_add_signed(temp_diff);

            accumulated_under_volts = accumulated_under_volts
                .saturating_add_signed(if volts < MIN_VOLTS { 1 } else { -1 });
        }

        const TICKS_PER_SEC: u32 = 100;
        let power_decrease = accumulated_under_volts
            .saturating_add((accumulated_over_temp) / (64 * TICKS_PER_SEC * 2));

        // decrease level by 1 for every two seconds of 1c over temp

        actual_level = actual_level.saturating_sub(power_decrease as u8);

        if actual_level != previous_level {
            previous_level = actual_level;

            paths.set_power_level(actual_level).await;
        }

        tick_this_time = crate::with_timeout::with_timeout(
            Some(embassy_time::Duration::from_hz(TICKS_PER_SEC)),
            POKE_POWER_CONTROLLER.wait(),
        )
        .await
        .is_err();
    }
}
