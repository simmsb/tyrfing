#[doc = "Register `ADCSRC` reader"]
pub type R = crate::R<ADCSRC_SPEC>;
#[doc = "Register `ADCSRC` writer"]
pub type W = crate::W<ADCSRC_SPEC>;
#[doc = "Field `ADSUT` reader - ADC Start-up Time"]
pub type ADSUT_R = crate::FieldReader<ADSUT_A>;
#[doc = "ADC Start-up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADSUT_A {
    #[doc = "0: 3 ADC clock cycles"]
    _3_ADC_CLOCK_CYCLES = 0,
    #[doc = "1: 7 ADC clock cycles"]
    _7_ADC_CLOCK_CYCLES = 1,
    #[doc = "2: 11 ADC clock cycles"]
    _11_ADC_CLOCK_CYCLES = 2,
    #[doc = "3: 15 ADC clock cycles"]
    _15_ADC_CLOCK_CYCLES = 3,
}
impl From<ADSUT_A> for u8 {
    #[inline(always)]
    fn from(variant: ADSUT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADSUT_A {
    type Ux = u8;
}
impl ADSUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADSUT_A> {
        match self.bits {
            0 => Some(ADSUT_A::_3_ADC_CLOCK_CYCLES),
            1 => Some(ADSUT_A::_7_ADC_CLOCK_CYCLES),
            2 => Some(ADSUT_A::_11_ADC_CLOCK_CYCLES),
            3 => Some(ADSUT_A::_15_ADC_CLOCK_CYCLES),
            _ => None,
        }
    }
    #[doc = "3 ADC clock cycles"]
    #[inline(always)]
    pub fn is_3_adc_clock_cycles(&self) -> bool {
        *self == ADSUT_A::_3_ADC_CLOCK_CYCLES
    }
    #[doc = "7 ADC clock cycles"]
    #[inline(always)]
    pub fn is_7_adc_clock_cycles(&self) -> bool {
        *self == ADSUT_A::_7_ADC_CLOCK_CYCLES
    }
    #[doc = "11 ADC clock cycles"]
    #[inline(always)]
    pub fn is_11_adc_clock_cycles(&self) -> bool {
        *self == ADSUT_A::_11_ADC_CLOCK_CYCLES
    }
    #[doc = "15 ADC clock cycles"]
    #[inline(always)]
    pub fn is_15_adc_clock_cycles(&self) -> bool {
        *self == ADSUT_A::_15_ADC_CLOCK_CYCLES
    }
}
#[doc = "Field `ADSUT` writer - ADC Start-up Time"]
pub type ADSUT_W<'a, REG> = crate::FieldWriter<'a, REG, 5, ADSUT_A>;
impl<'a, REG> ADSUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3 ADC clock cycles"]
    #[inline(always)]
    pub fn _3_adc_clock_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(ADSUT_A::_3_ADC_CLOCK_CYCLES)
    }
    #[doc = "7 ADC clock cycles"]
    #[inline(always)]
    pub fn _7_adc_clock_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(ADSUT_A::_7_ADC_CLOCK_CYCLES)
    }
    #[doc = "11 ADC clock cycles"]
    #[inline(always)]
    pub fn _11_adc_clock_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(ADSUT_A::_11_ADC_CLOCK_CYCLES)
    }
    #[doc = "15 ADC clock cycles"]
    #[inline(always)]
    pub fn _15_adc_clock_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(ADSUT_A::_15_ADC_CLOCK_CYCLES)
    }
}
#[doc = "Field `Res0` reader - Reserved"]
pub type RES0_R = crate::BitReader;
#[doc = "Field `Res0` writer - Reserved"]
pub type RES0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADTHT` reader - ADC Track-and-Hold Time"]
pub type ADTHT_R = crate::FieldReader<ADTHT_A>;
#[doc = "ADC Track-and-Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADTHT_A {
    #[doc = "0: Single ended: 1, differential 3 ADC clock cycles"]
    SINGLE_ENDED_1_DIFFERENTIAL_3_ADC_CLOCK_CYCLES = 0,
    #[doc = "1: Single ended: 2, differential 5 ADC clock cycles"]
    SINGLE_ENDED_2_DIFFERENTIAL_5_ADC_CLOCK_CYCLES = 1,
    #[doc = "2: Single ended: 3, differential 7 ADC clock cycles"]
    SINGLE_ENDED_3_DIFFERENTIAL_7_ADC_CLOCK_CYCLES = 2,
    #[doc = "3: Single ended: 4, differential 9 ADC clock cycles"]
    SINGLE_ENDED_4_DIFFERENTIAL_9_ADC_CLOCK_CYCLES = 3,
}
impl From<ADTHT_A> for u8 {
    #[inline(always)]
    fn from(variant: ADTHT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADTHT_A {
    type Ux = u8;
}
impl ADTHT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADTHT_A {
        match self.bits {
            0 => ADTHT_A::SINGLE_ENDED_1_DIFFERENTIAL_3_ADC_CLOCK_CYCLES,
            1 => ADTHT_A::SINGLE_ENDED_2_DIFFERENTIAL_5_ADC_CLOCK_CYCLES,
            2 => ADTHT_A::SINGLE_ENDED_3_DIFFERENTIAL_7_ADC_CLOCK_CYCLES,
            3 => ADTHT_A::SINGLE_ENDED_4_DIFFERENTIAL_9_ADC_CLOCK_CYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Single ended: 1, differential 3 ADC clock cycles"]
    #[inline(always)]
    pub fn is_single_ended_1_differential_3_adc_clock_cycles(&self) -> bool {
        *self == ADTHT_A::SINGLE_ENDED_1_DIFFERENTIAL_3_ADC_CLOCK_CYCLES
    }
    #[doc = "Single ended: 2, differential 5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_single_ended_2_differential_5_adc_clock_cycles(&self) -> bool {
        *self == ADTHT_A::SINGLE_ENDED_2_DIFFERENTIAL_5_ADC_CLOCK_CYCLES
    }
    #[doc = "Single ended: 3, differential 7 ADC clock cycles"]
    #[inline(always)]
    pub fn is_single_ended_3_differential_7_adc_clock_cycles(&self) -> bool {
        *self == ADTHT_A::SINGLE_ENDED_3_DIFFERENTIAL_7_ADC_CLOCK_CYCLES
    }
    #[doc = "Single ended: 4, differential 9 ADC clock cycles"]
    #[inline(always)]
    pub fn is_single_ended_4_differential_9_adc_clock_cycles(&self) -> bool {
        *self == ADTHT_A::SINGLE_ENDED_4_DIFFERENTIAL_9_ADC_CLOCK_CYCLES
    }
}
#[doc = "Field `ADTHT` writer - ADC Track-and-Hold Time"]
pub type ADTHT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ADTHT_A>;
impl<'a, REG> ADTHT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single ended: 1, differential 3 ADC clock cycles"]
    #[inline(always)]
    pub fn single_ended_1_differential_3_adc_clock_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(ADTHT_A::SINGLE_ENDED_1_DIFFERENTIAL_3_ADC_CLOCK_CYCLES)
    }
    #[doc = "Single ended: 2, differential 5 ADC clock cycles"]
    #[inline(always)]
    pub fn single_ended_2_differential_5_adc_clock_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(ADTHT_A::SINGLE_ENDED_2_DIFFERENTIAL_5_ADC_CLOCK_CYCLES)
    }
    #[doc = "Single ended: 3, differential 7 ADC clock cycles"]
    #[inline(always)]
    pub fn single_ended_3_differential_7_adc_clock_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(ADTHT_A::SINGLE_ENDED_3_DIFFERENTIAL_7_ADC_CLOCK_CYCLES)
    }
    #[doc = "Single ended: 4, differential 9 ADC clock cycles"]
    #[inline(always)]
    pub fn single_ended_4_differential_9_adc_clock_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(ADTHT_A::SINGLE_ENDED_4_DIFFERENTIAL_9_ADC_CLOCK_CYCLES)
    }
}
impl R {
    #[doc = "Bits 0:4 - ADC Start-up Time"]
    #[inline(always)]
    pub fn adsut(&self) -> ADSUT_R {
        ADSUT_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn res0(&self) -> RES0_R {
        RES0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - ADC Track-and-Hold Time"]
    #[inline(always)]
    pub fn adtht(&self) -> ADTHT_R {
        ADTHT_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC Start-up Time"]
    #[inline(always)]
    #[must_use]
    pub fn adsut(&mut self) -> ADSUT_W<ADCSRC_SPEC> {
        ADSUT_W::new(self, 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res0(&mut self) -> RES0_W<ADCSRC_SPEC> {
        RES0_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - ADC Track-and-Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn adtht(&mut self) -> ADTHT_W<ADCSRC_SPEC> {
        ADTHT_W::new(self, 6)
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
#[doc = "The ADC Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCSRC_SPEC;
impl crate::RegisterSpec for ADCSRC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcsrc::R`](R) reader structure"]
impl crate::Readable for ADCSRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcsrc::W`](W) writer structure"]
impl crate::Writable for ADCSRC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCSRC to value 0"]
impl crate::Resettable for ADCSRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
