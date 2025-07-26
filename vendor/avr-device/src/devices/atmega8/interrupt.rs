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
    #[doc = "3 - Timer/Counter2 Compare Match"]
    TIMER2_COMP = 3,
    #[doc = "4 - Timer/Counter2 Overflow"]
    TIMER2_OVF = 4,
    #[doc = "5 - Timer/Counter1 Capture Event"]
    TIMER1_CAPT = 5,
    #[doc = "6 - Timer/Counter1 Compare Match A"]
    TIMER1_COMPA = 6,
    #[doc = "7 - Timer/Counter1 Compare Match B"]
    TIMER1_COMPB = 7,
    #[doc = "8 - Timer/Counter1 Overflow"]
    TIMER1_OVF = 8,
    #[doc = "9 - Timer/Counter0 Overflow"]
    TIMER0_OVF = 9,
    #[doc = "10 - Serial Transfer Complete"]
    SPI_STC = 10,
    #[doc = "11 - USART, Rx Complete"]
    USART_RXC = 11,
    #[doc = "12 - USART Data Register Empty"]
    USART_UDRE = 12,
    #[doc = "13 - USART, Tx Complete"]
    USART_TXC = 13,
    #[doc = "14 - ADC Conversion Complete"]
    ADC = 14,
    #[doc = "15 - EEPROM Ready"]
    EE_RDY = 15,
    #[doc = "16 - Analog Comparator"]
    ANA_COMP = 16,
    #[doc = "17 - 2-wire Serial Interface"]
    TWI = 17,
    #[doc = "18 - Store Program Memory Ready"]
    SPM_RDY = 18,
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
            3 => Ok(Interrupt::TIMER2_COMP),
            4 => Ok(Interrupt::TIMER2_OVF),
            5 => Ok(Interrupt::TIMER1_CAPT),
            6 => Ok(Interrupt::TIMER1_COMPA),
            7 => Ok(Interrupt::TIMER1_COMPB),
            8 => Ok(Interrupt::TIMER1_OVF),
            9 => Ok(Interrupt::TIMER0_OVF),
            10 => Ok(Interrupt::SPI_STC),
            11 => Ok(Interrupt::USART_RXC),
            12 => Ok(Interrupt::USART_UDRE),
            13 => Ok(Interrupt::USART_TXC),
            14 => Ok(Interrupt::ADC),
            15 => Ok(Interrupt::EE_RDY),
            16 => Ok(Interrupt::ANA_COMP),
            17 => Ok(Interrupt::TWI),
            18 => Ok(Interrupt::SPM_RDY),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
