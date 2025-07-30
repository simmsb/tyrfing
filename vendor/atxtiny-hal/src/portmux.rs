//! # Port Multiplexer

// FIXME: Do we really need a constrained peripheral here? We could just get the
//        pointer to the PORTMUX in every `mux` implementation and work with that.
//        This also alleviates the need to pass a reference to it around.

use core::marker::PhantomData;

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
    mux: PORTMUX,
}

impl PortmuxExt for PORTMUX {
    fn constrain(self) -> Portmux {
        Portmux { mux: self }
    }
}

// A proof that the portmux is in a certain mode
pub struct PortmuxProof<'a, PARENT, MODE> {
    _phantom: PhantomData<(PARENT, &'a mut MODE)>,
}

pub struct PortmuxParts {
    pub tca_routes: PortmuxTCARoutes,
}

impl Portmux {
    pub fn split(self) -> PortmuxParts {
        PortmuxParts {
            tca_routes: PortmuxTCARoutes { mux: self.mux },
        }
    }
}

pub struct PortmuxTCARoutes {
    mux: PORTMUX,
}

pub enum TCA0PortA {}
pub enum TCA0PortC {}
pub enum TCA0PortD {}

impl PortmuxTCARoutes {
    pub fn port_a(&mut self) -> PortmuxProof<'_, Self, TCA0PortA> {
        self.mux.tcaroutea().write(|w| w.tca0().porta());

        PortmuxProof {
            _phantom: PhantomData,
        }
    }

    pub fn port_c(&mut self) -> PortmuxProof<'_, Self, TCA0PortC> {
        self.mux.tcaroutea().write(|w| w.tca0().portc());

        PortmuxProof {
            _phantom: PhantomData,
        }
    }

    pub fn port_d(&mut self) -> PortmuxProof<'_, Self, TCA0PortD> {
        self.mux.tcaroutea().write(|w| w.tca0().portd());

        PortmuxProof {
            _phantom: PhantomData,
        }
    }

    pub fn port_a_forever(self) -> PortmuxProof<'static, Self, TCA0PortA> {
        self.mux.tcaroutea().write(|w| w.tca0().porta());

        PortmuxProof {
            _phantom: PhantomData,
        }
    }

    pub fn port_c_forever(self) -> PortmuxProof<'static, Self, TCA0PortC> {
        self.mux.tcaroutea().write(|w| w.tca0().portc());

        PortmuxProof {
            _phantom: PhantomData,
        }
    }

    pub fn port_d_forever(self) -> PortmuxProof<'static, Self, TCA0PortD> {
        self.mux.tcaroutea().write(|w| w.tca0().portd());

        PortmuxProof {
            _phantom: PhantomData,
        }
    }
}
