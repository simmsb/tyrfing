#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - External Reset, Power-on Reset and Watchdog Reset"]
    RESET = 0,
    #[doc = "1 - External Interrupt 0"]
    INT0 = 1,
    #[doc = "2 - Pin Change Interrupt"]
    PCINT = 2,
    #[doc = "3 - Timer/Counter1 Compare Match 1A"]
    TIMER1_COMPA = 3,
    #[doc = "4 - Timer/Counter1 Compare Match 1B"]
    TIMER1_COMPB = 4,
    #[doc = "5 - Timer/Counter1 Overflow"]
    TIMER1_OVF = 5,
    #[doc = "6 - Timer/Counter0 Overflow"]
    TIMER0_OVF = 6,
    #[doc = "7 - USI Start"]
    USI_START = 7,
    #[doc = "8 - USI Overflow"]
    USI_OVF = 8,
    #[doc = "9 - EEPROM Ready"]
    EE_RDY = 9,
    #[doc = "10 - Analog Comparator"]
    ANA_COMP = 10,
    #[doc = "11 - ADC Conversion Complete"]
    ADC = 11,
    #[doc = "12 - Watchdog Time-Out"]
    WDT = 12,
    #[doc = "13 - External Interrupt 1"]
    INT1 = 13,
    #[doc = "14 - Timer/Counter0 Compare Match A"]
    TIMER0_COMPA = 14,
    #[doc = "15 - Timer/Counter0 Compare Match B"]
    TIMER0_COMPB = 15,
    #[doc = "16 - ADC Conversion Complete"]
    TIMER0_CAPT = 16,
    #[doc = "17 - Timer/Counter1 Compare Match D"]
    TIMER1_COMPD = 17,
    #[doc = "18 - Timer/Counter1 Fault Protection"]
    FAULT_PROTECTION = 18,
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
            2 => Ok(Interrupt::PCINT),
            3 => Ok(Interrupt::TIMER1_COMPA),
            4 => Ok(Interrupt::TIMER1_COMPB),
            5 => Ok(Interrupt::TIMER1_OVF),
            6 => Ok(Interrupt::TIMER0_OVF),
            7 => Ok(Interrupt::USI_START),
            8 => Ok(Interrupt::USI_OVF),
            9 => Ok(Interrupt::EE_RDY),
            10 => Ok(Interrupt::ANA_COMP),
            11 => Ok(Interrupt::ADC),
            12 => Ok(Interrupt::WDT),
            13 => Ok(Interrupt::INT1),
            14 => Ok(Interrupt::TIMER0_COMPA),
            15 => Ok(Interrupt::TIMER0_COMPB),
            16 => Ok(Interrupt::TIMER0_CAPT),
            17 => Ok(Interrupt::TIMER1_COMPD),
            18 => Ok(Interrupt::FAULT_PROTECTION),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
