#[doc = "Register `CLKSEL` reader"]
pub type R = crate::R<CLKSEL_SPEC>;
#[doc = "Register `CLKSEL` writer"]
pub type W = crate::W<CLKSEL_SPEC>;
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type CLKSEL_R = crate::FieldReader<CLKSEL_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: 32.768 kHz from OSC32K"]
    OSC32K = 0,
    #[doc = "1: 1.024 kHz from OSC32K"]
    OSC1K = 1,
    #[doc = "2: 32.768 kHz from XOSC32K"]
    XTAL32K = 2,
    #[doc = "3: External Clock"]
    EXTCLK = 3,
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
    pub const fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::OSC32K,
            1 => CLKSEL_A::OSC1K,
            2 => CLKSEL_A::XTAL32K,
            3 => CLKSEL_A::EXTCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "32.768 kHz from OSC32K"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == CLKSEL_A::OSC32K
    }
    #[doc = "1.024 kHz from OSC32K"]
    #[inline(always)]
    pub fn is_osc1k(&self) -> bool {
        *self == CLKSEL_A::OSC1K
    }
    #[doc = "32.768 kHz from XOSC32K"]
    #[inline(always)]
    pub fn is_xtal32k(&self) -> bool {
        *self == CLKSEL_A::XTAL32K
    }
    #[doc = "External Clock"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == CLKSEL_A::EXTCLK
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type CLKSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CLKSEL_A>;
impl<'a, REG> CLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32.768 kHz from OSC32K"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::OSC32K)
    }
    #[doc = "1.024 kHz from OSC32K"]
    #[inline(always)]
    pub fn osc1k(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::OSC1K)
    }
    #[doc = "32.768 kHz from XOSC32K"]
    #[inline(always)]
    pub fn xtal32k(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::XTAL32K)
    }
    #[doc = "External Clock"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::EXTCLK)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<CLKSEL_SPEC> {
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
#[doc = "Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKSEL_SPEC;
impl crate::RegisterSpec for CLKSEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clksel::R`](R) reader structure"]
impl crate::Readable for CLKSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clksel::W`](W) writer structure"]
impl crate::Writable for CLKSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSEL to value 0"]
impl crate::Resettable for CLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
