use atxtiny_hal::{
    clkctrl::Clocks,
    gpio::{self, Input},
    pac::{portmux, TCA0},
    portmux::{Portmux, PortmuxTCARoutes, TCA0PortA},
    prelude::*,
    timer::tca::TcaPinset,
};
use avr_hal_generic::prelude::_unwrap_infallible_UnwrapInfallible;
use cichlid::{color_codes::Brown, ColorRGB};

use crate::{adc::Voltage, sleep::Mug};

#[derive(Clone, Copy)]
struct Rgb1Bit {
    r: bool,
    g: bool,
    b: bool,
}

impl Rgb1Bit {
    fn new(r: bool, g: bool, b: bool) -> Self {
        Self { r, g, b }
    }

    fn to_colorrgb(&self) -> ColorRGB {
        const ON_LEVEL: u8 = 40;

        ColorRGB::new(
            if self.r { ON_LEVEL } else { 0 },
            if self.g { ON_LEVEL } else { 0 },
            if self.b { ON_LEVEL } else { 0 },
        )
    }
}

struct AuxLeds {
    pwm1: atxtiny_hal::timer::Pwm<
        TCA0,
        (
            atxtiny_hal::timer::Ch<{ atxtiny_hal::timer::C2 }>,
            atxtiny_hal::timer::Ch<{ atxtiny_hal::timer::C3 }>,
            atxtiny_hal::timer::Ch<{ atxtiny_hal::timer::C4 }>,
        ),
        (
            TcaPinset<
                'static,
                TCA0,
                atxtiny_hal::gpio::Pin<
                    atxtiny_hal::gpio::Porta,
                    atxtiny_hal::gpio::U<1>,
                    atxtiny_hal::gpio::Output<atxtiny_hal::gpio::Stateless>,
                >,
                TCA0PortA,
                { atxtiny_hal::timer::C2 },
            >,
            TcaPinset<
                'static,
                TCA0,
                atxtiny_hal::gpio::Pin<
                    atxtiny_hal::gpio::Porta,
                    atxtiny_hal::gpio::U<2>,
                    atxtiny_hal::gpio::Output<atxtiny_hal::gpio::Stateless>,
                >,
                TCA0PortA,
                { atxtiny_hal::timer::C3 },
            >,
            TcaPinset<
                'static,
                TCA0,
                atxtiny_hal::gpio::Pin<
                    atxtiny_hal::gpio::Porta,
                    atxtiny_hal::gpio::U<3>,
                    atxtiny_hal::gpio::Output<atxtiny_hal::gpio::Stateless>,
                >,
                TCA0PortA,
                { atxtiny_hal::timer::C4 },
            >,
        ),
        31250,
    >,
    mode: AuxLedsMode,
    wakelock: Mug,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AuxLedsMode {
    Low,
    Pwm,
}

impl AuxLeds {
    fn toggle_pwm(&mut self, on: bool) {
        if on {
            self.pwm1.enable(atxtiny_hal::timer::Channel::C2);
            self.pwm1.enable(atxtiny_hal::timer::Channel::C3);
            self.pwm1.enable(atxtiny_hal::timer::Channel::C4);
        } else {
            self.pwm1.disable(atxtiny_hal::timer::Channel::C2);
            self.pwm1.disable(atxtiny_hal::timer::Channel::C3);
            self.pwm1.disable(atxtiny_hal::timer::Channel::C4);
        }
    }

    fn red_pwm(&mut self, value: u8) {
        self.pwm1
            .set_duty_time(
                atxtiny_hal::timer::Channel::C2,
                fugit::Duration::<u32, _, _>::from_ticks(value as u32),
            )
            .unwrap();
    }

    fn green_pwm(&mut self, value: u8) {
        self.pwm1
            .set_duty_time(
                atxtiny_hal::timer::Channel::C3,
                fugit::Duration::<u32, _, _>::from_ticks(value as u32),
            )
            .unwrap();
    }

    fn blue_pwm(&mut self, value: u8) {
        self.pwm1
            .set_duty_time(
                atxtiny_hal::timer::Channel::C4,
                fugit::Duration::<u32, _, _>::from_ticks(value as u32),
            )
            .unwrap();
    }

    // We use the pull up input mode to make the LEDs turn on at a low
    // brightness while the UC can sleep. I don't want to write a dance of
    // passing around ownership of the pins because we can get away with just
    // toggling the mode

    fn red_low(&mut self, on: bool) {
        if on {
            unsafe {
                let mut p = atxtiny_hal::avr_device::avr32dd20::PORTA::steal()
                    .split()
                    .pa1
                    .into_push_pull_output();
                p.set_high().unwrap_infallible();
            };
        } else {
            unsafe {
                let mut p = atxtiny_hal::avr_device::avr32dd20::PORTA::steal()
                    .split()
                    .pa1
                    .into_push_pull_output();
                p.set_low().unwrap_infallible();
            };
        }
    }

