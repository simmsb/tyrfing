#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - External Pin, Power-on Reset, Brown-out Reset and Watchdog Reset"]
    RESET = 0,
    #[doc = "1 - External Interrupt Request 0"]
    INT0 = 1,
    #[doc = "2 - External Interrupt Request 1"]
    INT1 = 2,
    #[doc = "3 - Pin Change Interrupt Request 0"]
    PCINT0 = 3,
    #[doc = "4 - Pin Change Interrupt Request 1"]
    PCINT1 = 4,
    #[doc = "5 - Pin Change Interrupt Request 2"]
    PCINT2 = 5,
    #[doc = "6 - Pin Change Interrupt Request 3"]
    PCINT3 = 6,
    #[doc = "7 - Watchdog Time-out Interrupt"]
    WDT = 7,
    #[doc = "8 - Timer/Counter1 Capture Event"]
    TIMER1_CAPT = 8,
    #[doc = "9 - Timer/Counter1 Compare Match A"]
    TIMER1_COMPA = 9,
    #[doc = "10 - Timer/Counter1 Compare Match B"]
    TIMER1_COMPB = 10,
    #[doc = "11 - Timer/Counter1 Overflow"]
    TIMER1_OVF = 11,
    #[doc = "12 - TimerCounter0 Compare Match A"]
    TIMER0_COMPA = 12,
    #[doc = "13 - TimerCounter0 Compare Match B"]
    TIMER0_COMPB = 13,
    #[doc = "14 - Timer/Couner0 Overflow"]
    TIMER0_OVF = 14,
    #[doc = "15 - SPI Serial Transfer Complete"]
    SPI_STC = 15,
    #[doc = "16 - ADC Conversion Complete"]
    ADC = 16,
    #[doc = "17 - EEPROM Ready"]
    EE_RDY = 17,
    #[doc = "18 - Analog Comparator"]
    ANALOG_COMP = 18,
    #[doc = "19 - Two-wire Serial Interface"]
    TWI = 19,
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
            5 => Ok(Interrupt::PCINT2),
            6 => Ok(Interrupt::PCINT3),
            7 => Ok(Interrupt::WDT),
            8 => Ok(Interrupt::TIMER1_CAPT),
            9 => Ok(Interrupt::TIMER1_COMPA),
            10 => Ok(Interrupt::TIMER1_COMPB),
            11 => Ok(Interrupt::TIMER1_OVF),
            12 => Ok(Interrupt::TIMER0_COMPA),
            13 => Ok(Interrupt::TIMER0_COMPB),
            14 => Ok(Interrupt::TIMER0_OVF),
            15 => Ok(Interrupt::SPI_STC),
            16 => Ok(Interrupt::ADC),
            17 => Ok(Interrupt::EE_RDY),
            18 => Ok(Interrupt::ANALOG_COMP),
            19 => Ok(Interrupt::TWI),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
