#[doc = "Register `MCUCR` reader"]
pub type R = crate::R<MCUCR_SPEC>;
#[doc = "Register `MCUCR` writer"]
pub type W = crate::W<MCUCR_SPEC>;
#[doc = "Field `SM` reader - Sleep Mode Select Bits"]
pub type SM_R = crate::FieldReader<SM_A>;
#[doc = "Sleep Mode Select Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM_A {
    #[doc = "0: Idle"]
    IDLE = 0,
    #[doc = "1: ADC Noise Reduction (If Available)"]
    ADC = 1,
    #[doc = "2: Power Down"]
    PDOWN = 2,
    #[doc = "3: Standby"]
    STDBY = 3,
}
impl From<SM_A> for u8 {
    #[inline(always)]
    fn from(variant: SM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SM_A {
    type Ux = u8;
}
impl SM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SM_A {
        match self.bits {
            0 => SM_A::IDLE,
            1 => SM_A::ADC,
            2 => SM_A::PDOWN,
            3 => SM_A::STDBY,
            _ => unreachable!(),
        }
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SM_A::IDLE
    }
    #[doc = "ADC Noise Reduction (If Available)"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == SM_A::ADC
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub fn is_pdown(&self) -> bool {
        *self == SM_A::PDOWN
    }
    #[doc = "Standby"]
    #[inline(always)]
    pub fn is_stdby(&self) -> bool {
        *self == SM_A::STDBY
    }
}
#[doc = "Field `SM` writer - Sleep Mode Select Bits"]
pub type SM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SM_A>;
impl<'a, REG> SM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::IDLE)
    }
    #[doc = "ADC Noise Reduction (If Available)"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::ADC)
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub fn pdown(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::PDOWN)
    }
    #[doc = "Standby"]
    #[inline(always)]
    pub fn stdby(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::STDBY)
    }
}
#[doc = "Field `SE` reader - Sleep Enable"]
pub type SE_R = crate::BitReader;
#[doc = "Field `SE` writer - Sleep Enable"]
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUD` reader - Pull-up Disable"]
pub type PUD_R = crate::BitReader;
#[doc = "Field `PUD` writer - Pull-up Disable"]
pub type PUD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 3:4 - Sleep Mode Select Bits"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - Sleep Enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pull-up Disable"]
    #[inline(always)]
    pub fn pud(&self) -> PUD_R {
        PUD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:4 - Sleep Mode Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<MCUCR_SPEC> {
        SM_W::new(self, 3)
    }
    #[doc = "Bit 5 - Sleep Enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<MCUCR_SPEC> {
        SE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pull-up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pud(&mut self) -> PUD_W<MCUCR_SPEC> {
        PUD_W::new(self, 6)
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
#[doc = "MCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCUCR_SPEC;
impl crate::RegisterSpec for MCUCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mcucr::R`](R) reader structure"]
impl crate::Readable for MCUCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcucr::W`](W) writer structure"]
impl crate::Writable for MCUCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCR to value 0"]
impl crate::Resettable for MCUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
