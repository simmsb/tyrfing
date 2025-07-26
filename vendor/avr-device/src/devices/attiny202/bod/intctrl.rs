#[doc = "Register `INTCTRL` reader"]
pub type R = crate::R<INTCTRL_SPEC>;
#[doc = "Register `INTCTRL` writer"]
pub type W = crate::W<INTCTRL_SPEC>;
#[doc = "Field `VLMIE` reader - voltage level monitor interrrupt enable"]
pub type VLMIE_R = crate::BitReader;
#[doc = "Field `VLMIE` writer - voltage level monitor interrrupt enable"]
pub type VLMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLMCFG` reader - Configuration"]
pub type VLMCFG_R = crate::FieldReader<VLMCFG_A>;
#[doc = "Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VLMCFG_A {
    #[doc = "0: Interrupt when supply goes below VLM level"]
    BELOW = 0,
    #[doc = "1: Interrupt when supply goes above VLM level"]
    ABOVE = 1,
    #[doc = "2: Interrupt when supply crosses VLM level"]
    CROSS = 2,
}
impl From<VLMCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: VLMCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VLMCFG_A {
    type Ux = u8;
}
impl VLMCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VLMCFG_A> {
        match self.bits {
            0 => Some(VLMCFG_A::BELOW),
            1 => Some(VLMCFG_A::ABOVE),
            2 => Some(VLMCFG_A::CROSS),
            _ => None,
        }
    }
    #[doc = "Interrupt when supply goes below VLM level"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == VLMCFG_A::BELOW
    }
    #[doc = "Interrupt when supply goes above VLM level"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == VLMCFG_A::ABOVE
    }
    #[doc = "Interrupt when supply crosses VLM level"]
    #[inline(always)]
    pub fn is_cross(&self) -> bool {
        *self == VLMCFG_A::CROSS
    }
}
#[doc = "Field `VLMCFG` writer - Configuration"]
pub type VLMCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VLMCFG_A>;
impl<'a, REG> VLMCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt when supply goes below VLM level"]
    #[inline(always)]
    pub fn below(self) -> &'a mut crate::W<REG> {
        self.variant(VLMCFG_A::BELOW)
    }
    #[doc = "Interrupt when supply goes above VLM level"]
    #[inline(always)]
    pub fn above(self) -> &'a mut crate::W<REG> {
        self.variant(VLMCFG_A::ABOVE)
    }
    #[doc = "Interrupt when supply crosses VLM level"]
    #[inline(always)]
    pub fn cross(self) -> &'a mut crate::W<REG> {
        self.variant(VLMCFG_A::CROSS)
    }
}
impl R {
    #[doc = "Bit 0 - voltage level monitor interrrupt enable"]
    #[inline(always)]
    pub fn vlmie(&self) -> VLMIE_R {
        VLMIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Configuration"]
    #[inline(always)]
    pub fn vlmcfg(&self) -> VLMCFG_R {
        VLMCFG_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - voltage level monitor interrrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vlmie(&mut self) -> VLMIE_W<INTCTRL_SPEC> {
        VLMIE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn vlmcfg(&mut self) -> VLMCFG_W<INTCTRL_SPEC> {
        VLMCFG_W::new(self, 1)
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
#[doc = "Voltage level monitor interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCTRL_SPEC;
impl crate::RegisterSpec for INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intctrl::R`](R) reader structure"]
impl crate::Readable for INTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intctrl::W`](W) writer structure"]
impl crate::Writable for INTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::Resettable for INTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
