//! no_std driver for the MS5611 (barometric pressure sensor)
//!
//!
//! # Usage
//!
//! Use embedded-hal implementation to get SPI, NCS, and delay, then create
//! a ms5611 handle
//!
//! ```ignore
//! // Create handle
//! let ms5611 = Ms5611::new(spi, ncs, Delay)?;
//! // Get a second order compensated pressure (and thermo) sample,
//! let sample = ms5611.get_second_order_sample(Oversampling::OS_2048)?;
//! println!("{:?}", sample);
//! ```
//!
//! # References
//!
//! - [Product specification][1]
//!
//! [1]: https://www.te.com/commerce/DocumentDelivery/DDEController?Action=showdoc&DocId=Data+Sheet%7FMS5611-01BA03%7FB3%7Fpdf%7FEnglish%7FENG_DS_MS5611-01BA03_B3.pdf%7FCAT-BLPS0036

#![cfg_attr(not(test), no_std)]
extern crate embedded_hal as hal;

// Feature gate for MS5607
#[cfg(all(feature = "ms5611", feature = "ms5607"))]
compile_error!("Cannot specify both ms5611 and ms5607 flags. Try setting default-features = false");

use hal::blocking::delay::DelayMs;
use hal::blocking::spi::{Transfer, Write};
use hal::digital::v2::OutputPin;

/// MS5611 driver
pub struct Ms5611<SPI, NCS> {
    spi: SPI,
    ncs: NCS,
    coeffs: Coefficients,
}

