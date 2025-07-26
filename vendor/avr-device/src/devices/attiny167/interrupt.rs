#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - External Reset, Power-on Reset and Watchdog Reset"]
    RESET = 0,
    #[doc = "1 - External Interrupt Request 0"]
    INT0 = 1,
    #[doc = "2 - External Interrupt Request 1"]
    INT1 = 2,
    #[doc = "3 - Pin Change Interrupt Request 0"]
    PCINT0 = 3,
    #[doc = "4 - Pin Change Interrupt Request 1"]
    PCINT1 = 4,
    #[doc = "5 - Watchdog Time-Out Interrupt"]
    WDT = 5,
    #[doc = "6 - Timer/Counter1 Capture Event"]
    TIMER1_CAPT = 6,
    #[doc = "7 - Timer/Counter1 Compare Match 1A"]
    TIMER1_COMPA = 7,
    #[doc = "8 - Timer/Counter1 Compare Match 1B"]
    TIMER1_COMPB = 8,
    #[doc = "9 - Timer/Counter1 Overflow"]
    TIMER1_OVF = 9,
    #[doc = "10 - Timer/Counter0 Compare Match 0A"]
    TIMER0_COMPA = 10,
    #[doc = "11 - Timer/Counter0 Overflow"]
    TIMER0_OVF = 11,
    #[doc = "12 - LIN Transfer Complete"]
    LIN_TC = 12,
    #[doc = "13 - LIN Error"]
    LIN_ERR = 13,
    #[doc = "14 - SPI Serial Transfer Complete"]
    SPI_STC = 14,
    #[doc = "15 - ADC Conversion Complete"]
    ADC = 15,
    #[doc = "16 - EEPROM Ready"]
    EE_RDY = 16,
    #[doc = "17 - Analog Comparator"]
    ANA_COMP = 17,
    #[doc = "18 - USI Start Condition Detection"]
    USI_START = 18,
    #[doc = "19 - USI Counter Overflow"]
    USI_OVF = 19,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::RESET),
            1 => Ok(Interrupt::INT0),
            2 => Ok(Interrupt::INT1),
            3 => Ok(Interrupt::PCINT0),
            4 => Ok(Interrupt::PCINT1),
            5 => Ok(Interrupt::WDT),
            6 => Ok(Interrupt::TIMER1_CAPT),
            7 => Ok(Interrupt::TIMER1_COMPA),
            8 => Ok(Interrupt::TIMER1_COMPB),
            9 => Ok(Interrupt::TIMER1_OVF),
            10 => Ok(Interrupt::TIMER0_COMPA),
            11 => Ok(Interrupt::TIMER0_OVF),
            12 => Ok(Interrupt::LIN_TC),
            13 => Ok(Interrupt::LIN_ERR),
            14 => Ok(Interrupt::SPI_STC),
            15 => Ok(Interrupt::ADC),
            16 => Ok(Interrupt::EE_RDY),
            17 => Ok(Interrupt::ANA_COMP),
            18 => Ok(Interrupt::USI_START),
            19 => Ok(Interrupt::USI_OVF),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
