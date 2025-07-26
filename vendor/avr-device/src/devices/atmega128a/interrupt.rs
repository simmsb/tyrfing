#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - External Pin, Power-on Reset, Brown-out Reset, Watchdog Reset and JTAG AVR Reset"]
    RESET = 0,
    #[doc = "1 - External Interrupt Request 0"]
    INT0 = 1,
    #[doc = "2 - External Interrupt Request 1"]
    INT1 = 2,
    #[doc = "3 - External Interrupt Request 2"]
    INT2 = 3,
    #[doc = "4 - External Interrupt Request 3"]
    INT3 = 4,
    #[doc = "5 - External Interrupt Request 4"]
    INT4 = 5,
    #[doc = "6 - External Interrupt Request 5"]
    INT5 = 6,
    #[doc = "7 - External Interrupt Request 6"]
    INT6 = 7,
    #[doc = "8 - External Interrupt Request 7"]
    INT7 = 8,
    #[doc = "9 - Timer/Counter2 Compare Match"]
    TIMER2_COMP = 9,
    #[doc = "10 - Timer/Counter2 Overflow"]
    TIMER2_OVF = 10,
    #[doc = "11 - Timer/Counter1 Capture Event"]
    TIMER1_CAPT = 11,
    #[doc = "12 - Timer/Counter1 Compare Match A"]
    TIMER1_COMPA = 12,
    #[doc = "13 - Timer/Counter Compare Match B"]
    TIMER1_COMPB = 13,
    #[doc = "14 - Timer/Counter1 Overflow"]
    TIMER1_OVF = 14,
    #[doc = "15 - Timer/Counter0 Compare Match"]
    TIMER0_COMP = 15,
    #[doc = "16 - Timer/Counter0 Overflow"]
    TIMER0_OVF = 16,
    #[doc = "17 - SPI Serial Transfer Complete"]
    SPI_STC = 17,
    #[doc = "18 - USART0, Rx Complete"]
    USART0_RX = 18,
    #[doc = "19 - USART0 Data Register Empty"]
    USART0_UDRE = 19,
    #[doc = "20 - USART0, Tx Complete"]
    USART0_TX = 20,
    #[doc = "21 - ADC Conversion Complete"]
    ADC = 21,
    #[doc = "22 - EEPROM Ready"]
    EE_READY = 22,
    #[doc = "23 - Analog Comparator"]
    ANALOG_COMP = 23,
    #[doc = "24 - Timer/Counter1 Compare Match C"]
    TIMER1_COMPC = 24,
    #[doc = "25 - Timer/Counter3 Capture Event"]
    TIMER3_CAPT = 25,
    #[doc = "26 - Timer/Counter3 Compare Match A"]
    TIMER3_COMPA = 26,
    #[doc = "27 - Timer/Counter3 Compare Match B"]
    TIMER3_COMPB = 27,
    #[doc = "28 - Timer/Counter3 Compare Match C"]
    TIMER3_COMPC = 28,
    #[doc = "29 - Timer/Counter3 Overflow"]
    TIMER3_OVF = 29,
    #[doc = "30 - USART1, Rx Complete"]
    USART1_RX = 30,
    #[doc = "31 - USART1, Data Register Empty"]
    USART1_UDRE = 31,
    #[doc = "32 - USART1, Tx Complete"]
    USART1_TX = 32,
    #[doc = "33 - 2-wire Serial Interface"]
    TWI = 33,
    #[doc = "34 - Store Program Memory Read"]
    SPM_READY = 34,
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
            3 => Ok(Interrupt::INT2),
            4 => Ok(Interrupt::INT3),
            5 => Ok(Interrupt::INT4),
            6 => Ok(Interrupt::INT5),
            7 => Ok(Interrupt::INT6),
            8 => Ok(Interrupt::INT7),
            9 => Ok(Interrupt::TIMER2_COMP),
            10 => Ok(Interrupt::TIMER2_OVF),
            11 => Ok(Interrupt::TIMER1_CAPT),
            12 => Ok(Interrupt::TIMER1_COMPA),
            13 => Ok(Interrupt::TIMER1_COMPB),
            14 => Ok(Interrupt::TIMER1_OVF),
            15 => Ok(Interrupt::TIMER0_COMP),
            16 => Ok(Interrupt::TIMER0_OVF),
            17 => Ok(Interrupt::SPI_STC),
            18 => Ok(Interrupt::USART0_RX),
            19 => Ok(Interrupt::USART0_UDRE),
            20 => Ok(Interrupt::USART0_TX),
            21 => Ok(Interrupt::ADC),
            22 => Ok(Interrupt::EE_READY),
            23 => Ok(Interrupt::ANALOG_COMP),
            24 => Ok(Interrupt::TIMER1_COMPC),
            25 => Ok(Interrupt::TIMER3_CAPT),
            26 => Ok(Interrupt::TIMER3_COMPA),
            27 => Ok(Interrupt::TIMER3_COMPB),
            28 => Ok(Interrupt::TIMER3_COMPC),
            29 => Ok(Interrupt::TIMER3_OVF),
            30 => Ok(Interrupt::USART1_RX),
            31 => Ok(Interrupt::USART1_UDRE),
            32 => Ok(Interrupt::USART1_TX),
            33 => Ok(Interrupt::TWI),
            34 => Ok(Interrupt::SPM_READY),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