    fn green_low(&mut self, on: bool) {
        if on {
            unsafe {
                atxtiny_hal::avr_device::avr32dd20::PORTA::steal()
                    .split()
                    .pa2
                    .into_pull_up_input()
            };
        } else {
            unsafe {
                let mut r = atxtiny_hal::avr_device::avr32dd20::PORTA::steal()
                    .split()
                    .pa2
                    .into_push_pull_output();
                r.set_low().unwrap_infallible();
            };
        }
    }

    fn blue_low(&mut self, on: bool) {
        if on {
            unsafe {
                atxtiny_hal::avr_device::avr32dd20::PORTA::steal()
                    .split()
                    .pa3
                    .into_pull_up_input()
            };
        } else {
            unsafe {
                let mut r = atxtiny_hal::avr_device::avr32dd20::PORTA::steal()
                    .split()
                    .pa3
                    .into_push_pull_output();
                r.set_low().unwrap_infallible();
            };
        }
    }

    fn set_rgb_low(&mut self, rgb: Rgb1Bit) {
        self.red_low(rgb.r);
        self.green_low(rgb.g);
        self.blue_low(rgb.b);
    }

    fn set_mode(&mut self, mode: AuxLedsMode) {
        if self.mode != mode {
            match mode {
                AuxLedsMode::Pwm => {
                    self.set_rgb_low(Rgb1Bit {
                        r: false,
                        g: false,
                        b: false,
                    });
                    self.toggle_pwm(true);
                }
                AuxLedsMode::Low => {
                    self.toggle_pwm(false);
                    self.set_rgb_low(Rgb1Bit {
                        r: false,
                        g: false,
                        b: false,
                    });
                }
            }
            self.mode = mode;
        }
    }

