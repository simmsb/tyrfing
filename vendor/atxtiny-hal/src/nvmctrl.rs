//! # Non-Volatile Memory Controller

use cfg_if::cfg_if;

use core::ptr;
use crate::pac::NVMCTRL;

#[cfg(feature = "attiny817")]
use avr_device::{ccp::ProtectedWritable, attiny817::nvmctrl::ctrla::CMD_A};

#[cfg(feature = "attiny1616")]
use avr_device::{ccp::ProtectedWritable, attiny1616::nvmctrl::ctrla::CMD_A};

#[cfg(feature = "avr32dd20")]
use avr_device::{ccp::ProtectedWritable, avr32dd20::nvmctrl::ctrla::CMD_A};

// TODO: SIGROW  = 0x1100
//       FUSES   = 0x1280
//       USERROW = 0x1300
// TODO: Parse BOOTEND and APPEND fuses and offer some API?

cfg_if! {
    if #[cfg(any(
        feature = "attiny414",
    ))] {
        /// Start address of the flash in data space
        pub const FLASH_START:      usize = 0x8000;
        
        /// End address of the flash in data space
        pub const FLASH_END:        usize = 0x8FFF;

        /// Page size of the flash in data space
        pub const FLASH_PAGE_SIZE:  usize = 64;


        /// Start address of the EEPROM in data space
        pub const EEPROM_START:     usize = 0x1400;

        /// End address of the EEPROM in data space
        pub const EEPROM_END:       usize = 0x147F;

        /// Page size of the EEPROM in data space
        pub const EEPROM_PAGE_SIZE: usize = 32;

    } else if #[cfg(any(
        feature = "attiny814",
        feature = "attiny816",
        feature = "attiny817",
    ))] {
        /// Start address of the flash in data space
        pub const FLASH_START:      usize = 0x8000;
        
        /// End address of the flash in data space
        pub const FLASH_END:        usize = 0x9FFF;

        /// Page size of the flash in data space
        pub const FLASH_PAGE_SIZE:  usize = 64;


        /// Start address of the EEPROM in data space
        pub const EEPROM_START:     usize = 0x1400;

        /// End address of the EEPROM in data space
        pub const EEPROM_END:       usize = 0x147F;

        /// Page size of the EEPROM in data space
        pub const EEPROM_PAGE_SIZE: usize = 32;

    } else if #[cfg(any(
        feature = "attiny1614",
        feature = "attiny1616",
        feature = "attiny1617",
    ))] {
        /// Start address of the flash in data space
        pub const FLASH_START:      usize = 0x8000;
        
        /// End address of the flash in data space
        pub const FLASH_END:        usize = 0xBFFF;

        /// Page size of the flash in data space
        pub const FLASH_PAGE_SIZE:  usize = 64;


        /// Start address of the EEPROM in data space
        pub const EEPROM_START:     usize = 0x1400;

        /// End address of the EEPROM in data space
        pub const EEPROM_END:       usize = 0x14FF;

        /// Page size of the EEPROM in data space
        pub const EEPROM_PAGE_SIZE: usize = 32;

    } else if #[cfg(any(
        feature = "avr32dd20",
    ))] {
        /// Start address of the flash in data space
        pub const FLASH_START:      usize = 0x8000;

        /// End address of the flash in data space
        pub const FLASH_END:        usize = 0xFFFF;

        /// Page size of the flash in data space
        pub const FLASH_PAGE_SIZE:  usize = 512;


        /// Start address of the EEPROM in data space
        pub const EEPROM_START:     usize = 0x1400;

        /// End address of the EEPROM in data space
        pub const EEPROM_END:       usize = 0x14FF;

        /// Page size of the EEPROM in data space
        pub const EEPROM_PAGE_SIZE: usize = 32;

    } else if #[cfg(any(
        feature = "attiny3216",
        feature = "attiny3217",
    ))] {
        /// Start address of the flash in data space
        pub const FLASH_START:      usize = 0x8000;
        
        /// End address of the flash in data space
        pub const FLASH_END:        usize = 0xFFFF;

        /// Page size of the flash in data space
        pub const FLASH_PAGE_SIZE:  usize = 128;


        /// Start address of the EEPROM in data space
        pub const EEPROM_START:     usize = 0x1400;

        /// End address of the EEPROM in data space
        pub const EEPROM_END:       usize = 0x14FF;

        /// Page size of the EEPROM in data space
        pub const EEPROM_PAGE_SIZE: usize = 64;
    }
}