impl<SPI, NCS, E> Ms5611<SPI, NCS>
where
    SPI: Transfer<u8, Error = E> + Write<u8, Error = E>,
    NCS: OutputPin,
{
    /// Creates a new MS5611 driver from a SPI peripheral and a NCS pin
    pub fn new(spi: SPI, ncs: NCS, delay: &mut impl DelayMs<u8>) -> Result<Ms5611<SPI, NCS>, E> {
        let mut ms5611 = Ms5611 {
            spi,
            ncs,
            coeffs: Coefficients::default(),
        };

        ms5611.reset(delay)?;
        ms5611.coeffs = ms5611.read_coefficients()?;
        assert!(ms5611.coeffs.check_crc());

        Ok(ms5611)
    }

    /// Return the temperature compensation variables `dt`, `sens` and
    /// `off` from the raw sample temperature and the coefficients.
    fn get_temperature_compensation(&self, raw_sample: Sample) -> (i64, i64, i64) {
        let dt: i64 = ((raw_sample.temperature as i32)
            - ((self.coeffs.get_data(CoefficientsAddr::COEFF_5) as i32) << 8))
            as i64;

        #[cfg(any(feature = "ms5611"))]
        let (offset_sh, sens_sh) = ((16, 7), (15, 8));
        #[cfg(any(feature = "ms5607"))]
        let (offset_sh, sens_sh) = ((17, 6), (16, 7));

        let off = ((self.coeffs.get_data(CoefficientsAddr::COEFF_2) as i64) << offset_sh.0)
            + (((self.coeffs.get_data(CoefficientsAddr::COEFF_4) as i64) * dt) >> offset_sh.1);

        let sens = ((self.coeffs.get_data(CoefficientsAddr::COEFF_1) as i64) << sens_sh.0)
            + (((self.coeffs.get_data(CoefficientsAddr::COEFF_3) as i64) * dt) >> sens_sh.1);

        (dt, sens, off)
    }

    /// Reads and returns Pressure and Thermometer measurement
    pub fn get_compensated_sample(&mut self, osr: Oversampling, delay_source: &mut impl DelayMs<u8>) -> Result<Sample, E> {
        let raw_sample = self.read_raw_sample(osr, delay_source)?;

        // Get temperature compensation constants
        let (dt, sens, off) = self.get_temperature_compensation(raw_sample);

        let temperature = (2000i64
            + ((dt as i64) * (self.coeffs.get_data(CoefficientsAddr::COEFF_6) as i64) >> 23))
            as i32;
        let pressure = (((raw_sample.pressure as i64) * sens >> 21) - off) >> 15;

        let sample = Sample {
            pressure: pressure as i32,
            temperature: temperature as i32,
        };
        Ok(sample)
    }

    /// Reads and returns a second order compensated Pressure and Thermometer
    /// measurement as defined in datasheet.
    pub fn get_second_order_sample(&mut self, osr: Oversampling, delay_source: &mut impl DelayMs<u8>) -> Result<Sample, E> {
        let raw_sample = self.read_raw_sample(osr, delay_source)?;

        // Get temperature compensation constants
        let (dt, mut sens, mut off) = self.get_temperature_compensation(raw_sample);

        let mut temperature = (2000i64
            + ((dt as i64) * (self.coeffs.get_data(CoefficientsAddr::COEFF_6) as i64) >> 23))
            as i32;

        // Check low temp
        let mut offsets = if temperature < 2000 {
            (
                ((dt as i64 * dt as i64) >> 31) as i32,
                (5 * (temperature - 2000) * (temperature - 2000)) >> 1 as i32,
                (5 * (temperature - 2000) * (temperature - 2000)) >> 2 as i32,
            )
        } else {
            (0, 0, 0)
        };

        // Check very low temp
        if temperature < -1500 {
            offsets.0 = offsets.0 + 7 * (temperature + 1500) * (temperature + 1500);
            offsets.1 = offsets.1 + 11 * ((temperature + 1500) * (temperature + 1500) >> 1);
        }

        off -= offsets.1 as i64;
        sens -= offsets.2 as i64;

        temperature = temperature - offsets.0;
        let pressure = (((raw_sample.pressure as i64) * sens >> 21) - off) >> 15;

        let sample = Sample {
            pressure: pressure as i32,
            temperature: temperature as i32,
        };
        Ok(sample)
    }

    fn send(&mut self, addr: u8) -> Result<(), E> {
        let _ = self.ncs.set_low();
        self.spi.write(&[addr])?;
        let _ = self.ncs.set_high();
        Ok(())
    }

    fn read_raw(&mut self, addr: u8) -> Result<u32, E> {
        let mut buffer = [0; 4];
        buffer[0] = addr;
        let _ = self.ncs.set_low();
        self.spi.transfer(&mut buffer)?;
        let _ = self.ncs.set_high();

        let r = ((buffer[1] as u32) << 16) | ((buffer[2] as u32) << 8) | (buffer[3] as u32);

        Ok(r)
    }

    fn read_raw_u16(&mut self, addr: u8) -> Result<u16, E> {
        let mut buffer = [0; 3];
        buffer[0] = addr;
        let _ = self.ncs.set_low();
        self.spi.transfer(&mut buffer)?;
        let _ = self.ncs.set_high();

        let r = ((buffer[1] as u16) << 8) | (buffer[2] as u16);

        Ok(r)
    }

    fn read_raw_sample(&mut self, osr: Oversampling, delay_source: &mut impl  DelayMs<u8>) -> Result<Sample, E> {
        // Start convertion of D1 (pressure)
        self.send(Command::CONV_D1.address() + osr.offset())?;
        delay_source.delay_ms(osr.delay());
        let raw_pressure = self.read_raw(Command::ADC_READ.address())?;

        // Start convertion of D2 (temperature)
        self.send(Command::CONV_D2.address() + osr.offset())?;
        delay_source.delay_ms(osr.delay());
        let raw_temperature = self.read_raw(Command::ADC_READ.address())?;

        let sample = Sample {
            pressure: raw_pressure as i32,
            temperature: raw_temperature as i32,
        };

        Ok(sample)
    }

    fn reset(&mut self, delay_source: &mut impl DelayMs<u8>) -> Result<(), E> {
        self.send(Command::RESET.address())?;
        delay_source.delay_ms(3);
        Ok(())
    }

    fn read_coefficients(&mut self) -> Result<Coefficients, E> {
        let mut buffer = [0x00u16; 8];

        buffer[CoefficientsAddr::MANUFACTURER as usize >> 1] =
            self.read_raw_u16(Command::prom_address(CoefficientsAddr::MANUFACTURER))?;
        buffer[CoefficientsAddr::COEFF_1 as usize >> 1] =
            self.read_raw_u16(Command::prom_address(CoefficientsAddr::COEFF_1))?;
        buffer[CoefficientsAddr::COEFF_2 as usize >> 1] =
            self.read_raw_u16(Command::prom_address(CoefficientsAddr::COEFF_2))?;
        buffer[CoefficientsAddr::COEFF_3 as usize >> 1] =
            self.read_raw_u16(Command::prom_address(CoefficientsAddr::COEFF_3))?;
        buffer[CoefficientsAddr::COEFF_4 as usize >> 1] =
            self.read_raw_u16(Command::prom_address(CoefficientsAddr::COEFF_4))?;
        buffer[CoefficientsAddr::COEFF_5 as usize >> 1] =
            self.read_raw_u16(Command::prom_address(CoefficientsAddr::COEFF_5))?;
        buffer[CoefficientsAddr::COEFF_6 as usize >> 1] =
            self.read_raw_u16(Command::prom_address(CoefficientsAddr::COEFF_6))?;
        buffer[CoefficientsAddr::CRC as usize >> 1] =
            self.read_raw_u16(Command::prom_address(CoefficientsAddr::CRC))?;

        Ok(Coefficients { data: buffer })
    }
}

