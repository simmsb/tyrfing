#[doc = "Register `CHANNEL3` reader"]
pub type R = crate::R<CHANNEL3_SPEC>;
#[doc = "Register `CHANNEL3` writer"]
pub type W = crate::W<CHANNEL3_SPEC>;
#[doc = "Field `CHANNEL3` reader - Channel 3 generator select"]
pub type CHANNEL3_R = crate::FieldReader<CHANNEL3_A>;
#[doc = "Channel 3 generator select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHANNEL3_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: UPDI SYNCH character"]
    UPDI_SYNCH = 1,
    #[doc = "5: MVIO VDDIO2 OK"]
    MVIO = 5,
    #[doc = "6: Real Time Counter overflow"]
    RTC_OVF = 6,
    #[doc = "7: Real Time Counter compare"]
    RTC_CMP = 7,
    #[doc = "8: Periodic Interrupt Timer output 0"]
    RTC_PIT_DIV512 = 8,
    #[doc = "9: Periodic Interrupt Timer output 1"]
    RTC_PIT_DIV256 = 9,
    #[doc = "10: Periodic Interrupt Timer output 2"]
    RTC_PIT_DIV128 = 10,
    #[doc = "11: Periodic Interrupt Timer output 3"]
    RTC_PIT_DIV64 = 11,
    #[doc = "16: Configurable Custom Logic LUT0"]
    CCL_LUT0 = 16,
    #[doc = "17: Configurable Custom Logic LUT1"]
    CCL_LUT1 = 17,
    #[doc = "18: Configurable Custom Logic LUT2"]
    CCL_LUT2 = 18,
    #[doc = "19: Configurable Custom Logic LUT3"]
    CCL_LUT3 = 19,
    #[doc = "32: Analog Comparator 0 out"]
    AC0_OUT = 32,
    #[doc = "36: ADC 0 Result Ready"]
    ADC0_RESRDY = 36,
    #[doc = "48: Zero Cross Detect 3 out"]
    ZCD3 = 48,
    #[doc = "65: Port C Pin 1"]
    PORTC_PIN1 = 65,
    #[doc = "66: Port C Pin 2"]
    PORTC_PIN2 = 66,
    #[doc = "67: Port C Pin 3"]
    PORTC_PIN3 = 67,
    #[doc = "76: Port D Pin 4"]
    PORTD_PIN4 = 76,
    #[doc = "77: Port D Pin 5"]
    PORTD_PIN5 = 77,
    #[doc = "78: Port D Pin 6"]
    PORTD_PIN6 = 78,
    #[doc = "79: Port D Pin 7"]
    PORTD_PIN7 = 79,
    #[doc = "96: USART 0 XCK"]
    USART0_XCK = 96,
    #[doc = "97: USART 1 XCK"]
    USART1_XCK = 97,
    #[doc = "104: SPI 0 SCK"]
    SPI0_SCK = 104,
    #[doc = "128: Timer/Counter A0 overflow / low byte timer underflow"]
    TCA0_OVF_LUNF = 128,
    #[doc = "129: Timer/Counter A0 high byte timer underflow"]
    TCA0_HUNF = 129,
    #[doc = "132: Timer/Counter A0 compare 0 / low byte timer compare 0"]
    TCA0_CMP0_LCMP0 = 132,
    #[doc = "133: Timer/Counter A0 compare 1 / low byte timer compare 1"]
    TCA0_CMP1_LCMP1 = 133,
    #[doc = "134: Timer/Counter A0 compare 2 / low byte timer compare 2"]
    TCA0_CMP2_LCMP2 = 134,
    #[doc = "160: Timer/Counter B0 capture"]
    TCB0_CAPT = 160,
    #[doc = "161: Timer/Counter B0 overflow"]
    TCB0_OVF = 161,
    #[doc = "162: Timer/Counter B1 capture"]
    TCB1_CAPT = 162,
    #[doc = "163: Timer/Counter B1 overflow"]
    TCB1_OVF = 163,
    #[doc = "164: Timer/Counter B2 capture"]
    TCB2_CAPT = 164,
    #[doc = "165: Timer/Counter B2 overflow"]
    TCB2_OVF = 165,
    #[doc = "176: Timer/Counter D0 event 0"]
    TCD0_CMPBCLR = 176,
    #[doc = "177: Timer/Counter D0 event 1"]
    TCD0_CMPASET = 177,
    #[doc = "178: Timer/Counter D0 event 2"]
    TCD0_CMPBSET = 178,
    #[doc = "179: Timer/Counter D0 event 3"]
    TCD0_PROGEV = 179,
}
impl From<CHANNEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNEL3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHANNEL3_A {
    type Ux = u8;
}
impl CHANNEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CHANNEL3_A> {
        match self.bits {
            0 => Some(CHANNEL3_A::OFF),
            1 => Some(CHANNEL3_A::UPDI_SYNCH),
            5 => Some(CHANNEL3_A::MVIO),
            6 => Some(CHANNEL3_A::RTC_OVF),
            7 => Some(CHANNEL3_A::RTC_CMP),
            8 => Some(CHANNEL3_A::RTC_PIT_DIV512),
            9 => Some(CHANNEL3_A::RTC_PIT_DIV256),
            10 => Some(CHANNEL3_A::RTC_PIT_DIV128),
            11 => Some(CHANNEL3_A::RTC_PIT_DIV64),
            16 => Some(CHANNEL3_A::CCL_LUT0),
            17 => Some(CHANNEL3_A::CCL_LUT1),
            18 => Some(CHANNEL3_A::CCL_LUT2),
            19 => Some(CHANNEL3_A::CCL_LUT3),
            32 => Some(CHANNEL3_A::AC0_OUT),
            36 => Some(CHANNEL3_A::ADC0_RESRDY),
            48 => Some(CHANNEL3_A::ZCD3),
            65 => Some(CHANNEL3_A::PORTC_PIN1),
            66 => Some(CHANNEL3_A::PORTC_PIN2),
            67 => Some(CHANNEL3_A::PORTC_PIN3),
            76 => Some(CHANNEL3_A::PORTD_PIN4),
            77 => Some(CHANNEL3_A::PORTD_PIN5),
            78 => Some(CHANNEL3_A::PORTD_PIN6),
            79 => Some(CHANNEL3_A::PORTD_PIN7),
            96 => Some(CHANNEL3_A::USART0_XCK),
            97 => Some(CHANNEL3_A::USART1_XCK),
            104 => Some(CHANNEL3_A::SPI0_SCK),
            128 => Some(CHANNEL3_A::TCA0_OVF_LUNF),
            129 => Some(CHANNEL3_A::TCA0_HUNF),
            132 => Some(CHANNEL3_A::TCA0_CMP0_LCMP0),
            133 => Some(CHANNEL3_A::TCA0_CMP1_LCMP1),
            134 => Some(CHANNEL3_A::TCA0_CMP2_LCMP2),
            160 => Some(CHANNEL3_A::TCB0_CAPT),
            161 => Some(CHANNEL3_A::TCB0_OVF),
            162 => Some(CHANNEL3_A::TCB1_CAPT),
            163 => Some(CHANNEL3_A::TCB1_OVF),
            164 => Some(CHANNEL3_A::TCB2_CAPT),
            165 => Some(CHANNEL3_A::TCB2_OVF),
            176 => Some(CHANNEL3_A::TCD0_CMPBCLR),
            177 => Some(CHANNEL3_A::TCD0_CMPASET),
            178 => Some(CHANNEL3_A::TCD0_CMPBSET),
            179 => Some(CHANNEL3_A::TCD0_PROGEV),
            _ => None,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CHANNEL3_A::OFF
    }
    #[doc = "UPDI SYNCH character"]
    #[inline(always)]
    pub fn is_updi_synch(&self) -> bool {
        *self == CHANNEL3_A::UPDI_SYNCH
    }
    #[doc = "MVIO VDDIO2 OK"]
    #[inline(always)]
    pub fn is_mvio(&self) -> bool {
        *self == CHANNEL3_A::MVIO
    }
    #[doc = "Real Time Counter overflow"]
    #[inline(always)]
    pub fn is_rtc_ovf(&self) -> bool {
        *self == CHANNEL3_A::RTC_OVF
    }
    #[doc = "Real Time Counter compare"]
    #[inline(always)]
    pub fn is_rtc_cmp(&self) -> bool {
        *self == CHANNEL3_A::RTC_CMP
    }
    #[doc = "Periodic Interrupt Timer output 0"]
    #[inline(always)]
    pub fn is_rtc_pit_div512(&self) -> bool {
        *self == CHANNEL3_A::RTC_PIT_DIV512
    }
    #[doc = "Periodic Interrupt Timer output 1"]
    #[inline(always)]
    pub fn is_rtc_pit_div256(&self) -> bool {
        *self == CHANNEL3_A::RTC_PIT_DIV256
    }
    #[doc = "Periodic Interrupt Timer output 2"]
    #[inline(always)]
    pub fn is_rtc_pit_div128(&self) -> bool {
        *self == CHANNEL3_A::RTC_PIT_DIV128
    }
    #[doc = "Periodic Interrupt Timer output 3"]
    #[inline(always)]
    pub fn is_rtc_pit_div64(&self) -> bool {
        *self == CHANNEL3_A::RTC_PIT_DIV64
    }
    #[doc = "Configurable Custom Logic LUT0"]
    #[inline(always)]
    pub fn is_ccl_lut0(&self) -> bool {
        *self == CHANNEL3_A::CCL_LUT0
    }
    #[doc = "Configurable Custom Logic LUT1"]
    #[inline(always)]
    pub fn is_ccl_lut1(&self) -> bool {
        *self == CHANNEL3_A::CCL_LUT1
    }
    #[doc = "Configurable Custom Logic LUT2"]
    #[inline(always)]
    pub fn is_ccl_lut2(&self) -> bool {
        *self == CHANNEL3_A::CCL_LUT2
    }
    #[doc = "Configurable Custom Logic LUT3"]
    #[inline(always)]
    pub fn is_ccl_lut3(&self) -> bool {
        *self == CHANNEL3_A::CCL_LUT3
    }
    #[doc = "Analog Comparator 0 out"]
    #[inline(always)]
    pub fn is_ac0_out(&self) -> bool {
        *self == CHANNEL3_A::AC0_OUT
    }
    #[doc = "ADC 0 Result Ready"]
    #[inline(always)]
    pub fn is_adc0_resrdy(&self) -> bool {
        *self == CHANNEL3_A::ADC0_RESRDY
    }
    #[doc = "Zero Cross Detect 3 out"]
    #[inline(always)]
    pub fn is_zcd3(&self) -> bool {
        *self == CHANNEL3_A::ZCD3
    }
    #[doc = "Port C Pin 1"]
    #[inline(always)]
    pub fn is_portc_pin1(&self) -> bool {
        *self == CHANNEL3_A::PORTC_PIN1
    }
    #[doc = "Port C Pin 2"]
    #[inline(always)]
    pub fn is_portc_pin2(&self) -> bool {
        *self == CHANNEL3_A::PORTC_PIN2
    }
    #[doc = "Port C Pin 3"]
    #[inline(always)]
    pub fn is_portc_pin3(&self) -> bool {
        *self == CHANNEL3_A::PORTC_PIN3
    }
    #[doc = "Port D Pin 4"]
    #[inline(always)]
    pub fn is_portd_pin4(&self) -> bool {
        *self == CHANNEL3_A::PORTD_PIN4
    }
    #[doc = "Port D Pin 5"]
    #[inline(always)]
    pub fn is_portd_pin5(&self) -> bool {
        *self == CHANNEL3_A::PORTD_PIN5
    }
    #[doc = "Port D Pin 6"]
    #[inline(always)]
    pub fn is_portd_pin6(&self) -> bool {
        *self == CHANNEL3_A::PORTD_PIN6
    }
    #[doc = "Port D Pin 7"]
    #[inline(always)]
    pub fn is_portd_pin7(&self) -> bool {
        *self == CHANNEL3_A::PORTD_PIN7
    }
    #[doc = "USART 0 XCK"]
    #[inline(always)]
    pub fn is_usart0_xck(&self) -> bool {
        *self == CHANNEL3_A::USART0_XCK
    }
    #[doc = "USART 1 XCK"]
    #[inline(always)]
    pub fn is_usart1_xck(&self) -> bool {
        *self == CHANNEL3_A::USART1_XCK
    }
    #[doc = "SPI 0 SCK"]
    #[inline(always)]
    pub fn is_spi0_sck(&self) -> bool {
        *self == CHANNEL3_A::SPI0_SCK
    }
    #[doc = "Timer/Counter A0 overflow / low byte timer underflow"]
    #[inline(always)]
    pub fn is_tca0_ovf_lunf(&self) -> bool {
        *self == CHANNEL3_A::TCA0_OVF_LUNF
    }
    #[doc = "Timer/Counter A0 high byte timer underflow"]
    #[inline(always)]
    pub fn is_tca0_hunf(&self) -> bool {
        *self == CHANNEL3_A::TCA0_HUNF
    }
    #[doc = "Timer/Counter A0 compare 0 / low byte timer compare 0"]
    #[inline(always)]
    pub fn is_tca0_cmp0_lcmp0(&self) -> bool {
        *self == CHANNEL3_A::TCA0_CMP0_LCMP0
    }
    #[doc = "Timer/Counter A0 compare 1 / low byte timer compare 1"]
    #[inline(always)]
    pub fn is_tca0_cmp1_lcmp1(&self) -> bool {
        *self == CHANNEL3_A::TCA0_CMP1_LCMP1
    }
    #[doc = "Timer/Counter A0 compare 2 / low byte timer compare 2"]
    #[inline(always)]
    pub fn is_tca0_cmp2_lcmp2(&self) -> bool {
        *self == CHANNEL3_A::TCA0_CMP2_LCMP2
    }
    #[doc = "Timer/Counter B0 capture"]
    #[inline(always)]
    pub fn is_tcb0_capt(&self) -> bool {
        *self == CHANNEL3_A::TCB0_CAPT
    }
    #[doc = "Timer/Counter B0 overflow"]
    #[inline(always)]
    pub fn is_tcb0_ovf(&self) -> bool {
        *self == CHANNEL3_A::TCB0_OVF
    }
    #[doc = "Timer/Counter B1 capture"]
    #[inline(always)]
    pub fn is_tcb1_capt(&self) -> bool {
        *self == CHANNEL3_A::TCB1_CAPT
    }
    #[doc = "Timer/Counter B1 overflow"]
    #[inline(always)]
    pub fn is_tcb1_ovf(&self) -> bool {
        *self == CHANNEL3_A::TCB1_OVF
    }
    #[doc = "Timer/Counter B2 capture"]
    #[inline(always)]
    pub fn is_tcb2_capt(&self) -> bool {
        *self == CHANNEL3_A::TCB2_CAPT
    }
    #[doc = "Timer/Counter B2 overflow"]
    #[inline(always)]
    pub fn is_tcb2_ovf(&self) -> bool {
        *self == CHANNEL3_A::TCB2_OVF
    }
    #[doc = "Timer/Counter D0 event 0"]
    #[inline(always)]
    pub fn is_tcd0_cmpbclr(&self) -> bool {
        *self == CHANNEL3_A::TCD0_CMPBCLR
    }
    #[doc = "Timer/Counter D0 event 1"]
    #[inline(always)]
    pub fn is_tcd0_cmpaset(&self) -> bool {
        *self == CHANNEL3_A::TCD0_CMPASET
    }
    #[doc = "Timer/Counter D0 event 2"]
    #[inline(always)]
    pub fn is_tcd0_cmpbset(&self) -> bool {
        *self == CHANNEL3_A::TCD0_CMPBSET
    }
    #[doc = "Timer/Counter D0 event 3"]
    #[inline(always)]
    pub fn is_tcd0_progev(&self) -> bool {
        *self == CHANNEL3_A::TCD0_PROGEV
    }
}
#[doc = "Field `CHANNEL3` writer - Channel 3 generator select"]
pub type CHANNEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CHANNEL3_A>;
impl<'a, REG> CHANNEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::OFF)
    }
    #[doc = "UPDI SYNCH character"]
    #[inline(always)]
    pub fn updi_synch(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::UPDI_SYNCH)
    }
    #[doc = "MVIO VDDIO2 OK"]
    #[inline(always)]
    pub fn mvio(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::MVIO)
    }
    #[doc = "Real Time Counter overflow"]
    #[inline(always)]
    pub fn rtc_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::RTC_OVF)
    }
    #[doc = "Real Time Counter compare"]
    #[inline(always)]
    pub fn rtc_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::RTC_CMP)
    }
    #[doc = "Periodic Interrupt Timer output 0"]
    #[inline(always)]
    pub fn rtc_pit_div512(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::RTC_PIT_DIV512)
    }
    #[doc = "Periodic Interrupt Timer output 1"]
    #[inline(always)]
    pub fn rtc_pit_div256(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::RTC_PIT_DIV256)
    }
    #[doc = "Periodic Interrupt Timer output 2"]
    #[inline(always)]
    pub fn rtc_pit_div128(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::RTC_PIT_DIV128)
    }
    #[doc = "Periodic Interrupt Timer output 3"]
    #[inline(always)]
    pub fn rtc_pit_div64(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::RTC_PIT_DIV64)
    }
    #[doc = "Configurable Custom Logic LUT0"]
    #[inline(always)]
    pub fn ccl_lut0(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::CCL_LUT0)
    }
    #[doc = "Configurable Custom Logic LUT1"]
    #[inline(always)]
    pub fn ccl_lut1(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::CCL_LUT1)
    }
    #[doc = "Configurable Custom Logic LUT2"]
    #[inline(always)]
    pub fn ccl_lut2(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::CCL_LUT2)
    }
    #[doc = "Configurable Custom Logic LUT3"]
    #[inline(always)]
    pub fn ccl_lut3(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::CCL_LUT3)
    }
    #[doc = "Analog Comparator 0 out"]
    #[inline(always)]
    pub fn ac0_out(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::AC0_OUT)
    }
    #[doc = "ADC 0 Result Ready"]
    #[inline(always)]
    pub fn adc0_resrdy(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::ADC0_RESRDY)
    }
    #[doc = "Zero Cross Detect 3 out"]
    #[inline(always)]
    pub fn zcd3(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::ZCD3)
    }
    #[doc = "Port C Pin 1"]
    #[inline(always)]
    pub fn portc_pin1(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::PORTC_PIN1)
    }
    #[doc = "Port C Pin 2"]
    #[inline(always)]
    pub fn portc_pin2(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::PORTC_PIN2)
    }
    #[doc = "Port C Pin 3"]
    #[inline(always)]
    pub fn portc_pin3(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::PORTC_PIN3)
    }
    #[doc = "Port D Pin 4"]
    #[inline(always)]
    pub fn portd_pin4(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::PORTD_PIN4)
    }
    #[doc = "Port D Pin 5"]
    #[inline(always)]
    pub fn portd_pin5(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::PORTD_PIN5)
    }
    #[doc = "Port D Pin 6"]
    #[inline(always)]
    pub fn portd_pin6(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::PORTD_PIN6)
    }
    #[doc = "Port D Pin 7"]
    #[inline(always)]
    pub fn portd_pin7(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::PORTD_PIN7)
    }
    #[doc = "USART 0 XCK"]
    #[inline(always)]
    pub fn usart0_xck(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::USART0_XCK)
    }
    #[doc = "USART 1 XCK"]
    #[inline(always)]
    pub fn usart1_xck(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::USART1_XCK)
    }
    #[doc = "SPI 0 SCK"]
    #[inline(always)]
    pub fn spi0_sck(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::SPI0_SCK)
    }
    #[doc = "Timer/Counter A0 overflow / low byte timer underflow"]
    #[inline(always)]
    pub fn tca0_ovf_lunf(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCA0_OVF_LUNF)
    }
    #[doc = "Timer/Counter A0 high byte timer underflow"]
    #[inline(always)]
    pub fn tca0_hunf(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCA0_HUNF)
    }
    #[doc = "Timer/Counter A0 compare 0 / low byte timer compare 0"]
    #[inline(always)]
    pub fn tca0_cmp0_lcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCA0_CMP0_LCMP0)
    }
    #[doc = "Timer/Counter A0 compare 1 / low byte timer compare 1"]
    #[inline(always)]
    pub fn tca0_cmp1_lcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCA0_CMP1_LCMP1)
    }
    #[doc = "Timer/Counter A0 compare 2 / low byte timer compare 2"]
    #[inline(always)]
    pub fn tca0_cmp2_lcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCA0_CMP2_LCMP2)
    }
    #[doc = "Timer/Counter B0 capture"]
    #[inline(always)]
    pub fn tcb0_capt(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCB0_CAPT)
    }
    #[doc = "Timer/Counter B0 overflow"]
    #[inline(always)]
    pub fn tcb0_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCB0_OVF)
    }
    #[doc = "Timer/Counter B1 capture"]
    #[inline(always)]
    pub fn tcb1_capt(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCB1_CAPT)
    }
    #[doc = "Timer/Counter B1 overflow"]
    #[inline(always)]
    pub fn tcb1_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCB1_OVF)
    }
    #[doc = "Timer/Counter B2 capture"]
    #[inline(always)]
    pub fn tcb2_capt(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCB2_CAPT)
    }
    #[doc = "Timer/Counter B2 overflow"]
    #[inline(always)]
    pub fn tcb2_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCB2_OVF)
    }
    #[doc = "Timer/Counter D0 event 0"]
    #[inline(always)]
    pub fn tcd0_cmpbclr(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCD0_CMPBCLR)
    }
    #[doc = "Timer/Counter D0 event 1"]
    #[inline(always)]
    pub fn tcd0_cmpaset(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCD0_CMPASET)
    }
    #[doc = "Timer/Counter D0 event 2"]
    #[inline(always)]
    pub fn tcd0_cmpbset(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCD0_CMPBSET)
    }
    #[doc = "Timer/Counter D0 event 3"]
    #[inline(always)]
    pub fn tcd0_progev(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL3_A::TCD0_PROGEV)
    }
}
impl R {
    #[doc = "Bits 0:7 - Channel 3 generator select"]
    #[inline(always)]
    pub fn channel3(&self) -> CHANNEL3_R {
        CHANNEL3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel 3 generator select"]
    #[inline(always)]
    #[must_use]
    pub fn channel3(&mut self) -> CHANNEL3_W<CHANNEL3_SPEC> {
        CHANNEL3_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Multiplexer Channel 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHANNEL3_SPEC;
impl crate::RegisterSpec for CHANNEL3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`channel3::R`](R) reader structure"]
impl crate::Readable for CHANNEL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`channel3::W`](W) writer structure"]
impl crate::Writable for CHANNEL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANNEL3 to value 0"]
impl crate::Resettable for CHANNEL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