impl crate::private::Sealed for NVMCTRL {}

pub trait NvmctrlExt: crate::private::Sealed {
    /// Create a [`FlashAccess`] instance that allows to read and write program flash pages
    fn flash(&self) -> FlashAccess;

    /// Create a [`EepromAccess`] instance that allows to read and write EEPROM pages
    fn eeprom(&self) -> EepromAccess;
}

impl NvmctrlExt for NVMCTRL {
    /// Get access to the Flash of the microcontroller for reading and writing
    fn flash(&self) -> FlashAccess {
        FlashAccess { nvmctrl: self }
    }

    /// Get access to the EEPROM of the microcontroller for reading and writing
    fn eeprom(&self) -> EepromAccess {
        EepromAccess { nvmctrl: self }
    }
}

/// Errors that can occur when reading or writing to Flash or EEPROM
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The hardware returned a write error condition.
    Write,

    /// The supplied offset and length would cause an out of bounds access when
    /// reading or writing Flash or EEPROM.
    OutOfBounds
}

/// The flash access module which allows reading from and writing to flash
pub struct FlashAccess<'a> {
    nvmctrl: &'a NVMCTRL
}

impl FlashAccess<'_> {
    /// Erase and write flash.
    /// 
    /// When calling this method, the flash is erased page-wise starting from
    /// `offset` and the data in the `bytes` slice is written to it afterwards.
    /// 
    /// Non-page-aligned write accesses are handled automatically.
    /// 
    /// Returns an [`Error::OutOfBounds`] in case data outside of the flash
    /// region defined by [`FLASH_START`] and [`FLASH_END`] is accessed.
    /// In case of a hardware write error [`Error::Write`] is returned.
    pub fn program(&self, offset: usize, bytes: &[u8]) -> Result<(), Error> {
        if FLASH_START + offset + bytes.len() - 1 > FLASH_END {
            return Err(Error::OutOfBounds);
        }

        let mut ptr = ((FLASH_START + offset) & !(FLASH_PAGE_SIZE-1)) as *mut u8;

        // Clear the page buffer
        self.nvmctrl_cmd(CMD_A::PBC)?;

        // Fill the page buffer with original data that should not be overwritten
        let start_offset = offset % FLASH_PAGE_SIZE;
        for _ in 0..start_offset {
            unsafe { 
                ptr::write_volatile(ptr, ptr::read_volatile(ptr));
                ptr = ptr.add(1);
            };
        }

        // Write the new data into the page buffer
        for b in bytes.iter() {
            unsafe { 
                ptr::write_volatile(ptr, *b);
                ptr = ptr.add(1);

                if ptr as usize % FLASH_PAGE_SIZE == 0 {
                    self.nvmctrl_cmd(CMD_A::ERWP)?;
                }
            };
        }

        // Write the remainder of the page into the page buffer
        if (ptr as usize) % FLASH_PAGE_SIZE > 0 {
            while (ptr as usize) % FLASH_PAGE_SIZE != 0 {
                unsafe {
                    ptr::write_volatile(ptr, ptr::read_volatile(ptr));
                    ptr = ptr.add(1);
                }
            }

            self.nvmctrl_cmd(CMD_A::ERWP)?;
        }

        Ok(())
    }

    /// Read from flash.
    /// 
    /// Returns a slice that gives raw access to the data stored in flash
    /// starting from `offset` with length `len`.
    /// 
    /// Returns an [`Error::OutOfBounds`] in case data outside of the flash
    /// region defined by [`FLASH_START`] and [`FLASH_END`] is accessed.
    pub fn read(&self, offset: usize, len: usize) -> Result<&[u8], Error> {
        if FLASH_START + offset + len - 1 > FLASH_END {
            return Err(Error::OutOfBounds);
        }

        let ptr = (FLASH_START + offset) as *mut u8;
        Ok(unsafe { core::slice::from_raw_parts(ptr, len) })
    }

    fn nvmctrl_cmd(&self, cmd: CMD_A) -> Result<(), Error> {
        self.nvmctrl.ctrla().write_protected(|w| { w.cmd().variant(cmd) });

        while self.nvmctrl.status().read().fbusy().bit_is_set() {}

        if self.nvmctrl.status().read().error().variant() != Some(avr_device::avr32dd20::nvmctrl::status::ERROR_A::NOERROR) {
            return Err(Error::Write);
        }

        Ok(())
    }
}