/// Pressure and Thermometer measurement
#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq)]
pub struct Sample {
    pub pressure: i32,
    pub temperature: i32,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
enum Command {
    RESET = 0x1E,
    CONV_D1 = 0x40,
    CONV_D2 = 0x50,
    ADC_READ = 0x00,
    PROM_BASE = 0xA0,
}

impl Command {
    pub fn address(self) -> u8 {
        self as u8
    }

    pub fn prom_address(offset: CoefficientsAddr) -> u8 {
        Command::PROM_BASE.address() + offset as u8
    }
}


/// Oversampling rates as defined in datasheet
/// defines for how long reading a sample will block
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Oversampling {
    OS_256,
    OS_512,
    OS_1024,
    OS_2048,
    OS_4096,
}

impl Oversampling {
    pub fn offset(self) -> u8 {
        match self {
            Oversampling::OS_256 => 0,
            Oversampling::OS_512 => 2,
            Oversampling::OS_1024 => 4,
            Oversampling::OS_2048 => 6,
            Oversampling::OS_4096 => 8,
        }
    }

    pub fn delay(self) -> u8 {
        match self {
            Oversampling::OS_256 => 1,
            Oversampling::OS_512 => 2,
            Oversampling::OS_1024 => 3,
            Oversampling::OS_2048 => 5,
            Oversampling::OS_4096 => 10,
        }
    }
}

/// Default factory coefficients
#[derive(ufmt::derive::uDebug, Debug, Default)]
struct Coefficients {
    data: [u16; 8],
}

#[allow(non_camel_case_types)]
enum CoefficientsAddr {
    MANUFACTURER = 0x0,
    COEFF_1 = 0x2,
    COEFF_2 = 0x4,
    COEFF_3 = 0x6,
    COEFF_4 = 0x8,
    COEFF_5 = 0xA,
    COEFF_6 = 0xC,
    CRC = 0xE,
}

impl Coefficients {
    fn get_data(&self, addr: CoefficientsAddr) -> u16 {
        (self.data[addr as usize >> 1])
    }

    fn get_crc(&self) -> u8 {
        ((self.get_data(CoefficientsAddr::CRC) & 0xF) as u8)
    }

    pub fn check_crc(&self) -> bool {
        let mut crc: u16 = 0;
        let data_crc = self.get_crc() as u16;
        for item in self.data[..self.data.len() - 1].iter() {
            crc = Self::crc_coefficient(crc, item);
        }
        crc = Self::crc_coefficient(crc, &(self.get_data(CoefficientsAddr::CRC) & 0xFF00));

        crc = (crc >> 12) & 0xF;
        (crc == data_crc)
    }

