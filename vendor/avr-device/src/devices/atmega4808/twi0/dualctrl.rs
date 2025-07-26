#[doc = "Register `DUALCTRL` reader"]
pub type R = crate::R<DUALCTRL_SPEC>;
#[doc = "Register `DUALCTRL` writer"]
pub type W = crate::W<DUALCTRL_SPEC>;
#[doc = "Field `ENABLE` reader - Dual Control Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Dual Control Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMPEN` reader - FM Plus Enable"]
pub type FMPEN_R = crate::BitReader;
#[doc = "Field `FMPEN` writer - FM Plus Enable"]
pub type FMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDAHOLD` reader - SDA Hold Time"]
pub type SDAHOLD_R = crate::FieldReader<SDAHOLD_A>;
#[doc = "SDA Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDAHOLD_A {
    #[doc = "0: SDA hold time off"]
    OFF = 0,
    #[doc = "1: Typical 50ns hold time"]
    _50NS = 1,
    #[doc = "2: Typical 300ns hold time"]
    _300NS = 2,
    #[doc = "3: Typical 500ns hold time"]
    _500NS = 3,
}
impl From<SDAHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: SDAHOLD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDAHOLD_A {
    type Ux = u8;
}
impl SDAHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDAHOLD_A {
        match self.bits {
            0 => SDAHOLD_A::OFF,
            1 => SDAHOLD_A::_50NS,
            2 => SDAHOLD_A::_300NS,
            3 => SDAHOLD_A::_500NS,
            _ => unreachable!(),
        }
    }
    #[doc = "SDA hold time off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SDAHOLD_A::OFF
    }
    #[doc = "Typical 50ns hold time"]
    #[inline(always)]
    pub fn is_50ns(&self) -> bool {
        *self == SDAHOLD_A::_50NS
    }
    #[doc = "Typical 300ns hold time"]
    #[inline(always)]
    pub fn is_300ns(&self) -> bool {
        *self == SDAHOLD_A::_300NS
    }
    #[doc = "Typical 500ns hold time"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == SDAHOLD_A::_500NS
    }
}
#[doc = "Field `SDAHOLD` writer - SDA Hold Time"]
pub type SDAHOLD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SDAHOLD_A>;
impl<'a, REG> SDAHOLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDA hold time off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLD_A::OFF)
    }
    #[doc = "Typical 50ns hold time"]
    #[inline(always)]
    pub fn _50ns(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLD_A::_50NS)
    }
    #[doc = "Typical 300ns hold time"]
    #[inline(always)]
    pub fn _300ns(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLD_A::_300NS)
    }
    #[doc = "Typical 500ns hold time"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLD_A::_500NS)
    }
}
impl R {
    #[doc = "Bit 0 - Dual Control Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FM Plus Enable"]
    #[inline(always)]
    pub fn fmpen(&self) -> FMPEN_R {
        FMPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&self) -> SDAHOLD_R {
        SDAHOLD_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Dual Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<DUALCTRL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - FM Plus Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmpen(&mut self) -> FMPEN_W<DUALCTRL_SPEC> {
        FMPEN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - SDA Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn sdahold(&mut self) -> SDAHOLD_W<DUALCTRL_SPEC> {
        SDAHOLD_W::new(self, 2)
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
#[doc = "Dual Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dualctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dualctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DUALCTRL_SPEC;
impl crate::RegisterSpec for DUALCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dualctrl::R`](R) reader structure"]
impl crate::Readable for DUALCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dualctrl::W`](W) writer structure"]
impl crate::Writable for DUALCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DUALCTRL to value 0"]
impl crate::Resettable for DUALCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
