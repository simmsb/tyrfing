atxtiny-hal
===========

Hardware abstraction layer for the tinyAVR 0-, 1- and 2-series microcontrollers.

Possibly applicable to other modern AVRs based on the xmega peripherals as well as long as somebody does the work of implementing macros for peripheral instantiation and actual definitions of the peripherals of the different controllers.

I built this by myself for private projects by looking at similar HAL crates for the STM32 series or the avr-hal from rahix.

## Supported controllers

* atttiny817

## Supported peripherals:

* CLKCTRL - Clock controller
* NVMCTRL - Nonvolatile Memory Controller
* SLPCTRL - Sleep controller
* RSTCTRL - Reset controller
* BOD - Brownout detector
* VREF - Voltage reference
* AC - Analog comparator
* DAC - Digital to Analog converter
* GPIO - General Purpose I/O
* PORTMUX - Port Multiplexer
* WDT - Watchdog Timer
* USART - Universal Synchronous and Asynchronous Receiver and Transmitter
    * only in asynchronous serial mode
* SPI - Serial Peripheral Interface
    * only unbuffered host mode
* TWI - Two-Wire Interface
    * only master mode
* CCL
* TCA
* TCB
* CPUINT
* RTC

## Missing peripheral support:

* TCD
    * including event generators
* ADC
    * including event generators
* PIT
    * including event generators
* CRCSCAN
* (PTC) - very proprietary and undocumented

## General TODOs:

* Implement SPI and UART `embedded_hal_nb` traits

* FIXME: When releasing a pin from the portmux, we never mux back to GPIO. We should do that when releasing all the pinsets

* FIXME: move all the pin modes in portmux from Output<Sateless> to Peripheral<PERI> to avoid type conflicts? Already done for TWI and EVOUT

* fugit should support 16 bit time types on AVR, 32 bits are wasteful as most timers only have 16 bits resolution anyway
    * is this even feasible?

* CLKCTRL
    * allow to configure 32khz clock either from internal source, external crystal or external clock
    * pass this clock to RTC to make sure if external 32KHz clock is used that it is actually configured and works

* EVSYS
    * More event generators and users for a few peripherals
    * Software strobes
    * Use the type-system the declare generator and user constants somehow
    * Allow multiple users per channel - see FIXME in example when assigning users
    * It's really only applicable to the 1-series AVRs, 2-series is very different

* RTC
    * Event system
    * External 32KHz clock support
    * Compare channel support (see general timer TODO)

* Generally introduce a lot of macros to instantiate peripherals in different configs for different controllers
    * Right now we only support the Attiny817

* Custom startup code
    * Right now we depend on avr-libc for crt0 stuff, linking and interrupt vectors
    * Ideally we want our own code that does this just like for the ARM with the cortex-m crate for example

## TODO CCL:

* The API is suboptimal
* Passing input pins into it is meh
* The timers solve this problem with tuples and macros, maybe do the same on CCL?

## TODO Timers:

* General
    * RUNSTDBY support?
    * Allow for dummy pins to be able to use compare channels with EVSYS later
    * split PWM trait in PWM and compare channel traits as we have timers that have compare channels, but not assigned PWM outputs like the RTC

* TCA
    * Split mode

* TCB
    * oneshot mode
    * delay in oneshot mode
    * PWM in 8 bit mode
    * input capture mode
    * timeout check mode

* TCD
    * completely different beast, currently no support at all


# Setup

## Patched dependency crates

* atdf2svd
  * Needed to handle register map modes - [patch has been merged upstream](https://github.com/Rahix/atdf2svd/pull/48)

* svd2rust
  * Support for CCP protected registers - [patch submitted](https://github.com/rust-embedded/svd2rust/pull/784)

* avr-device
  * attiny817 support - [patch submitted](https://github.com/Rahix/avr-device/pull/144)

## Installing stuff

1. You need a patched version of avr-libc which contains support for the newer ATtiny chips. On Arch Linux there is an [AUR package](https://aur.archlinux.org/packages/avr-libc-avrxmega3-svn) available

2. Install the rest of the AVR toolchain from your distro like binutils

3. Prepare all the rust crates and build everything:
```
git clone https://github.com/Rahix/atdf2svd.git
cd atdf2svd
cargo install --path .
cd ..

git clone -b avr_ccp https://github.com/G33KatWork/svd2rust.git
cd svd2rust
cargo install --path .
cd ..

git clone -b attiny817 https://github.com/G33KatWork/avr-device.git
cd avr-device
make -j16
cd ..

git clone https://github.com/G33KatWork/atxtiny-hal.git
cd atxtiny-hal
cargo build --release --examples
```

Right now all of this is a bit cumbersome to use. Should become easier once patches we depend on are merged upstream into `svd2rust` and `avr-device`.

## Programming things using pymcuprog

```
# Installing pymcuprog
python -m venv env
source env/bin/activate
pip install pymcuprog

# Building and flashing stuff
cargo build --release --examples && avr-objcopy -O ihex target/avr-attiny817/release/examples/serial.elf /tmp/avr.hex && pymcuprog -d attiny817 -t uart -u /dev/ttyUSB0 erase && pymcuprog -d attiny817 -t uart -u /dev/ttyUSB0 write -f /tmp/avr.hex
```

## Other possible programmers

* https://daumemo.com/diy-updi-usb-programmer-which-can-be-made-with-cheap-hardware/
