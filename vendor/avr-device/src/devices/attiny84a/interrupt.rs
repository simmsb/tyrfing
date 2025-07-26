#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - External Pin, Power-on Reset, Brown-out Reset,Watchdog Reset"]
    RESET = 0,
    #[doc = "1 - External Interrupt Request 0"]
    EXT_INT0 = 1,
    #[doc = "2 - Pin Change Interrupt Request 0"]
    PCINT0 = 2,
    #[doc = "3 - Pin Change Interrupt Request 1"]
    PCINT1 = 3,
    #[doc = "4 - Watchdog Time-out"]
    WDT = 4,
    #[doc = "5 - Timer/Counter1 Capture Event"]
    TIM1_CAPT = 5,
    #[doc = "6 - Timer/Counter1 Compare Match A"]
    TIM1_COMPA = 6,
    #[doc = "7 - Timer/Counter1 Compare Match B"]
    TIM1_COMPB = 7,
    #[doc = "8 - Timer/Counter1 Overflow"]
    TIM1_OVF = 8,
    #[doc = "9 - Timer/Counter0 Compare Match A"]
    TIM0_COMPA = 9,
    #[doc = "10 - Timer/Counter0 Compare Match B"]
    TIM0_COMPB = 10,
    #[doc = "11 - Timer/Counter0 Overflow"]
    TIM0_OVF = 11,
    #[doc = "12 - Analog Comparator"]
    ANA_COMP = 12,
    #[doc = "13 - ADC Conversion Complete"]
    ADC = 13,
    #[doc = "14 - EEPROM Ready"]
    EE_RDY = 14,
    #[doc = "15 - USI START"]
    USI_STR = 15,
    #[doc = "16 - USI Overflow"]
    USI_OVF = 16,
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
            1 => Ok(Interrupt::EXT_INT0),
            2 => Ok(Interrupt::PCINT0),
            3 => Ok(Interrupt::PCINT1),
            4 => Ok(Interrupt::WDT),
            5 => Ok(Interrupt::TIM1_CAPT),
            6 => Ok(Interrupt::TIM1_COMPA),
            7 => Ok(Interrupt::TIM1_COMPB),
            8 => Ok(Interrupt::TIM1_OVF),
            9 => Ok(Interrupt::TIM0_COMPA),
            10 => Ok(Interrupt::TIM0_COMPB),
            11 => Ok(Interrupt::TIM0_OVF),
            12 => Ok(Interrupt::ANA_COMP),
            13 => Ok(Interrupt::ADC),
            14 => Ok(Interrupt::EE_RDY),
            15 => Ok(Interrupt::USI_STR),
            16 => Ok(Interrupt::USI_OVF),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
