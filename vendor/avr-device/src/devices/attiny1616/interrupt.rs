#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "1 - No Description."]
    CRCSCAN_NMI = 1,
    #[doc = "2 - No Description."]
    BOD_VLM = 2,
    #[doc = "3 - No Description."]
    PORTA_PORT = 3,
    #[doc = "4 - No Description."]
    PORTB_PORT = 4,
    #[doc = "5 - No Description."]
    PORTC_PORT = 5,
    #[doc = "6 - No Description."]
    RTC_CNT = 6,
    #[doc = "7 - No Description."]
    RTC_PIT = 7,
    #[doc = "8 - No Description."]
    TCA0_LUNF_OVF = 8,
    #[doc = "9 - No Description."]
    TCA0_HUNF = 9,
    #[doc = "10 - No Description."]
    TCA0_CMP0_LCMP0 = 10,
    #[doc = "11 - No Description."]
    TCA0_CMP1_LCMP1 = 11,
    #[doc = "12 - No Description."]
    TCA0_CMP2_LCMP2 = 12,
    #[doc = "13 - No Description."]
    TCB0_INT = 13,
    #[doc = "14 - No Description."]
    TCB1_INT = 14,
    #[doc = "15 - No Description."]
    TCD0_OVF = 15,
    #[doc = "16 - No Description."]
    TCD0_TRIG = 16,
    #[doc = "17 - No Description."]
    AC0_AC = 17,
    #[doc = "18 - No Description."]
    AC1_AC = 18,
    #[doc = "19 - No Description."]
    AC2_AC = 19,
    #[doc = "20 - No Description."]
    ADC0_RESRDY = 20,
    #[doc = "21 - No Description."]
    ADC0_WCOMP = 21,
    #[doc = "22 - No Description."]
    ADC1_RESRDY = 22,
    #[doc = "23 - No Description."]
    ADC1_WCOMP = 23,
    #[doc = "24 - No Description."]
    TWI0_TWIS = 24,
    #[doc = "25 - No Description."]
    TWI0_TWIM = 25,
    #[doc = "26 - No Description."]
    SPI0_INT = 26,
    #[doc = "27 - No Description."]
    USART0_RXC = 27,
    #[doc = "28 - No Description."]
    USART0_DRE = 28,
    #[doc = "29 - No Description."]
    USART0_TXC = 29,
    #[doc = "30 - No Description."]
    NVMCTRL_EE = 30,
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
            3 => Ok(Interrupt::PORTA_PORT),
            4 => Ok(Interrupt::PORTB_PORT),
            5 => Ok(Interrupt::PORTC_PORT),
            6 => Ok(Interrupt::RTC_CNT),
            7 => Ok(Interrupt::RTC_PIT),
            8 => Ok(Interrupt::TCA0_LUNF_OVF),
            9 => Ok(Interrupt::TCA0_HUNF),
            10 => Ok(Interrupt::TCA0_CMP0_LCMP0),
            11 => Ok(Interrupt::TCA0_CMP1_LCMP1),
            12 => Ok(Interrupt::TCA0_CMP2_LCMP2),
            13 => Ok(Interrupt::TCB0_INT),
            14 => Ok(Interrupt::TCB1_INT),
            15 => Ok(Interrupt::TCD0_OVF),
            16 => Ok(Interrupt::TCD0_TRIG),
            17 => Ok(Interrupt::AC0_AC),
            18 => Ok(Interrupt::AC1_AC),
            19 => Ok(Interrupt::AC2_AC),
            20 => Ok(Interrupt::ADC0_RESRDY),
            21 => Ok(Interrupt::ADC0_WCOMP),
            22 => Ok(Interrupt::ADC1_RESRDY),
            23 => Ok(Interrupt::ADC1_WCOMP),
            24 => Ok(Interrupt::TWI0_TWIS),
            25 => Ok(Interrupt::TWI0_TWIM),
            26 => Ok(Interrupt::SPI0_INT),
            27 => Ok(Interrupt::USART0_RXC),
            28 => Ok(Interrupt::USART0_DRE),
            29 => Ok(Interrupt::USART0_TXC),
            30 => Ok(Interrupt::NVMCTRL_EE),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
