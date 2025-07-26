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
    #[doc = "9 - Pin Change Interrupt Request 0"]
    PCINT0 = 9,
    #[doc = "10 - USB General Interrupt Request"]
    USB_GEN = 10,
    #[doc = "11 - USB Endpoint/Pipe Interrupt Communication Request"]
    USB_COM = 11,
    #[doc = "12 - Watchdog Time-out Interrupt"]
    WDT = 12,
    #[doc = "13 - Timer/Counter2 Compare Match A"]
    TIMER2_COMPA = 13,
    #[doc = "14 - Timer/Counter2 Compare Match B"]
    TIMER2_COMPB = 14,
    #[doc = "15 - Timer/Counter2 Overflow"]
    TIMER2_OVF = 15,
    #[doc = "16 - Timer/Counter1 Capture Event"]
    TIMER1_CAPT = 16,
    #[doc = "17 - Timer/Counter1 Compare Match A"]
    TIMER1_COMPA = 17,
    #[doc = "18 - Timer/Counter1 Compare Match B"]
    TIMER1_COMPB = 18,
    #[doc = "19 - Timer/Counter1 Compare Match C"]
    TIMER1_COMPC = 19,
    #[doc = "20 - Timer/Counter1 Overflow"]
    TIMER1_OVF = 20,
    #[doc = "21 - Timer/Counter0 Compare Match A"]
    TIMER0_COMPA = 21,
    #[doc = "22 - Timer/Counter0 Compare Match B"]
    TIMER0_COMPB = 22,
    #[doc = "23 - Timer/Counter0 Overflow"]
    TIMER0_OVF = 23,
    #[doc = "24 - SPI Serial Transfer Complete"]
    SPI_STC = 24,
    #[doc = "25 - USART1, Rx Complete"]
    USART1_RX = 25,
    #[doc = "26 - USART1 Data register Empty"]
    USART1_UDRE = 26,
    #[doc = "27 - USART1, Tx Complete"]
    USART1_TX = 27,
    #[doc = "28 - Analog Comparator"]
    ANALOG_COMP = 28,
    #[doc = "29 - ADC Conversion Complete"]
    ADC = 29,
    #[doc = "30 - EEPROM Ready"]
    EE_READY = 30,
    #[doc = "31 - Timer/Counter3 Capture Event"]
    TIMER3_CAPT = 31,
    #[doc = "32 - Timer/Counter3 Compare Match A"]
    TIMER3_COMPA = 32,
    #[doc = "33 - Timer/Counter3 Compare Match B"]
    TIMER3_COMPB = 33,
    #[doc = "34 - Timer/Counter3 Compare Match C"]
    TIMER3_COMPC = 34,
    #[doc = "35 - Timer/Counter3 Overflow"]
    TIMER3_OVF = 35,
    #[doc = "36 - 2-wire Serial Interface"]
    TWI = 36,
    #[doc = "37 - Store Program Memory Read"]
    SPM_READY = 37,
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
            9 => Ok(Interrupt::PCINT0),
            10 => Ok(Interrupt::USB_GEN),
            11 => Ok(Interrupt::USB_COM),
            12 => Ok(Interrupt::WDT),
            13 => Ok(Interrupt::TIMER2_COMPA),
            14 => Ok(Interrupt::TIMER2_COMPB),
            15 => Ok(Interrupt::TIMER2_OVF),
            16 => Ok(Interrupt::TIMER1_CAPT),
            17 => Ok(Interrupt::TIMER1_COMPA),
            18 => Ok(Interrupt::TIMER1_COMPB),
            19 => Ok(Interrupt::TIMER1_COMPC),
            20 => Ok(Interrupt::TIMER1_OVF),
            21 => Ok(Interrupt::TIMER0_COMPA),
            22 => Ok(Interrupt::TIMER0_COMPB),
            23 => Ok(Interrupt::TIMER0_OVF),
            24 => Ok(Interrupt::SPI_STC),
            25 => Ok(Interrupt::USART1_RX),
            26 => Ok(Interrupt::USART1_UDRE),
            27 => Ok(Interrupt::USART1_TX),
            28 => Ok(Interrupt::ANALOG_COMP),
            29 => Ok(Interrupt::ADC),
            30 => Ok(Interrupt::EE_READY),
            31 => Ok(Interrupt::TIMER3_CAPT),
            32 => Ok(Interrupt::TIMER3_COMPA),
            33 => Ok(Interrupt::TIMER3_COMPB),
            34 => Ok(Interrupt::TIMER3_COMPC),
            35 => Ok(Interrupt::TIMER3_OVF),
            36 => Ok(Interrupt::TWI),
            37 => Ok(Interrupt::SPM_READY),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
