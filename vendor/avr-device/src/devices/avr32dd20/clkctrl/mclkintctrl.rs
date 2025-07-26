#[doc = "Register `MCLKINTCTRL` reader"]
pub type R = crate::R<MCLKINTCTRL_SPEC>;
#[doc = "Register `MCLKINTCTRL` writer"]
pub type W = crate::W<MCLKINTCTRL_SPEC>;
#[doc = "Field `CFD` reader - Clock Failure Detect Interrupt Enable"]
pub type CFD_R = crate::BitReader;
#[doc = "Field `CFD` writer - Clock Failure Detect Interrupt Enable"]
pub type CFD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTYPE` reader - Interrupt type"]
pub type INTTYPE_R = crate::BitReader<INTTYPE_A>;
#[doc = "Interrupt type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTTYPE_A {
    #[doc = "0: Regular Interrupt"]
    INT = 0,
    #[doc = "1: NMI"]
    NMI = 1,
}
impl From<INTTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: INTTYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl INTTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTTYPE_A {
        match self.bits {
            false => INTTYPE_A::INT,
            true => INTTYPE_A::NMI,
        }
    }
    #[doc = "Regular Interrupt"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == INTTYPE_A::INT
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        *self == INTTYPE_A::NMI
    }
}
#[doc = "Field `INTTYPE` writer - Interrupt type"]
pub type INTTYPE_W<'a, REG> = crate::BitWriter<'a, REG, INTTYPE_A>;
impl<'a, REG> INTTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular Interrupt"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(INTTYPE_A::INT)
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut crate::W<REG> {
        self.variant(INTTYPE_A::NMI)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Failure Detect Interrupt Enable"]
    #[inline(always)]
    pub fn cfd(&self) -> CFD_R {
        CFD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt type"]
    #[inline(always)]
    pub fn inttype(&self) -> INTTYPE_R {
        INTTYPE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfd(&mut self) -> CFD_W<MCLKINTCTRL_SPEC> {
        CFD_W::new(self, 0)
    }
    #[doc = "Bit 7 - Interrupt type"]
    #[inline(always)]
    #[must_use]
    pub fn inttype(&mut self) -> INTTYPE_W<MCLKINTCTRL_SPEC> {
        INTTYPE_W::new(self, 7)
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
#[doc = "MCLK Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkintctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkintctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCLKINTCTRL_SPEC;
impl crate::RegisterSpec for MCLKINTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mclkintctrl::R`](R) reader structure"]
impl crate::Readable for MCLKINTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mclkintctrl::W`](W) writer structure"]
impl crate::Writable for MCLKINTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKINTCTRL to value 0"]
impl crate::Resettable for MCLKINTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
