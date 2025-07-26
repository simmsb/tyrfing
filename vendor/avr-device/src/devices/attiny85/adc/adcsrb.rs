#[doc = "Register `ADCSRB` reader"]
pub type R = crate::R<ADCSRB_SPEC>;
#[doc = "Register `ADCSRB` writer"]
pub type W = crate::W<ADCSRB_SPEC>;
#[doc = "Field `ADTS` reader - ADC Auto Trigger Sources"]
pub type ADTS_R = crate::FieldReader<ADTS_A>;
#[doc = "ADC Auto Trigger Sources\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADTS_A {
    #[doc = "0: Free Running mode"]
    FREE = 0,
    #[doc = "1: Analog Comparator"]
    AC = 1,
    #[doc = "2: External Interrupt Request 0"]
    INT0 = 2,
    #[doc = "3: Timer/Counter0 Compare Match A"]
    TC0_CMA = 3,
    #[doc = "4: Timer/Counter0 Overflow"]
    TC0_OVF = 4,
    #[doc = "5: Timer/Counter0 Compare Match B"]
    TC0_CMB = 5,
    #[doc = "6: Pin Change Interrupt Request"]
    PCIR = 6,
}
impl From<ADTS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADTS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADTS_A {
    type Ux = u8;
}
impl ADTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADTS_A> {
        match self.bits {
            0 => Some(ADTS_A::FREE),
            1 => Some(ADTS_A::AC),
            2 => Some(ADTS_A::INT0),
            3 => Some(ADTS_A::TC0_CMA),
            4 => Some(ADTS_A::TC0_OVF),
            5 => Some(ADTS_A::TC0_CMB),
            6 => Some(ADTS_A::PCIR),
            _ => None,
        }
    }
    #[doc = "Free Running mode"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == ADTS_A::FREE
    }
    #[doc = "Analog Comparator"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        *self == ADTS_A::AC
    }
    #[doc = "External Interrupt Request 0"]
    #[inline(always)]
    pub fn is_int0(&self) -> bool {
        *self == ADTS_A::INT0
    }
    #[doc = "Timer/Counter0 Compare Match A"]
    #[inline(always)]
    pub fn is_tc0_cma(&self) -> bool {
        *self == ADTS_A::TC0_CMA
    }
    #[doc = "Timer/Counter0 Overflow"]
    #[inline(always)]
    pub fn is_tc0_ovf(&self) -> bool {
        *self == ADTS_A::TC0_OVF
    }
    #[doc = "Timer/Counter0 Compare Match B"]
    #[inline(always)]
    pub fn is_tc0_cmb(&self) -> bool {
        *self == ADTS_A::TC0_CMB
    }
    #[doc = "Pin Change Interrupt Request"]
    #[inline(always)]
    pub fn is_pcir(&self) -> bool {
        *self == ADTS_A::PCIR
    }
}
#[doc = "Field `ADTS` writer - ADC Auto Trigger Sources"]
pub type ADTS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADTS_A>;
impl<'a, REG> ADTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Free Running mode"]
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(ADTS_A::FREE)
    }
    #[doc = "Analog Comparator"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut crate::W<REG> {
        self.variant(ADTS_A::AC)
    }
    #[doc = "External Interrupt Request 0"]
    #[inline(always)]
    pub fn int0(self) -> &'a mut crate::W<REG> {
        self.variant(ADTS_A::INT0)
    }
    #[doc = "Timer/Counter0 Compare Match A"]
    #[inline(always)]
    pub fn tc0_cma(self) -> &'a mut crate::W<REG> {
        self.variant(ADTS_A::TC0_CMA)
    }
    #[doc = "Timer/Counter0 Overflow"]
    #[inline(always)]
    pub fn tc0_ovf(self) -> &'a mut crate::W<REG> {
        self.variant(ADTS_A::TC0_OVF)
    }
    #[doc = "Timer/Counter0 Compare Match B"]
    #[inline(always)]
    pub fn tc0_cmb(self) -> &'a mut crate::W<REG> {
        self.variant(ADTS_A::TC0_CMB)
    }
    #[doc = "Pin Change Interrupt Request"]
    #[inline(always)]
    pub fn pcir(self) -> &'a mut crate::W<REG> {
        self.variant(ADTS_A::PCIR)
    }
}
#[doc = "Field `IPR` reader - Input Polarity Mode"]
pub type IPR_R = crate::BitReader;
#[doc = "Field `IPR` writer - Input Polarity Mode"]
pub type IPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIN` reader - Bipolar Input Mode"]
pub type BIN_R = crate::BitReader;
#[doc = "Field `BIN` writer - Bipolar Input Mode"]
pub type BIN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - ADC Auto Trigger Sources"]
    #[inline(always)]
    pub fn adts(&self) -> ADTS_R {
        ADTS_R::new(self.bits & 7)
    }
    #[doc = "Bit 5 - Input Polarity Mode"]
    #[inline(always)]
    pub fn ipr(&self) -> IPR_R {
        IPR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Bipolar Input Mode"]
    #[inline(always)]
    pub fn bin(&self) -> BIN_R {
        BIN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC Auto Trigger Sources"]
    #[inline(always)]
    #[must_use]
    pub fn adts(&mut self) -> ADTS_W<ADCSRB_SPEC> {
        ADTS_W::new(self, 0)
    }
    #[doc = "Bit 5 - Input Polarity Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ipr(&mut self) -> IPR_W<ADCSRB_SPEC> {
        IPR_W::new(self, 5)
    }
    #[doc = "Bit 7 - Bipolar Input Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bin(&mut self) -> BIN_W<ADCSRB_SPEC> {
        BIN_W::new(self, 7)
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
#[doc = "ADC Control and Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCSRB_SPEC;
impl crate::RegisterSpec for ADCSRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcsrb::R`](R) reader structure"]
impl crate::Readable for ADCSRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcsrb::W`](W) writer structure"]
impl crate::Writable for ADCSRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCSRB to value 0"]
impl crate::Resettable for ADCSRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