    fn crc_coefficient(crc: u16, coefficient: &u16) -> u16 {
        let mut crc = crc;
        crc ^= (coefficient >> 8) & 0xFFu16;
        crc = Self::crc_round(crc);
        crc ^= coefficient & 0xFF;
        crc = Self::crc_round(crc);
        (crc)
    }

    fn crc_round(crc: u16) -> u16 {
        let mut crc = crc;
        for _ in (1..9).rev() {
            crc = if (crc & 0x8000) > 0 {
                (crc << 1) ^ 0x3000
            } else {
                (crc << 1)
            }
        }
        (crc)
    }
}

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock;

    use self::embedded_hal_mock::delay::MockNoop;
    use self::embedded_hal_mock::spi::{Mock as SpiMock, Transaction as SpiTransaction};
    use super::*;

    struct Pin;

    impl hal::digital::v2::OutputPin for Pin {
	type Error = u32;
        fn set_low(&mut self) -> Result<(), Self::Error> { Ok(()) }
        fn set_high(&mut self) -> Result<(), Self::Error> { Ok(()) }
    }

    #[test]
    fn get_coeffs() {
        let data = [
            0x3132, 0x3334, 0x3536, 0x3738, 0x3940, 0x4142, 0x4344, 0x450b,
        ];
        let coeffs = Coefficients { data };

        assert_eq!(coeffs.get_data(CoefficientsAddr::MANUFACTURER), 0x3132);
        assert_eq!(coeffs.get_data(CoefficientsAddr::COEFF_1), 0x3334);
        assert_eq!(coeffs.get_data(CoefficientsAddr::COEFF_2), 0x3536);
        assert_eq!(coeffs.get_data(CoefficientsAddr::COEFF_3), 0x3738);
        assert_eq!(coeffs.get_data(CoefficientsAddr::COEFF_4), 0x3940);
        assert_eq!(coeffs.get_data(CoefficientsAddr::COEFF_5), 0x4142);
        assert_eq!(coeffs.get_data(CoefficientsAddr::COEFF_6), 0x4344);
        assert_eq!(coeffs.get_data(CoefficientsAddr::CRC), 0x450b);

        assert_eq!(coeffs.get_crc(), 0xb);
    }

    #[test]
    fn check_crc() {
        let mut data = [
            0x0024, 0xB3D8, 0xBD83, 0x6E00, 0x628A, 0x8063, 0x6ADB, 0x947B,
        ];
        let coeffs = Coefficients { data };
        assert_eq!(coeffs.check_crc(), true);

        data[7] = 0x460b;
        let coeffs = Coefficients { data };
        assert_eq!(coeffs.check_crc(), false);
    }

    #[test]
    #[cfg(feature = "ms5611")]
    fn read_compensated_samples_ms5611() {
        /* The following values are taken from the example in the datasheet */
        let expectations = [
            SpiTransaction::write(vec![0x1E]),
            SpiTransaction::transfer(vec![0xA0, 0, 0], vec![0, 0x00, 0x00]),
            SpiTransaction::transfer(vec![0xA2, 0, 0], vec![0, 0x9C, 0xBF]), // 40127
            SpiTransaction::transfer(vec![0xA4, 0, 0], vec![0, 0x90, 0x3C]), // 36924
            SpiTransaction::transfer(vec![0xA6, 0, 0], vec![0, 0x5B, 0x15]), // 23317
            SpiTransaction::transfer(vec![0xA8, 0, 0], vec![0, 0x5A, 0xF2]), // 23282
            SpiTransaction::transfer(vec![0xAA, 0, 0], vec![0, 0x82, 0xB8]), // 33464
            SpiTransaction::transfer(vec![0xAC, 0, 0], vec![0, 0x6E, 0x98]), // 28312
            SpiTransaction::transfer(vec![0xAE, 0, 0], vec![0, 0x00, 0x00]),
            SpiTransaction::write(vec![0x46]), // Convert D1 OSR=2048
            SpiTransaction::transfer(vec![0, 0, 0, 0], vec![0, 0x8A, 0xA2, 0x1A]), // 9085466
            SpiTransaction::write(vec![0x56]), // Convert D2 OSR=2048
            SpiTransaction::transfer(vec![0, 0, 0, 0], vec![0, 0x82, 0xC1, 0x3E]), // 9085466
            SpiTransaction::write(vec![0x46]), // Convert D1 OSR=2048
            SpiTransaction::transfer(vec![0, 0, 0, 0], vec![0, 0x8A, 0xA2, 0x1A]), // 9085466
            SpiTransaction::write(vec![0x56]), // Convert D2 OSR=2048
            SpiTransaction::transfer(vec![0, 0, 0, 0], vec![0, 0x82, 0xC1, 0x3E]), // 9085466
        ];

        let spi = SpiMock::new(&expectations);
        let pin = Pin;
        let mut delay_source = MockNoop::new();
        let mut ms5611 = Ms5611::new(spi, pin, &mut delay_source).unwrap();
        let sample1 = ms5611
            .get_compensated_sample(Oversampling::OS_2048, &mut delay_source)
            .unwrap();
        let sample2 = ms5611
            .get_second_order_sample(Oversampling::OS_2048, &mut delay_source)
            .unwrap();

        assert_eq!(
            Sample {
                pressure: 100009,
                temperature: 2007
            },
            sample1
        );
        assert_eq!(sample1, sample2);
    }

