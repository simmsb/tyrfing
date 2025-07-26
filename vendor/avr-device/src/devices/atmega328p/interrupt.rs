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
    #[doc = "6 - Watchdog Time-out Interrupt"]
    WDT = 6,
    #[doc = "7 - Timer/Counter2 Compare Match A"]
    TIMER2_COMPA = 7,
    #[doc = "8 - Timer/Counter2 Compare Match B"]
    TIMER2_COMPB = 8,
    #[doc = "9 - Timer/Counter2 Overflow"]
    TIMER2_OVF = 9,
    #[doc = "10 - Timer/Counter1 Capture Event"]
    TIMER1_CAPT = 10,
    #[doc = "11 - Timer/Counter1 Compare Match A"]
    TIMER1_COMPA = 11,
    #[doc = "12 - Timer/Counter1 Compare Match B"]
    TIMER1_COMPB = 12,
    #[doc = "13 - Timer/Counter1 Overflow"]
    TIMER1_OVF = 13,
    #[doc = "14 - TimerCounter0 Compare Match A"]
    TIMER0_COMPA = 14,
    #[doc = "15 - TimerCounter0 Compare Match B"]
    TIMER0_COMPB = 15,
    #[doc = "16 - Timer/Couner0 Overflow"]
    TIMER0_OVF = 16,
    #[doc = "17 - SPI Serial Transfer Complete"]
    SPI_STC = 17,
    #[doc = "18 - USART Rx Complete"]
    USART_RX = 18,
    #[doc = "19 - USART, Data Register Empty"]
    USART_UDRE = 19,
    #[doc = "20 - USART Tx Complete"]
    USART_TX = 20,
    #[doc = "21 - ADC Conversion Complete"]
    ADC = 21,
    #[doc = "22 - EEPROM Ready"]
    EE_READY = 22,
    #[doc = "23 - Analog Comparator"]
    ANALOG_COMP = 23,
    #[doc = "24 - Two-wire Serial Interface"]
    TWI = 24,
    #[doc = "25 - Store Program Memory Read"]
    SPM_READY = 25,
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
            6 => Ok(Interrupt::WDT),
            7 => Ok(Interrupt::TIMER2_COMPA),
            8 => Ok(Interrupt::TIMER2_COMPB),
            9 => Ok(Interrupt::TIMER2_OVF),
            10 => Ok(Interrupt::TIMER1_CAPT),
            11 => Ok(Interrupt::TIMER1_COMPA),
            12 => Ok(Interrupt::TIMER1_COMPB),
            13 => Ok(Interrupt::TIMER1_OVF),
            14 => Ok(Interrupt::TIMER0_COMPA),
            15 => Ok(Interrupt::TIMER0_COMPB),
            16 => Ok(Interrupt::TIMER0_OVF),
            17 => Ok(Interrupt::SPI_STC),
            18 => Ok(Interrupt::USART_RX),
            19 => Ok(Interrupt::USART_UDRE),
            20 => Ok(Interrupt::USART_TX),
            21 => Ok(Interrupt::ADC),
            22 => Ok(Interrupt::EE_READY),
            23 => Ok(Interrupt::ANALOG_COMP),
            24 => Ok(Interrupt::TWI),
            25 => Ok(Interrupt::SPM_READY),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
