//! # System configuration

use crate::pac::SYSCFG;

/// Extension trait for the [`SYSCFG`] peripheral
pub trait SyscfgExt {
    /// Constrains the [`SYSCFG`] peripheral.
    ///
    /// Offers a convenience method on the [`pac::SYSCFG`] peripheral to retrieve
    /// the device's revision ID.
    ///
    /// [`pac::SYSCFG`]: `crate::pac::SYSCFG`
    fn get_revision_id(&self) -> u8;
}

impl SyscfgExt for SYSCFG {
    fn get_revision_id(&self) -> u8 {
        self.revid().read().bits()
    }
}
