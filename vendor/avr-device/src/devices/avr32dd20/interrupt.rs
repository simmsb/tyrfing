#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "1 - No Description."]
    CRCSCAN_NMI = 1,
    #[doc = "2 - No Description."]
    BOD_VLM = 2,
    #[doc = "3 - No Description."]
    CLKCTRL_CFD = 3,
    #[doc = "4 - No Description."]
    MVIO_MVIO = 4,
    #[doc = "5 - No Description."]
    RTC_CNT = 5,
    #[doc = "6 - No Description."]
    RTC_PIT = 6,
    #[doc = "7 - No Description."]
    CCL_CCL = 7,
    #[doc = "8 - No Description."]
    PORTA_PORT = 8,
    #[doc = "9 - No Description."]
    TCA0_LUNF_OVF = 9,
    #[doc = "10 - No Description."]
    TCA0_HUNF = 10,
    #[doc = "11 - No Description."]
    TCA0_CMP0_LCMP0 = 11,
    #[doc = "12 - No Description."]
    TCA0_CMP1_LCMP1 = 12,
    #[doc = "13 - No Description."]
    TCA0_CMP2_LCMP2 = 13,
    #[doc = "14 - No Description."]
    TCB0_INT = 14,
    #[doc = "15 - No Description."]
    TCB1_INT = 15,
    #[doc = "16 - No Description."]
    TCD0_OVF = 16,
    #[doc = "17 - No Description."]
    TCD0_TRIG = 17,
    #[doc = "18 - No Description."]
    TWI0_TWIS = 18,
    #[doc = "19 - No Description."]
    TWI0_TWIM = 19,
    #[doc = "20 - No Description."]
    SPI0_INT = 20,
    #[doc = "21 - No Description."]
    USART0_RXC = 21,
    #[doc = "22 - No Description."]
    USART0_DRE = 22,
    #[doc = "23 - No Description."]
    USART0_TXC = 23,
    #[doc = "24 - No Description."]
    PORTD_PORT = 24,
    #[doc = "25 - No Description."]
    AC0_AC = 25,
    #[doc = "26 - No Description."]
    ADC0_RESRDY = 26,
    #[doc = "27 - No Description."]
    ADC0_WCMP = 27,
    #[doc = "28 - No Description."]
    ZCD3_ZCD = 28,
    #[doc = "29 - No Description."]
    PORTC_PORT = 29,
    #[doc = "31 - No Description."]
    USART1_RXC = 31,
    #[doc = "32 - No Description."]
    USART1_DRE = 32,
    #[doc = "33 - No Description."]
    USART1_TXC = 33,
    #[doc = "34 - No Description."]
    PORTF_PORT = 34,
    #[doc = "35 - No Description."]
    NVMCTRL_EE = 35,
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
            3 => Ok(Interrupt::CLKCTRL_CFD),
            4 => Ok(Interrupt::MVIO_MVIO),
            5 => Ok(Interrupt::RTC_CNT),
            6 => Ok(Interrupt::RTC_PIT),
            7 => Ok(Interrupt::CCL_CCL),
            8 => Ok(Interrupt::PORTA_PORT),
            9 => Ok(Interrupt::TCA0_LUNF_OVF),
            10 => Ok(Interrupt::TCA0_HUNF),
            11 => Ok(Interrupt::TCA0_CMP0_LCMP0),
            12 => Ok(Interrupt::TCA0_CMP1_LCMP1),
            13 => Ok(Interrupt::TCA0_CMP2_LCMP2),
            14 => Ok(Interrupt::TCB0_INT),
            15 => Ok(Interrupt::TCB1_INT),
            16 => Ok(Interrupt::TCD0_OVF),
            17 => Ok(Interrupt::TCD0_TRIG),
            18 => Ok(Interrupt::TWI0_TWIS),
            19 => Ok(Interrupt::TWI0_TWIM),
            20 => Ok(Interrupt::SPI0_INT),
            21 => Ok(Interrupt::USART0_RXC),
            22 => Ok(Interrupt::USART0_DRE),
            23 => Ok(Interrupt::USART0_TXC),
            24 => Ok(Interrupt::PORTD_PORT),
            25 => Ok(Interrupt::AC0_AC),
            26 => Ok(Interrupt::ADC0_RESRDY),
            27 => Ok(Interrupt::ADC0_WCMP),
            28 => Ok(Interrupt::ZCD3_ZCD),
            29 => Ok(Interrupt::PORTC_PORT),
            31 => Ok(Interrupt::USART1_RXC),
            32 => Ok(Interrupt::USART1_DRE),
            33 => Ok(Interrupt::USART1_TXC),
            34 => Ok(Interrupt::PORTF_PORT),
            35 => Ok(Interrupt::NVMCTRL_EE),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