/// The EEPROM access module which allows reading from and writing to EEPROM
pub struct EepromAccess<'a> {
    nvmctrl: &'a NVMCTRL
}

impl EepromAccess<'_> {
    /// Erase and write EEPROM.
    /// 
    /// When calling this method, the EEPROM is erased byte-wise starting from
    /// `offset` and the data in the `bytes` slice is written to it afterwards.
    /// 
    /// Returns an [`Error::OutOfBounds`] in case data outside of the flash
    /// region defined by [`FLASH_START`] and [`FLASH_END`] is accessed.
    /// In case of a hardware write error [`Error::Write`] is returned.
    pub fn program(&self, offset: usize, bytes: &[u8]) -> Result<(), Error> {
        if EEPROM_START + offset + bytes.len() - 1 > EEPROM_END {
            return Err(Error::OutOfBounds);
        }

        let mut ptr = (EEPROM_START + offset) as *mut u8;

        // Clear the page buffer
        self.nvmctrl_cmd(CMD_A::PBC)?;

        // Write the new data into the page buffer and flush it
        // to the EEPROM when reaching a page boundary
        for b in bytes.iter() {
            unsafe { 
                ptr::write_volatile(ptr, *b);
                ptr = ptr.add(1);

                if ptr as usize % EEPROM_PAGE_SIZE == 0 {
                    self.nvmctrl_cmd(CMD_A::ERWP)?;
                }
            };
        }

        // Write the remaining bytes from the page buffer into the EEPROM
        if (ptr as usize) % FLASH_PAGE_SIZE > 0 {
            self.nvmctrl_cmd(CMD_A::ERWP)?;
        }

        Ok(())
    }

    /// Read from EEPROM.
    /// 
    /// Returns a slice that gives raw access to the data stored in EEPROM
    /// starting from `offset` with length `len`.
    /// 
    /// Returns an [`Error::OutOfBounds`] in case data outside of the flash
    /// region defined by [`FLASH_START`] and [`FLASH_END`] is accessed.
    pub fn read(&self, offset: usize, len: usize) -> Result<&[u8], Error> {
        if EEPROM_START + offset + len - 1 > EEPROM_END {
            return Err(Error::OutOfBounds);
        }

        let ptr = (EEPROM_START + offset) as *mut u8;
        Ok(unsafe { core::slice::from_raw_parts(ptr, len) })
    }

    fn nvmctrl_cmd(&self, cmd: CMD_A) -> Result<(), Error> {
        self.nvmctrl.ctrla().write_protected(|w| { w.cmd().variant(cmd) });

        while self.nvmctrl.status().read().eebusy().bit_is_set() {}

        if self.nvmctrl.status().read().error().variant() != Some(avr_device::avr32dd20::nvmctrl::status::ERROR_A::NOERROR) {
            return Err(Error::Write);
        }

        Ok(())
    }
}
