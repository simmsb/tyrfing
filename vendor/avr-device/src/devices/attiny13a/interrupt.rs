#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - External Reset, Power-on Reset and Watchdog Reset"]
    RESET = 0,
    #[doc = "1 - External Interrupt 0"]
    INT0 = 1,
    #[doc = "2 - External Interrupt Request 0"]
    PCINT0 = 2,
    #[doc = "3 - Timer/Counter0 Overflow"]
    TIM0_OVF = 3,
    #[doc = "4 - EEPROM Ready"]
    EE_RDY = 4,
    #[doc = "5 - Analog Comparator"]
    ANA_COMP = 5,
    #[doc = "6 - Timer/Counter Compare Match A"]
    TIM0_COMPA = 6,
    #[doc = "7 - Timer/Counter Compare Match B"]
    TIM0_COMPB = 7,
    #[doc = "8 - Watchdog Time-out"]
    WDT = 8,
    #[doc = "9 - ADC Conversion Complete"]
    ADC = 9,
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
            3 => Ok(Interrupt::TIM0_OVF),
            4 => Ok(Interrupt::EE_RDY),
            5 => Ok(Interrupt::ANA_COMP),
            6 => Ok(Interrupt::TIM0_COMPA),
            7 => Ok(Interrupt::TIM0_COMPB),
            8 => Ok(Interrupt::WDT),
            9 => Ok(Interrupt::ADC),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
