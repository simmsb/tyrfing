#[doc = "Register `MCLKCTRLA` reader"]
pub type R = crate::R<MCLKCTRLA_SPEC>;
#[doc = "Register `MCLKCTRLA` writer"]
pub type W = crate::W<MCLKCTRLA_SPEC>;
#[doc = "Field `CLKSEL` reader - Clock select"]
pub type CLKSEL_R = crate::FieldReader<CLKSEL_A>;
#[doc = "Clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: 20MHz internal oscillator"]
    OSC20M = 0,
    #[doc = "1: 32KHz internal Ultra Low Power oscillator"]
    OSCULP32K = 1,
    #[doc = "3: External clock"]
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
    pub const fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::OSC20M),
            1 => Some(CLKSEL_A::OSCULP32K),
            3 => Some(CLKSEL_A::EXTCLK),
            _ => None,
        }
    }
    #[doc = "20MHz internal oscillator"]
    #[inline(always)]
    pub fn is_osc20m(&self) -> bool {
        *self == CLKSEL_A::OSC20M
    }
    #[doc = "32KHz internal Ultra Low Power oscillator"]
    #[inline(always)]
    pub fn is_osculp32k(&self) -> bool {
        *self == CLKSEL_A::OSCULP32K
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == CLKSEL_A::EXTCLK
    }
}
#[doc = "Field `CLKSEL` writer - Clock select"]
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLKSEL_A>;
impl<'a, REG> CLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "20MHz internal oscillator"]
    #[inline(always)]
    pub fn osc20m(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::OSC20M)
    }
    #[doc = "32KHz internal Ultra Low Power oscillator"]
    #[inline(always)]
    pub fn osculp32k(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::OSCULP32K)
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::EXTCLK)
    }
}
#[doc = "Field `CLKOUT` reader - System clock out"]
pub type CLKOUT_R = crate::BitReader;
#[doc = "Field `CLKOUT` writer - System clock out"]
pub type CLKOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Clock select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(self.bits & 3)
    }
    #[doc = "Bit 7 - System clock out"]
    #[inline(always)]
    pub fn clkout(&self) -> CLKOUT_R {
        CLKOUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<MCLKCTRLA_SPEC> {
        CLKSEL_W::new(self, 0)
    }
    #[doc = "Bit 7 - System clock out"]
    #[inline(always)]
    #[must_use]
    pub fn clkout(&mut self) -> CLKOUT_W<MCLKCTRLA_SPEC> {
        CLKOUT_W::new(self, 7)
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
#[doc = "MCLK Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCLKCTRLA_SPEC;
impl crate::RegisterSpec for MCLKCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mclkctrla::R`](R) reader structure"]
impl crate::Readable for MCLKCTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mclkctrla::W`](W) writer structure"]
impl crate::Writable for MCLKCTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKCTRLA to value 0"]
impl crate::Resettable for MCLKCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
