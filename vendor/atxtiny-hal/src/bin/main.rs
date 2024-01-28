// NOTE (Andy): This is a test binary that I use to quickly try out things
//              I'll keep this for now, I might throw this away later
//              This might also serve you as a template to get your own
//              project started

#![no_std]
#![no_main]
#![feature(result_option_inspect)]
#![feature(abi_avr_interrupt)]

use atxtiny_hal::serial::UartPinset;
use atxtiny_hal::timer::Channel;
//use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;

use atxtiny_hal::serial::Serial;
use atxtiny_hal::watchdog::{WdtExt, WatchdogTimeout};
use atxtiny_hal::gpio::{Input, Output, Stateless};

mod misc;

use core::fmt::Write;
//use core::cell::RefCell;
//use avr_device::interrupt::{self, Mutex};
use atxtiny_hal::pac::USART0;

atxtiny_hal::impl_panic_handler!(Serial<USART0, UartPinset<USART0, atxtiny_hal::gpio::porta::PA2<Input>, atxtiny_hal::gpio::porta::PA1<Output<Stateless>>>>);

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let clkctrl = dp.CLKCTRL.constrain();
    let portmux = dp.PORTMUX.constrain();

    let clocks = clkctrl.freeze();

    let a = dp.PORTA.split();
    let b = dp.PORTB.split();
    let _c = dp.PORTC.split();
    let btn = b.pb7.into_pull_up_input();
    let mut led = b.pb6.into_push_pull_output();
    let mut led2 = b.pb5.into_push_pull_output();

    // Serial port
    // We need to annotate the pins with the peripheral here because PA1/2 can
    // also be used as TWI pins and we need to tell the MUX what bit to flip
    let rxpin = a.pa2.into_peripheral::<USART0>();
    let txpin = a.pa1.into_peripheral::<USART0>();
    // let rxpin = b.pb3.into_peripheral();
    // let txpin = b.pb2.into_peripheral();

    let usart_pair = (rxpin, txpin);
    let usart_pair = usart_pair.mux(&portmux);

    let s = Serial::new(dp.USART0, usart_pair, 115200u32.bps(), clocks);
    let s = share_serial_port_with_panic(s);
    s.write_str("Hello\r\n".into()).unwrap();

    // // Initialize global USART variable
    // interrupt::free(|cs| {
    //     let mut u = USART.borrow(cs).borrow_mut();
    //     *u = Some(MySerial(unsafe { crate::pac::Peripherals::steal().USART0 }));
    // });

    // Watchdog
    let mut wd = dp.WDT.constrain();
    wd.start(WatchdogTimeout::S8);


    use atxtiny_hal::timer::{FTimer, Timer};
    use atxtiny_hal::timer::tca::WaveformGenerationMode;
    // Create a new timer with a fixed frequency using TCA0
    //let tca = FTimer::<_, 312500>::new(dp.TCA0, clocks).unwrap();
    //let tca0_clk = tca.use_as_clock_source();
    let t = Timer::new(dp.TCA0, clocks);

    let pwm_pins = (
        b.pb0.into_stateless_push_pull_output().mux(&portmux),
        b.pb1.into_stateless_push_pull_output().mux(&portmux),
        b.pb2.into_stateless_push_pull_output().mux(&portmux),
    );

    //let mut pwm = tca.pwm(pwm_pins, 10.millis(), WaveformGenerationMode::SingleSlope).unwrap();
    let mut pwm = t.pwm_hz(pwm_pins, 1.kHz(), WaveformGenerationMode::SingleSlope).unwrap();

    let max_duty = pwm.get_max_duty() as u16;
    ufmt::uwriteln!(s, "Max duty: {}", max_duty).unwrap();

    pwm.set_duty(Channel::C1, 0);
    pwm.enable(Channel::C1);

    pwm.set_duty(Channel::C2, 5000);
    pwm.enable(Channel::C2);

    pwm.set_duty(Channel::C3, 1000);
    pwm.enable(Channel::C3);

    //let (pin1, pin2, pin3) = pwm.split();


    // Timer for delay
    let tcb = FTimer::<_, 10000000>::new(dp.TCB0, clocks.into()).unwrap();
    let mut d = tcb.delay();
    // hz2.start(100.millis()).unwrap();


    //unsafe { avr_device::interrupt::enable() };

    let mut i = 0;

    loop {
        wd.feed();

        if btn.is_low().unwrap() {
            led2.set_high().unwrap();
        } else {
            led2.set_low().unwrap();
        }


        pwm.set_duty(Channel::C1, i);
        i += 10;

        if i > max_duty {
            ufmt::uwrite!(s, ".");
            i = 0;
        }
        d.delay(1.millis());

        // if hz2.wait().is_ok() {
        //     led.toggle().unwrap();

        //     //pwm.set_duty_time(Channel::C1, i.millis()).unwrap();

        //     //s.write_str("Hallo\r\n".into()).unwrap();
        //     //serial_println!("Hallo {}\r", i);
        //     i += 1;

        //     if i > 10 {
        //         i = 0;
        //     }
        // }
    }
}
