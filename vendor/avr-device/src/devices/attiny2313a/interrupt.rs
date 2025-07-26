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
    #[doc = "3 - Timer/Counter1 Capture Event"]
    TIMER1_CAPT = 3,
    #[doc = "4 - Timer/Counter1 Compare Match A"]
    TIMER1_COMPA = 4,
    #[doc = "5 - Timer/Counter1 Overflow"]
    TIMER1_OVF = 5,
    #[doc = "6 - Timer/Counter0 Overflow"]
    TIMER0_OVF = 6,
    #[doc = "7 - USART, Rx Complete"]
    USART_RX = 7,
    #[doc = "8 - USART Data Register Empty"]
    USART_UDRE = 8,
    #[doc = "9 - USART, Tx Complete"]
    USART_TX = 9,
    #[doc = "10 - Analog Comparator"]
    ANA_COMP = 10,
    #[doc = "11 - Pin Change Interrupt Request B"]
    PCINT_B = 11,
    #[doc = "12 - No Description."]
    TIMER1_COMPB = 12,
    #[doc = "13 - No Description."]
    TIMER0_COMPA = 13,
    #[doc = "14 - No Description."]
    TIMER0_COMPB = 14,
    #[doc = "15 - USI Start Condition"]
    USI_START = 15,
    #[doc = "16 - USI Overflow"]
    USI_OVERFLOW = 16,
    #[doc = "17 - No Description."]
    EEPROM_READY = 17,
    #[doc = "18 - Watchdog Timer Overflow"]
    WDT_OVERFLOW = 18,
    #[doc = "19 - Pin Change Interrupt Request A"]
    PCINT_A = 19,
    #[doc = "20 - Pin Change Interrupt Request D"]
    PCINT_D = 20,
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
            3 => Ok(Interrupt::TIMER1_CAPT),
            4 => Ok(Interrupt::TIMER1_COMPA),
            5 => Ok(Interrupt::TIMER1_OVF),
            6 => Ok(Interrupt::TIMER0_OVF),
            7 => Ok(Interrupt::USART_RX),
            8 => Ok(Interrupt::USART_UDRE),
            9 => Ok(Interrupt::USART_TX),
            10 => Ok(Interrupt::ANA_COMP),
            11 => Ok(Interrupt::PCINT_B),
            12 => Ok(Interrupt::TIMER1_COMPB),
            13 => Ok(Interrupt::TIMER0_COMPA),
            14 => Ok(Interrupt::TIMER0_COMPB),
            15 => Ok(Interrupt::USI_START),
            16 => Ok(Interrupt::USI_OVERFLOW),
            17 => Ok(Interrupt::EEPROM_READY),
            18 => Ok(Interrupt::WDT_OVERFLOW),
            19 => Ok(Interrupt::PCINT_A),
            20 => Ok(Interrupt::PCINT_D),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
