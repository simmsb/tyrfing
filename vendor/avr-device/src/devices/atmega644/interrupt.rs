#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - External Pin,Power-on Reset,Brown-out Reset,Watchdog Reset,and JTAG AVR Reset. See Datasheet."]
    RESET = 0,
    #[doc = "1 - External Interrupt Request 0"]
    INT0 = 1,
    #[doc = "2 - External Interrupt Request 1"]
    INT1 = 2,
    #[doc = "3 - External Interrupt Request 2"]
    INT2 = 3,
    #[doc = "4 - Pin Change Interrupt Request 0"]
    PCINT0 = 4,
    #[doc = "5 - Pin Change Interrupt Request 1"]
    PCINT1 = 5,
    #[doc = "6 - Pin Change Interrupt Request 2"]
    PCINT2 = 6,
    #[doc = "7 - Pin Change Interrupt Request 3"]
    PCINT3 = 7,
    #[doc = "8 - Watchdog Time-out Interrupt"]
    WDT = 8,
    #[doc = "9 - Timer/Counter2 Compare Match A"]
    TIMER2_COMPA = 9,
    #[doc = "10 - Timer/Counter2 Compare Match B"]
    TIMER2_COMPB = 10,
    #[doc = "11 - Timer/Counter2 Overflow"]
    TIMER2_OVF = 11,
    #[doc = "12 - Timer/Counter1 Capture Event"]
    TIMER1_CAPT = 12,
    #[doc = "13 - Timer/Counter1 Compare Match A"]
    TIMER1_COMPA = 13,
    #[doc = "14 - Timer/Counter1 Compare Match B"]
    TIMER1_COMPB = 14,
    #[doc = "15 - Timer/Counter1 Overflow"]
    TIMER1_OVF = 15,
    #[doc = "16 - Timer/Counter0 Compare Match A"]
    TIMER0_COMPA = 16,
    #[doc = "17 - Timer/Counter0 Compare Match B"]
    TIMER0_COMPB = 17,
    #[doc = "18 - Timer/Counter0 Overflow"]
    TIMER0_OVF = 18,
    #[doc = "19 - SPI Serial Transfer Complete"]
    SPI_STC = 19,
    #[doc = "20 - USART0, Rx Complete"]
    USART0_RX = 20,
    #[doc = "21 - USART0 Data register Empty"]
    USART0_UDRE = 21,
    #[doc = "22 - USART0, Tx Complete"]
    USART0_TX = 22,
    #[doc = "23 - Analog Comparator"]
    ANALOG_COMP = 23,
    #[doc = "24 - ADC Conversion Complete"]
    ADC = 24,
    #[doc = "25 - EEPROM Ready"]
    EE_READY = 25,
    #[doc = "26 - 2-wire Serial Interface"]
    TWI = 26,
    #[doc = "27 - Store Program Memory Read"]
    SPM_READY = 27,
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
            4 => Ok(Interrupt::PCINT0),
            5 => Ok(Interrupt::PCINT1),
            6 => Ok(Interrupt::PCINT2),
            7 => Ok(Interrupt::PCINT3),
            8 => Ok(Interrupt::WDT),
            9 => Ok(Interrupt::TIMER2_COMPA),
            10 => Ok(Interrupt::TIMER2_COMPB),
            11 => Ok(Interrupt::TIMER2_OVF),
            12 => Ok(Interrupt::TIMER1_CAPT),
            13 => Ok(Interrupt::TIMER1_COMPA),
            14 => Ok(Interrupt::TIMER1_COMPB),
            15 => Ok(Interrupt::TIMER1_OVF),
            16 => Ok(Interrupt::TIMER0_COMPA),
            17 => Ok(Interrupt::TIMER0_COMPB),
            18 => Ok(Interrupt::TIMER0_OVF),
            19 => Ok(Interrupt::SPI_STC),
            20 => Ok(Interrupt::USART0_RX),
            21 => Ok(Interrupt::USART0_UDRE),
            22 => Ok(Interrupt::USART0_TX),
            23 => Ok(Interrupt::ANALOG_COMP),
            24 => Ok(Interrupt::ADC),
            25 => Ok(Interrupt::EE_READY),
            26 => Ok(Interrupt::TWI),
            27 => Ok(Interrupt::SPM_READY),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