    fn set_rgb_pwm(&mut self, rgb: ColorRGB) {
        self.red_pwm(rgb.r.min(254));
        self.green_pwm(rgb.g.min(254));
        self.blue_pwm(rgb.b.min(254));
    }
}

async fn transition_to_pwm(leds: &mut AuxLeds, prior: ColorRGB, target: ColorRGB) {
    leds.set_mode(AuxLedsMode::Pwm);
    leds.set_rgb_pwm(prior);

    for i in (0..255u8).step_by(20) {
        let mut c = prior;
        c.blend(target, i);
        leds.set_rgb_pwm(c);

        embassy_time::Timer::after_millis(16).await;
    }
}

fn hue_to_rgb(hue: u8) -> ColorRGB {
    let hue = cichlid::math::scale_u8(hue, 191);

    const HSV_SECTION_4: u8 = 0x40;

    let section = hue / HSV_SECTION_4;
    let offset = hue % HSV_SECTION_4;

    let brightness_floor = 0;

    let ramp_up = offset;
    let ramp_down = HSV_SECTION_4 - offset;

    let amplitude = 191_u16;

    let ramp_up_amp_adj = ((ramp_up as u16 * amplitude) / (256 / 4)) as u8;
    let ramp_down_amp_adj = ((ramp_down as u16 * amplitude) / (256 / 4)) as u8;

    let ramp_up = ramp_up.saturating_add(ramp_up_amp_adj);
    let ramp_down = ramp_down.saturating_add(ramp_down_amp_adj);

    match section {
        0 => ColorRGB::new(ramp_down, ramp_up, brightness_floor),
        1 => ColorRGB::new(brightness_floor, ramp_down, ramp_up),
        _ => ColorRGB::new(ramp_up, brightness_floor, ramp_down),
    }
}

async fn rainbow_aux(leds: &mut AuxLeds, prior: ColorRGB) -> ColorRGB {
    leds.wakelock.caffeinate();

    let mut h = 0u8;
    let target_startup_colour = hue_to_rgb(h);

    transition_to_pwm(leds, prior, target_startup_colour).await;

    loop {
        let rgb = hue_to_rgb(h);

        if !crate::states::is_torch_on() {
            return rgb;
        }

        leds.set_rgb_pwm(rgb);

        h = h.wrapping_add(1);

        embassy_time::Timer::after_millis(16).await;
    }
}

fn volts_to_rgb() -> ColorRGB {
    let volts = crate::sensing::VOLTAGE.get().volts_times_50();

    const MAX: u8 = Voltage::volts_to_adc_output(4.2).volts_times_50();
    const MIN: u8 = Voltage::volts_to_adc_output(3.4).volts_times_50();

    // red
    let min_hue = 0u8;
    // magenta
    let max_hue = 212u8;

    let hue = if volts < MIN {
        min_hue
    } else if volts > MAX {
        max_hue
    } else {
        fixed::types::U8F0::inv_lerp::<fixed::types::extra::U8>(
            volts.into(),
            MIN.into(),
            MAX.into(),
        )
        .lerp(min_hue.into(), max_hue.into())
        .to_num()
    };

    hue_to_rgb(hue)
}

fn volts_to_1bit_rgb() -> Rgb1Bit {
    let volts = crate::sensing::VOLTAGE.get();

    const fn v(x: f32) -> Voltage<u16> {
        Voltage::volts_to_adc_output(x)
    }

    if volts > const { v(4.1) } {
        Rgb1Bit::new(true, false, true)
    } else if volts > const { v(3.9) } {
        Rgb1Bit::new(false, false, true)
    } else if volts > const { v(3.7) } {
        Rgb1Bit::new(false, true, true)
    } else if volts > const { v(3.5) } {
        Rgb1Bit::new(false, true, false)
    } else if volts > const { v(3.3) } {
        Rgb1Bit::new(true, true, false)
    } else {
        Rgb1Bit::new(true, false, false)
    }
}

async fn voltage_high_aux(leds: &mut AuxLeds, prior: ColorRGB) -> ColorRGB {
    leds.wakelock.caffeinate();

    let target_startup_colour = volts_to_rgb();

    transition_to_pwm(leds, prior, target_startup_colour).await;

    loop {
        let rgb = volts_to_rgb();

        if crate::states::is_torch_on() || !crate::states::is_torch_unlocked() {
            return rgb;
        }

        leds.set_rgb_pwm(rgb);

        embassy_time::Timer::after_millis(64).await;
    }
}

async fn voltage_low_aux(leds: &mut AuxLeds, prior: ColorRGB) -> ColorRGB {
    let target_startup_colour = volts_to_1bit_rgb().to_colorrgb();

    transition_to_pwm(leds, prior, target_startup_colour).await;

    leds.wakelock.decaffeinate();

    leds.set_mode(AuxLedsMode::Low);

    loop {
        let rgb = volts_to_1bit_rgb();
        leds.set_rgb_low(rgb);

        if crate::states::is_torch_unlocked() {
            return rgb.to_colorrgb();
        }

        embassy_time::Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn aux_control(mut leds: AuxLeds) {
    let mut prior_colour = ColorRGB::Black;

    loop {
        if crate::states::is_torch_on() {
            prior_colour = rainbow_aux(&mut leds, prior_colour).await;
        } else if crate::states::is_torch_unlocked() {
            prior_colour = voltage_high_aux(&mut leds, prior_colour).await;
        } else {
            prior_colour = voltage_low_aux(&mut leds, prior_colour).await;
        }
    }
}

pub fn setup(
    spawner: embassy_executor::Spawner,
    tca0: TCA0,
    clocks: Clocks,
    portmux: PortmuxTCARoutes,
    pa1: gpio::PA1<Input>,
    pa2: gpio::PA2<Input>,
    pa3: gpio::PA3<Input>,
) {
    let t = atxtiny_hal::timer::FTimer::<_, 31250>::new(tca0, clocks).unwrap();

    let portmux_proof = portmux.port_a_forever();

    let pwm_pins = (
        TcaPinset::prove(pa1.into_stateless_push_pull_output(), &portmux_proof),
        TcaPinset::prove(pa2.into_stateless_push_pull_output(), &portmux_proof),
        TcaPinset::prove(pa3.into_stateless_push_pull_output(), &portmux_proof),
    );

    let mut pwm1 = t
        .pwm(pwm_pins, fugit::Duration::<u32, _, _>::from_ticks(255))
        .unwrap();

    pwm1.set_duty_time(
        atxtiny_hal::timer::Channel::C2,
        fugit::Duration::<u32, _, _>::from_ticks(150),
    )
    .unwrap();
    pwm1.set_duty_time(
        atxtiny_hal::timer::Channel::C3,
        fugit::Duration::<u32, _, _>::from_ticks(100),
    )
    .unwrap();
    pwm1.set_duty_time(
        atxtiny_hal::timer::Channel::C4,
        fugit::Duration::<u32, _, _>::from_ticks(200),
    )
    .unwrap();
    pwm1.enable(atxtiny_hal::timer::Channel::C2);
    pwm1.enable(atxtiny_hal::timer::Channel::C3);
    pwm1.enable(atxtiny_hal::timer::Channel::C4);

    spawner.must_spawn(aux_control(AuxLeds {
        pwm1,
        mode: AuxLedsMode::Pwm,
        wakelock: Mug::new(),
    }))
}
