#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - External Pin, Power-on Reset, Brown-out Reset and Watchdog Reset"]
    RESET = 0,
    #[doc = "1 - External Interrupt Request 0"]
    INT0 = 1,
    #[doc = "2 - Pin Change Interrupt Request 0"]
    PCINT0 = 2,
    #[doc = "3 - Pin Change Interrupt Request 1"]
    PCINT1 = 3,
    #[doc = "4 - Watchdog Time-out Interrupt"]
    WDT = 4,
    #[doc = "5 - Timer/Counter1 Capture Event"]
    TIMER1_CAPT = 5,
    #[doc = "6 - Timer/Counter1 Compare Match A"]
    TIMER1_COMPA = 6,
    #[doc = "7 - Timer/Counter1 Compare Match B"]
    TIMER1_COMPB = 7,
    #[doc = "8 - Timer/Counter1 Overflow"]
    TIMER1_OVF = 8,
    #[doc = "9 - TimerCounter0 Compare Match A"]
    TIMER0_COMPA = 9,
    #[doc = "10 - TimerCounter0 Compare Match B"]
    TIMER0_COMPB = 10,
    #[doc = "11 - Timer/Couner0 Overflow"]
    TIMER0_OVF = 11,
    #[doc = "12 - Analog Comparator 0"]
    ANA_COMP0 = 12,
    #[doc = "13 - ADC Conversion Complete"]
    ADC = 13,
    #[doc = "14 - EEPROM Ready"]
    EE_RDY = 14,
    #[doc = "15 - Analog Comparator 1"]
    ANA_COMP1 = 15,
    #[doc = "16 - Timer/Counter2 Capture Event"]
    TIMER2_CAPT = 16,
    #[doc = "17 - Timer/Counter2 Compare Match A"]
    TIMER2_COMPA = 17,
    #[doc = "18 - Timer/Counter2 Compare Match B"]
    TIMER2_COMPB = 18,
    #[doc = "19 - Timer/Counter2 Overflow"]
    TIMER2_OVF = 19,
    #[doc = "20 - Serial Peripheral Interface"]
    SPI = 20,
    #[doc = "21 - USART0, Start"]
    USART0_START = 21,
    #[doc = "22 - USART0, Rx Complete"]
    USART0_RX = 22,
    #[doc = "23 - USART0 Data Register Empty"]
    USART0_UDRE = 23,
    #[doc = "24 - USART0, Tx Complete"]
    USART0_TX = 24,
    #[doc = "25 - USART1, Start"]
    USART1_START = 25,
    #[doc = "26 - USART1, Rx Complete"]
    USART1_RX = 26,
    #[doc = "27 - USART1 Data Register Empty"]
    USART1_UDRE = 27,
    #[doc = "28 - USART1, Tx Complete"]
    USART1_TX = 28,
    #[doc = "29 - Two-wire Serial Interface"]
    TWI_SLAVE = 29,
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
            3 => Ok(Interrupt::PCINT1),
            4 => Ok(Interrupt::WDT),
            5 => Ok(Interrupt::TIMER1_CAPT),
            6 => Ok(Interrupt::TIMER1_COMPA),
            7 => Ok(Interrupt::TIMER1_COMPB),
            8 => Ok(Interrupt::TIMER1_OVF),
            9 => Ok(Interrupt::TIMER0_COMPA),
            10 => Ok(Interrupt::TIMER0_COMPB),
            11 => Ok(Interrupt::TIMER0_OVF),
            12 => Ok(Interrupt::ANA_COMP0),
            13 => Ok(Interrupt::ADC),
            14 => Ok(Interrupt::EE_RDY),
            15 => Ok(Interrupt::ANA_COMP1),
            16 => Ok(Interrupt::TIMER2_CAPT),
            17 => Ok(Interrupt::TIMER2_COMPA),
            18 => Ok(Interrupt::TIMER2_COMPB),
            19 => Ok(Interrupt::TIMER2_OVF),
            20 => Ok(Interrupt::SPI),
            21 => Ok(Interrupt::USART0_START),
            22 => Ok(Interrupt::USART0_RX),
            23 => Ok(Interrupt::USART0_UDRE),
            24 => Ok(Interrupt::USART0_TX),
            25 => Ok(Interrupt::USART1_START),
            26 => Ok(Interrupt::USART1_RX),
            27 => Ok(Interrupt::USART1_UDRE),
            28 => Ok(Interrupt::USART1_TX),
            29 => Ok(Interrupt::TWI_SLAVE),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