    #[test]
    #[cfg(feature = "ms5607")]
    fn read_compensated_samples_ms5607() {
        /* The following values are taken from the example in the datasheet */
        let expectations = [
            SpiTransaction::write(vec![0x1E]),
            SpiTransaction::transfer(vec![0xA0, 0, 0], vec![0, 0x00, 0x00]),
            SpiTransaction::transfer(vec![0xA2, 0, 0], vec![0, 0xB5, 0x24]), // 46372
            SpiTransaction::transfer(vec![0xA4, 0, 0], vec![0, 0xAB, 0xCD]), // 43981
            SpiTransaction::transfer(vec![0xA6, 0, 0], vec![0, 0x71, 0x83]), // 29059
            SpiTransaction::transfer(vec![0xA8, 0, 0], vec![0, 0x6C, 0xC2]), // 27842
            SpiTransaction::transfer(vec![0xAA, 0, 0], vec![0, 0x7B, 0x41]), // 31553
            SpiTransaction::transfer(vec![0xAC, 0, 0], vec![0, 0x6E, 0x05]), // 28165
            SpiTransaction::transfer(vec![0xAE, 0, 0], vec![0, 0x00, 0x08]),
            SpiTransaction::write(vec![0x46]), // Convert D1 OSR=2048
            SpiTransaction::transfer(vec![0, 0, 0, 0], vec![0, 0x62, 0xA7, 0xA4]), // 6465444
            SpiTransaction::write(vec![0x56]), // Convert D2 OSR=2048
            SpiTransaction::transfer(vec![0, 0, 0, 0], vec![0, 0x7B, 0x41, 0x44]), // 8077636
            SpiTransaction::write(vec![0x46]), // Convert D1 OSR=2048
            SpiTransaction::transfer(vec![0, 0, 0, 0], vec![0, 0x62, 0xA7, 0xA4]), // 6465444
            SpiTransaction::write(vec![0x56]), // Convert D2 OSR=2048
            SpiTransaction::transfer(vec![0, 0, 0, 0], vec![0, 0x7B, 0x41, 0x44]), // 8077636
        ];

        let spi = SpiMock::new(&expectations);
        let pin = Pin;
        let mut delay_source = MockNoop::new();
        let mut ms5611 = Ms5611::new(spi, pin, &mut delay_source).unwrap();
        let sample1 = ms5611
            .get_compensated_sample(Oversampling::OS_2048, &mut delay_source)
            .unwrap();
        let sample2 = ms5611
            .get_second_order_sample(Oversampling::OS_2048, &mut delay_source)
            .unwrap();

        assert_eq!(
            Sample {
                pressure: 110002,
                temperature: 2000
            },
            sample1
        );
        assert_eq!(sample1, sample2);
    }
}
