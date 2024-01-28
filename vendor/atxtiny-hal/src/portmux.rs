//! # Port Multiplexer

// FIXME: Do we really need a constrained peripheral here? We could just get the
//        pointer to the PORTMUX in every `mux` implementation and work with that.
//        This also alleviates the need to pass a reference to it around.

use embedded_hal::digital::OutputPin;

use crate::pac::PORTMUX;

/// Extension trait that constrains the [`PORTMUX`] peripheral
pub trait PortmuxExt {
    /// Constrains the [`PORTMUX`] peripheral.
    ///
    /// Consumes the [`pac::PORTMUX`] peripheral and converts it to a [`HAL`] internal type
    /// constraining it's public access surface to fit the design of the `HAL`.
    ///
    /// [`pac::PORTMUX`]: `crate::pac::PORTMUX`
    /// [`HAL`]: `crate`
    fn constrain(self) -> Portmux;
}

/// Constrained Portmux peripheral
///
/// An instance of this struct is acquired by calling the [`constrain`](PortmuxExt::constrain) function
/// on the [`PORTMUX`] struct.
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let portmux = dp.PORTMUX.constrain();
/// ```
pub struct Portmux {
    mux: PORTMUX
}

impl PortmuxExt for PORTMUX {
    fn constrain(self) -> Portmux {
        Portmux { mux: self }
    }
}

/// Trait implemented by pinsets that can be muxed onto physical pins.
/// 
/// The actual muxing happens when calling the [`IntoMuxedPinset::mux`] method
/// on a defined pinset
/// 
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let portmux = dp.PORTMUX.constrain();
/// let porta = dp.PORTA.split();
/// 
/// let rxpin = porta.pa2.into_peripheral::<pac::USART0>();
/// let txpin = porta.pa1.into_peripheral::<pac::USART0>();
/// 
/// let usart_pair = (rxpin, txpin);
/// let usart_pair = usart_pair.mux(&portmux);
/// ```
pub trait IntoMuxedPinset<Peripheral> {
    /// The resulting pinset that is returned when the mux is configure to
    /// enable it.
    type Pinset;

    /// Setup the hardware to enable the multiplexing of this pinset.
    /// 
    /// Calling this function may also reconfigure GPIO input or output modes
    /// and set pin levels if needed.
    fn mux(self, portmux: &Portmux) -> Self::Pinset;
}



use crate::gpio::{Peripheral, Input, Output, Stateless};

// Serial
use crate::serial::UartPinset;
use crate::pac::USART0;

impl IntoMuxedPinset<USART0> for (crate::gpio::portb::PB3<Peripheral<USART0>>, crate::gpio::portb::PB2<Peripheral<USART0>>) {
    type Pinset = UartPinset<USART0, crate::gpio::portb::PB3<Input>, crate::gpio::portb::PB2<Output<Stateless>>>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrlb().modify(|_r, w| w.usart0().clear_bit());
        let mut tx = self.1.into_stateless_push_pull_output();

        // Set the TX pin high to turn switch it to idle level
        // Otherwise receivers might mistake the low level as a start bit and if
        // not enough time passes between init and the first data to be sent, the
        // receiver becomes confused because it's not in sync with the transmitter
        // anymore
        tx.set_high().unwrap();

        UartPinset::new(self.0.into_floating_input(), tx)
    }
}

impl IntoMuxedPinset<USART0> for (crate::gpio::porta::PA2<Peripheral<USART0>>, crate::gpio::porta::PA1<Peripheral<USART0>>) {
    type Pinset = UartPinset<USART0, crate::gpio::porta::PA2<Input>, crate::gpio::porta::PA1<Output<Stateless>>>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrlb().modify(|_r, w| w.usart0().set_bit());
        let mut tx = self.1.into_stateless_push_pull_output();

        // Set the TX pin high to turn switch it to idle level
        // Otherwise receivers might mistake the low level as a start bit and if
        // not enough time passes between init and the first data to be sent, the
        // receiver becomes confused because it's not in sync with the transmitter
        // anymore
        tx.set_high().unwrap();

