#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "1 - No Description."]
    CRCSCAN_NMI = 1,
    #[doc = "2 - No Description."]
    BOD_VLM = 2,
    #[doc = "3 - No Description."]
    RTC_CNT = 3,
    #[doc = "4 - No Description."]
    RTC_PIT = 4,
    #[doc = "5 - No Description."]
    CCL_CCL = 5,
    #[doc = "6 - No Description."]
    PORTA_PORT = 6,
    #[doc = "7 - No Description."]
    TCA0_LUNF_OVF = 7,
    #[doc = "8 - No Description."]
    TCA0_HUNF = 8,
    #[doc = "9 - No Description."]
    TCA0_CMP0_LCMP0 = 9,
    #[doc = "10 - No Description."]
    TCA0_CMP1_LCMP1 = 10,
    #[doc = "11 - No Description."]
    TCA0_CMP2_LCMP2 = 11,
    #[doc = "12 - No Description."]
    TCB0_INT = 12,
    #[doc = "13 - No Description."]
    TCB1_INT = 13,
    #[doc = "14 - No Description."]
    TWI0_TWIS = 14,
    #[doc = "15 - No Description."]
    TWI0_TWIM = 15,
    #[doc = "16 - No Description."]
    SPI0_INT = 16,
    #[doc = "17 - No Description."]
    USART0_RXC = 17,
    #[doc = "18 - No Description."]
    USART0_DRE = 18,
    #[doc = "19 - No Description."]
    USART0_TXC = 19,
    #[doc = "20 - No Description."]
    PORTD_PORT = 20,
    #[doc = "21 - No Description."]
    AC0_AC = 21,
    #[doc = "22 - No Description."]
    ADC0_RESRDY = 22,
    #[doc = "23 - No Description."]
    ADC0_WCOMP = 23,
    #[doc = "24 - No Description."]
    PORTC_PORT = 24,
    #[doc = "25 - No Description."]
    TCB2_INT = 25,
    #[doc = "26 - No Description."]
    USART1_RXC = 26,
    #[doc = "27 - No Description."]
    USART1_DRE = 27,
    #[doc = "28 - No Description."]
    USART1_TXC = 28,
    #[doc = "29 - No Description."]
    PORTF_PORT = 29,
    #[doc = "30 - No Description."]
    NVMCTRL_EE = 30,
    #[doc = "31 - No Description."]
    USART2_RXC = 31,
    #[doc = "32 - No Description."]
    USART2_DRE = 32,
    #[doc = "33 - No Description."]
    USART2_TXC = 33,
    #[doc = "34 - No Description."]
    PORTB_PORT = 34,
    #[doc = "35 - No Description."]
    PORTE_PORT = 35,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            1 => Ok(Interrupt::CRCSCAN_NMI),
            2 => Ok(Interrupt::BOD_VLM),
            3 => Ok(Interrupt::RTC_CNT),
            4 => Ok(Interrupt::RTC_PIT),
            5 => Ok(Interrupt::CCL_CCL),
            6 => Ok(Interrupt::PORTA_PORT),
            7 => Ok(Interrupt::TCA0_LUNF_OVF),
            8 => Ok(Interrupt::TCA0_HUNF),
            9 => Ok(Interrupt::TCA0_CMP0_LCMP0),
            10 => Ok(Interrupt::TCA0_CMP1_LCMP1),
            11 => Ok(Interrupt::TCA0_CMP2_LCMP2),
            12 => Ok(Interrupt::TCB0_INT),
            13 => Ok(Interrupt::TCB1_INT),
            14 => Ok(Interrupt::TWI0_TWIS),
            15 => Ok(Interrupt::TWI0_TWIM),
            16 => Ok(Interrupt::SPI0_INT),
            17 => Ok(Interrupt::USART0_RXC),
            18 => Ok(Interrupt::USART0_DRE),
            19 => Ok(Interrupt::USART0_TXC),
            20 => Ok(Interrupt::PORTD_PORT),
            21 => Ok(Interrupt::AC0_AC),
            22 => Ok(Interrupt::ADC0_RESRDY),
            23 => Ok(Interrupt::ADC0_WCOMP),
            24 => Ok(Interrupt::PORTC_PORT),
            25 => Ok(Interrupt::TCB2_INT),
            26 => Ok(Interrupt::USART1_RXC),
            27 => Ok(Interrupt::USART1_DRE),
            28 => Ok(Interrupt::USART1_TXC),
            29 => Ok(Interrupt::PORTF_PORT),
            30 => Ok(Interrupt::NVMCTRL_EE),
            31 => Ok(Interrupt::USART2_RXC),
            32 => Ok(Interrupt::USART2_DRE),
            33 => Ok(Interrupt::USART2_TXC),
            34 => Ok(Interrupt::PORTB_PORT),
            35 => Ok(Interrupt::PORTE_PORT),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
