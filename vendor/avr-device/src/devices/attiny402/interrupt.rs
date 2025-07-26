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
    #[doc = "16 - No Description."]
    AC0_AC = 16,
    #[doc = "17 - No Description."]
    ADC0_RESRDY = 17,
    #[doc = "18 - No Description."]
    ADC0_WCOMP = 18,
    #[doc = "19 - No Description."]
    TWI0_TWIS = 19,
    #[doc = "20 - No Description."]
    TWI0_TWIM = 20,
    #[doc = "21 - No Description."]
    SPI0_INT = 21,
    #[doc = "22 - No Description."]
    USART0_RXC = 22,
    #[doc = "23 - No Description."]
    USART0_DRE = 23,
    #[doc = "24 - No Description."]
    USART0_TXC = 24,
    #[doc = "25 - No Description."]
    NVMCTRL_EE = 25,
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
            6 => Ok(Interrupt::RTC_CNT),
            7 => Ok(Interrupt::RTC_PIT),
            8 => Ok(Interrupt::TCA0_LUNF_OVF),
            9 => Ok(Interrupt::TCA0_HUNF),
            10 => Ok(Interrupt::TCA0_CMP0_LCMP0),
            11 => Ok(Interrupt::TCA0_CMP1_LCMP1),
            12 => Ok(Interrupt::TCA0_CMP2_LCMP2),
            13 => Ok(Interrupt::TCB0_INT),
            16 => Ok(Interrupt::AC0_AC),
            17 => Ok(Interrupt::ADC0_RESRDY),
            18 => Ok(Interrupt::ADC0_WCOMP),
            19 => Ok(Interrupt::TWI0_TWIS),
            20 => Ok(Interrupt::TWI0_TWIM),
            21 => Ok(Interrupt::SPI0_INT),
            22 => Ok(Interrupt::USART0_RXC),
            23 => Ok(Interrupt::USART0_DRE),
            24 => Ok(Interrupt::USART0_TXC),
            25 => Ok(Interrupt::NVMCTRL_EE),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