        UartPinset::new(self.0.into_floating_input(), tx)
    }
}

// TWI
use crate::twi::TwiPinset;
use crate::pac::TWI0;

impl IntoMuxedPinset<TWI0> for (crate::gpio::portb::PB0<Peripheral<TWI0>>, crate::gpio::portb::PB1<Peripheral<TWI0>>) {
    type Pinset = TwiPinset<TWI0, crate::gpio::portb::PB0<Peripheral<TWI0>>, crate::gpio::portb::PB1<Peripheral<TWI0>>>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrlb().modify(|_r, w| w.twi0().clear_bit());
        TwiPinset::new(self.0, self.1)
    }
}

impl IntoMuxedPinset<TWI0> for (crate::gpio::porta::PA2<Peripheral<TWI0>>, crate::gpio::porta::PA1<Peripheral<TWI0>>) {
    type Pinset = TwiPinset<TWI0, crate::gpio::porta::PA2<Peripheral<TWI0>>, crate::gpio::porta::PA1<Peripheral<TWI0>>>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrlb().modify(|_r, w| w.twi0().set_bit());
        TwiPinset::new(self.0, self.1)
    }
}

// SPI
use crate::spi::SpiPinset;
use crate::pac::SPI0;

impl IntoMuxedPinset<SPI0> for (crate::gpio::porta::PA3<Peripheral<SPI0>>, crate::gpio::porta::PA2<Peripheral<SPI0>>, crate::gpio::porta::PA1<Peripheral<SPI0>>) {
    type Pinset = SpiPinset<SPI0, crate::gpio::porta::PA3<Output<Stateless>>, crate::gpio::porta::PA2<Input>, crate::gpio::porta::PA1<Output<Stateless>>>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrlb().modify(|_r, w| w.spi0().clear_bit());
        // Turn the pins into stateless outputs
        // In SPI host mode, this hands over the pin to the SPI peripheral
        SpiPinset::new(
            self.0.into_stateless_push_pull_output(),
            self.1.into_floating_input(),
            self.2.into_stateless_push_pull_output(),
        )
    }
}

impl IntoMuxedPinset<SPI0> for (crate::gpio::portc::PC0<Peripheral<SPI0>>, crate::gpio::portc::PC1<Peripheral<SPI0>>, crate::gpio::portc::PC2<Peripheral<SPI0>>) {
    type Pinset = SpiPinset<SPI0, crate::gpio::portc::PC0<Output<Stateless>>, crate::gpio::portc::PC1<Input>, crate::gpio::portc::PC2<Output<Stateless>>>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrlb().modify(|_r, w| w.spi0().set_bit());
        // Turn the pins into stateless outputs
        // In SPI host mode, this hands over the pin to the SPI peripheral
        SpiPinset::new(
            self.0.into_stateless_push_pull_output(),
            self.1.into_floating_input(),
            self.2.into_stateless_push_pull_output(),
        )
    }
}

// CCL
use crate::ccl::{LUT0, LUT1, CclLutOutputPinset};

impl IntoMuxedPinset<LUT0> for crate::gpio::porta::PA4<Output<Stateless>> {
    type Pinset = CclLutOutputPinset<LUT0, crate::gpio::porta::PA4<Output<Stateless>>>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrla().modify(|_r, w| w.lut0().clear_bit());
        CclLutOutputPinset::new(self)
    }
}

impl IntoMuxedPinset<LUT0> for crate::gpio::portb::PB4<Output<Stateless>> {
    type Pinset = CclLutOutputPinset<LUT0, crate::gpio::portb::PB4<Output<Stateless>>>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrla().modify(|_r, w| w.lut0().set_bit());
        CclLutOutputPinset::new(self)
    }
}

impl IntoMuxedPinset<LUT1> for crate::gpio::porta::PA7<Output<Stateless>> {
    type Pinset = CclLutOutputPinset<LUT1, crate::gpio::porta::PA7<Output<Stateless>>>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrla().modify(|_r, w| w.lut1().clear_bit());
        CclLutOutputPinset::new(self)
    }
}

