#![no_std]
#![no_main]

use panic_halt as _;

use atxtiny_hal::prelude::*;
use atxtiny_hal::pac;
use atxtiny_hal::spi::Spi;
use atxtiny_hal::serial::Serial;

use atxtiny_hal::embedded_hal::spi::SpiDevice;
use atxtiny_hal::embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Constrain a few peripherals into our HAL types
    let clkctrl = dp.CLKCTRL.constrain();
    let portmux = dp.PORTMUX.constrain();

    // Configure our clocks
    let clocks = clkctrl.freeze();

    // Split the POARTA and PORTC peripheral into its pins
    let (a, c) = (dp.PORTA.split(), dp.PORTC.split());

    // Serial port setup
    let usart_pair = (a.pa2.into_peripheral::<pac::USART0>(), a.pa1.into_peripheral::<pac::USART0>()).mux(&portmux);
    let mut s = Serial::new(dp.USART0, usart_pair, 115200u32.bps(), clocks);

    // Grab the SPI pins
    let sckpin = c.pc0.into_peripheral();
    let misopin = c.pc1.into_peripheral();
    let mosipin = c.pc2.into_peripheral();
    let mut cs_ms = c.pc3.into_stateless_push_pull_output();
    let mut cs_mpu = c.pc4.into_stateless_push_pull_output();

    // Deselect any chip-selects
    cs_ms.set_high().unwrap();
    cs_mpu.set_high().unwrap();

    // Multiplex the SPI pins
    let spi_pair = (sckpin, misopin, mosipin);
    let spi_pair = spi_pair.mux(&portmux);

    // Create an SPI abstraction
    let spi = Spi::new_unbuffered(dp.SPI0, spi_pair, 625_000.Hz(), clocks);

    // Create an SpiDevice for the MS5611
    let mut ms5611 = ExclusiveDevice::new(spi, cs_ms, NoDelay);

    // Read MS5611 PROM
    let mut prom = [0u16; 8];
    for i in 0..8 {
        let mut buf = [0xA0 + i*2, 0xFF, 0xFF];
        ms5611.transfer_in_place(&mut buf).unwrap();

        prom[i as usize] = ((buf[1] as u16) << 8) | (buf[2] as u16);
    }

    let c = Coefficients { data: prom };

    if c.check_crc() {
        ufmt::uwriteln!(s, "CRC of MS5611 PROM correct!").unwrap();
    }

    ufmt::uwrite!(s, "Calibration coefficients: ").unwrap();
    for b in prom {
        ufmt::uwrite!(s, "{:04x} ", b).unwrap();
    }
    ufmt::uwriteln!(s, "").unwrap();

    loop { }
}

/// MSP5611 default factory coefficients
#[derive(ufmt::derive::uDebug, Debug, Default)]
pub struct Coefficients {
    data: [u16; 8],
}

#[allow(non_camel_case_types)]
pub enum CoefficientsAddr {
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
    pub fn get_data(&self, addr: CoefficientsAddr) -> u16 {
        self.data[addr as usize >> 1]
    }

    fn get_crc(&self) -> u8 {
        (self.get_data(CoefficientsAddr::CRC) & 0xF) as u8
    }

    pub fn check_crc(&self) -> bool {
        let mut crc: u16 = 0;
        let data_crc = self.get_crc() as u16;
        for item in self.data[..self.data.len() - 1].iter() {
            crc = Self::crc_coefficient(crc, item);
        }
        crc = Self::crc_coefficient(crc, &(self.get_data(CoefficientsAddr::CRC) & 0xFF00));

        crc = (crc >> 12) & 0xF;
        crc == data_crc
    }

    fn crc_coefficient(crc: u16, coefficient: &u16) -> u16 {
        let mut crc = crc;
        crc ^= (coefficient >> 8) & 0xFFu16;
        crc = Self::crc_round(crc);
        crc ^= coefficient & 0xFF;
        crc = Self::crc_round(crc);
        crc
    }

    fn crc_round(crc: u16) -> u16 {
        let mut crc = crc;
        for _ in (1..9).rev() {
            crc = if (crc & 0x8000) > 0 {
                (crc << 1) ^ 0x3000
            } else {
                crc << 1
            }
        }
        crc
    }
}
