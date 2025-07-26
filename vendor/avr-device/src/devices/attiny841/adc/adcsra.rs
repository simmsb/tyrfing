#[doc = "Register `ADCSRA` reader"]
pub type R = crate::R<ADCSRA_SPEC>;
#[doc = "Register `ADCSRA` writer"]
pub type W = crate::W<ADCSRA_SPEC>;
#[doc = "Field `ADPS` reader - ADC Prescaler Select Bits"]
pub type ADPS_R = crate::FieldReader<ADPS_A>;
#[doc = "ADC Prescaler Select Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADPS_A {
    #[doc = "1: Prescaler Value 2"]
    PRESCALER_2 = 1,
    #[doc = "2: Prescaler Value 4"]
    PRESCALER_4 = 2,
    #[doc = "3: Prescaler Value 8"]
    PRESCALER_8 = 3,
    #[doc = "4: Prescaler Value 16"]
    PRESCALER_16 = 4,
    #[doc = "5: Prescaler Value 32"]
    PRESCALER_32 = 5,
    #[doc = "6: Prescaler Value 64"]
    PRESCALER_64 = 6,
    #[doc = "7: Prescaler Value 128"]
    PRESCALER_128 = 7,
}
impl From<ADPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADPS_A {
    type Ux = u8;
}
impl ADPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADPS_A> {
        match self.bits {
            1 => Some(ADPS_A::PRESCALER_2),
            2 => Some(ADPS_A::PRESCALER_4),
            3 => Some(ADPS_A::PRESCALER_8),
            4 => Some(ADPS_A::PRESCALER_16),
            5 => Some(ADPS_A::PRESCALER_32),
            6 => Some(ADPS_A::PRESCALER_64),
            7 => Some(ADPS_A::PRESCALER_128),
            _ => None,
        }
    }
    #[doc = "Prescaler Value 2"]
    #[inline(always)]
    pub fn is_prescaler_2(&self) -> bool {
        *self == ADPS_A::PRESCALER_2
    }
    #[doc = "Prescaler Value 4"]
    #[inline(always)]
    pub fn is_prescaler_4(&self) -> bool {
        *self == ADPS_A::PRESCALER_4
    }
    #[doc = "Prescaler Value 8"]
    #[inline(always)]
    pub fn is_prescaler_8(&self) -> bool {
        *self == ADPS_A::PRESCALER_8
    }
    #[doc = "Prescaler Value 16"]
    #[inline(always)]
    pub fn is_prescaler_16(&self) -> bool {
        *self == ADPS_A::PRESCALER_16
    }
    #[doc = "Prescaler Value 32"]
    #[inline(always)]
    pub fn is_prescaler_32(&self) -> bool {
        *self == ADPS_A::PRESCALER_32
    }
    #[doc = "Prescaler Value 64"]
    #[inline(always)]
    pub fn is_prescaler_64(&self) -> bool {
        *self == ADPS_A::PRESCALER_64
    }
    #[doc = "Prescaler Value 128"]
    #[inline(always)]
    pub fn is_prescaler_128(&self) -> bool {
        *self == ADPS_A::PRESCALER_128
    }
}
#[doc = "Field `ADPS` writer - ADC Prescaler Select Bits"]
pub type ADPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADPS_A>;
impl<'a, REG> ADPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler Value 2"]
    #[inline(always)]
    pub fn prescaler_2(self) -> &'a mut crate::W<REG> {
        self.variant(ADPS_A::PRESCALER_2)
    }
    #[doc = "Prescaler Value 4"]
    #[inline(always)]
    pub fn prescaler_4(self) -> &'a mut crate::W<REG> {
        self.variant(ADPS_A::PRESCALER_4)
    }
    #[doc = "Prescaler Value 8"]
    #[inline(always)]
    pub fn prescaler_8(self) -> &'a mut crate::W<REG> {
        self.variant(ADPS_A::PRESCALER_8)
    }
    #[doc = "Prescaler Value 16"]
    #[inline(always)]
    pub fn prescaler_16(self) -> &'a mut crate::W<REG> {
        self.variant(ADPS_A::PRESCALER_16)
    }
    #[doc = "Prescaler Value 32"]
    #[inline(always)]
    pub fn prescaler_32(self) -> &'a mut crate::W<REG> {
        self.variant(ADPS_A::PRESCALER_32)
    }
    #[doc = "Prescaler Value 64"]
    #[inline(always)]
    pub fn prescaler_64(self) -> &'a mut crate::W<REG> {
        self.variant(ADPS_A::PRESCALER_64)
    }
    #[doc = "Prescaler Value 128"]
    #[inline(always)]
    pub fn prescaler_128(self) -> &'a mut crate::W<REG> {
        self.variant(ADPS_A::PRESCALER_128)
    }
}
#[doc = "Field `ADIE` reader - ADC Interrupt Enable"]
pub type ADIE_R = crate::BitReader;
#[doc = "Field `ADIE` writer - ADC Interrupt Enable"]
pub type ADIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIF` reader - ADC Interrupt Flag"]
pub type ADIF_R = crate::BitReader;
#[doc = "Field `ADIF` writer - ADC Interrupt Flag"]
pub type ADIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADATE` reader - ADC Auto Trigger Enable"]
pub type ADATE_R = crate::BitReader;
#[doc = "Field `ADATE` writer - ADC Auto Trigger Enable"]
pub type ADATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSC` reader - ADC Start Conversion"]
pub type ADSC_R = crate::BitReader;
#[doc = "Field `ADSC` writer - ADC Start Conversion"]
pub type ADSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADEN` reader - ADC Enable"]
pub type ADEN_R = crate::BitReader;
#[doc = "Field `ADEN` writer - ADC Enable"]
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - ADC Prescaler Select Bits"]
    #[inline(always)]
    pub fn adps(&self) -> ADPS_R {
        ADPS_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - ADC Interrupt Enable"]
    #[inline(always)]
    pub fn adie(&self) -> ADIE_R {
        ADIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Interrupt Flag"]
    #[inline(always)]
    pub fn adif(&self) -> ADIF_R {
        ADIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC Auto Trigger Enable"]
    #[inline(always)]
    pub fn adate(&self) -> ADATE_R {
        ADATE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC Start Conversion"]
    #[inline(always)]
    pub fn adsc(&self) -> ADSC_R {
        ADSC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC Enable"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC Prescaler Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn adps(&mut self) -> ADPS_W<ADCSRA_SPEC> {
        ADPS_W::new(self, 0)
    }
    #[doc = "Bit 3 - ADC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adie(&mut self) -> ADIE_W<ADCSRA_SPEC> {
        ADIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adif(&mut self) -> ADIF_W<ADCSRA_SPEC> {
        ADIF_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC Auto Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adate(&mut self) -> ADATE_W<ADCSRA_SPEC> {
        ADATE_W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC Start Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn adsc(&mut self) -> ADSC_W<ADCSRA_SPEC> {
        ADSC_W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<ADCSRA_SPEC> {
        ADEN_W::new(self, 7)
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
#[doc = "The ADC Control and Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCSRA_SPEC;
impl crate::RegisterSpec for ADCSRA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcsra::R`](R) reader structure"]
impl crate::Readable for ADCSRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcsra::W`](W) writer structure"]
impl crate::Writable for ADCSRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCSRA to value 0"]
impl crate::Resettable for ADCSRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