impl IntoMuxedPinset<LUT1> for crate::gpio::portc::PC1<Output<Stateless>> {
    type Pinset = CclLutOutputPinset<LUT1, crate::gpio::portc::PC1<Output<Stateless>>>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrla().modify(|_r, w| w.lut1().set_bit());
        CclLutOutputPinset::new(self)
    }
}

// TCA
use crate::timer::tca::TcaPinset;
use crate::timer::{C1, C2, C3};
use crate::pac::TCA0;

impl IntoMuxedPinset<TCA0> for crate::gpio::portb::PB0<Output<Stateless>> {
    type Pinset = TcaPinset<TCA0, crate::gpio::portb::PB0<Output<Stateless>>, C1>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrlc().modify(|_r, w| w.tca00().clear_bit());
        TcaPinset::new(self)
    }
}

impl IntoMuxedPinset<TCA0> for crate::gpio::portb::PB1<Output<Stateless>> {
    type Pinset = TcaPinset<TCA0, crate::gpio::portb::PB1<Output<Stateless>>, C2>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrlc().modify(|_r, w| w.tca01().clear_bit());
        TcaPinset::new(self)
    }
}

impl IntoMuxedPinset<TCA0> for crate::gpio::portb::PB2<Output<Stateless>> {
    type Pinset = TcaPinset<TCA0, crate::gpio::portb::PB2<Output<Stateless>>, C3>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrlc().modify(|_r, w| w.tca02().clear_bit());
        TcaPinset::new(self)
    }
}

impl IntoMuxedPinset<TCA0> for crate::gpio::portb::PB3<Output<Stateless>> {
    type Pinset = TcaPinset<TCA0, crate::gpio::portb::PB3<Output<Stateless>>, C1>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrlc().modify(|_r, w| w.tca00().set_bit());
        TcaPinset::new(self)
    }
}

impl IntoMuxedPinset<TCA0> for crate::gpio::portb::PB4<Output<Stateless>> {
    type Pinset = TcaPinset<TCA0, crate::gpio::portb::PB4<Output<Stateless>>, C2>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrlc().modify(|_r, w| w.tca01().set_bit());
        TcaPinset::new(self)
    }
}

impl IntoMuxedPinset<TCA0> for crate::gpio::portb::PB5<Output<Stateless>> {
    type Pinset = TcaPinset<TCA0, crate::gpio::portb::PB5<Output<Stateless>>, C3>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrlc().modify(|_r, w| w.tca02().set_bit());
        TcaPinset::new(self)
    }
}

// EVOUT
use crate::evout::EventOutputPinset;
use crate::evout::{EVOUT0, EVOUT1, EVOUT2};
use crate::pac::EVSYS;

impl IntoMuxedPinset<EVSYS> for crate::gpio::porta::PA2<Peripheral<EVSYS>> {
    type Pinset = EventOutputPinset<EVSYS, crate::gpio::porta::PA2<Peripheral<EVSYS>>, EVOUT0>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrla().modify(|_r, w| w.evout0().set_bit());
        EventOutputPinset::new(self)
    }
}

impl IntoMuxedPinset<EVSYS> for crate::gpio::portb::PB2<Peripheral<EVSYS>> {
    type Pinset = EventOutputPinset<EVSYS, crate::gpio::portb::PB2<Peripheral<EVSYS>>, EVOUT1>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrla().modify(|_r, w| w.evout1().set_bit());
        EventOutputPinset::new(self)
    }
}

impl IntoMuxedPinset<EVSYS> for crate::gpio::portc::PC2<Peripheral<EVSYS>> {
    type Pinset = EventOutputPinset<EVSYS, crate::gpio::portc::PC2<Peripheral<EVSYS>>, EVOUT2>;

    fn mux(self, portmux: &Portmux) -> Self::Pinset {
        portmux.mux.ctrla().modify(|_r, w| w.evout2().set_bit());
        EventOutputPinset::new(self)
    }
}
