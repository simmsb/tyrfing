#[doc = "Register `OSCCFG` reader"]
pub type R = crate::R<OSCCFG_SPEC>;
#[doc = "Register `OSCCFG` writer"]
pub type W = crate::W<OSCCFG_SPEC>;
#[doc = "Field `CLKSEL` reader - Frequency Select"]
pub type CLKSEL_R = crate::FieldReader<CLKSEL_A>;
#[doc = "Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: 1-32MHz internal oscillator"]
    OSCHF = 0,
    #[doc = "1: 32.768kHz internal oscillator"]
    OSC32K = 1,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKSEL_A {
    type Ux = u8;
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::OSCHF),
            1 => Some(CLKSEL_A::OSC32K),
            _ => None,
        }
    }
    #[doc = "1-32MHz internal oscillator"]
    #[inline(always)]
    pub fn is_oschf(&self) -> bool {
        *self == CLKSEL_A::OSCHF
    }
    #[doc = "32.768kHz internal oscillator"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == CLKSEL_A::OSC32K
    }
}
#[doc = "Field `CLKSEL` writer - Frequency Select"]
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CLKSEL_A>;
impl<'a, REG> CLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1-32MHz internal oscillator"]
    #[inline(always)]
    pub fn oschf(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::OSCHF)
    }
    #[doc = "32.768kHz internal oscillator"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::OSC32K)
    }
}
impl R {
    #[doc = "Bits 0:2 - Frequency Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<OSCCFG_SPEC> {
        CLKSEL_W::new(self, 0)
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
#[doc = "Oscillator Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCCFG_SPEC;
impl crate::RegisterSpec for OSCCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osccfg::R`](R) reader structure"]
impl crate::Readable for OSCCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osccfg::W`](W) writer structure"]
impl crate::Writable for OSCCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCCFG to value 0"]
impl crate::Resettable for OSCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
