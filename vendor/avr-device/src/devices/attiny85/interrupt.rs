#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - External Pin, Power-on Reset, Brown-out Reset,Watchdog Reset"]
    RESET = 0,
    #[doc = "1 - External Interrupt 0"]
    INT0 = 1,
    #[doc = "2 - Pin change Interrupt Request 0"]
    PCINT0 = 2,
    #[doc = "3 - Timer/Counter1 Compare Match 1A"]
    TIMER1_COMPA = 3,
    #[doc = "4 - Timer/Counter1 Overflow"]
    TIMER1_OVF = 4,
    #[doc = "5 - Timer/Counter0 Overflow"]
    TIMER0_OVF = 5,
    #[doc = "6 - EEPROM Ready"]
    EE_RDY = 6,
    #[doc = "7 - Analog comparator"]
    ANA_COMP = 7,
    #[doc = "8 - ADC Conversion ready"]
    ADC = 8,
    #[doc = "9 - Timer/Counter1 Compare Match B"]
    TIMER1_COMPB = 9,
    #[doc = "10 - Timer/Counter0 Compare Match A"]
    TIMER0_COMPA = 10,
    #[doc = "11 - Timer/Counter0 Compare Match B"]
    TIMER0_COMPB = 11,
    #[doc = "12 - Watchdog Time-out"]
    WDT = 12,
    #[doc = "13 - USI START"]
    USI_START = 13,
    #[doc = "14 - USI Overflow"]
    USI_OVF = 14,
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
            2 => Ok(Interrupt::PCINT0),
            3 => Ok(Interrupt::TIMER1_COMPA),
            4 => Ok(Interrupt::TIMER1_OVF),
            5 => Ok(Interrupt::TIMER0_OVF),
            6 => Ok(Interrupt::EE_RDY),
            7 => Ok(Interrupt::ANA_COMP),
            8 => Ok(Interrupt::ADC),
            9 => Ok(Interrupt::TIMER1_COMPB),
            10 => Ok(Interrupt::TIMER0_COMPA),
            11 => Ok(Interrupt::TIMER0_COMPB),
            12 => Ok(Interrupt::WDT),
            13 => Ok(Interrupt::USI_START),
            14 => Ok(Interrupt::USI_OVF),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
