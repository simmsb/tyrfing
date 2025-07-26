#[doc = "Register `ACSRB` reader"]
pub type R = crate::R<ACSRB_SPEC>;
#[doc = "Register `ACSRB` writer"]
pub type W = crate::W<ACSRB_SPEC>;
#[doc = "Field `ACPMUX` reader - Analog Comparator Positive Input Multiplexer Bits 1:0"]
pub type ACPMUX_R = crate::FieldReader;
#[doc = "Field `ACPMUX` writer - Analog Comparator Positive Input Multiplexer Bits 1:0"]
pub type ACPMUX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `ACNMUX` reader - Analog Comparator Negative Input Multiplexer"]
pub type ACNMUX_R = crate::FieldReader;
#[doc = "Field `ACNMUX` writer - Analog Comparator Negative Input Multiplexer"]
pub type ACNMUX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `HLEV` reader - Hysteresis Level"]
pub type HLEV_R = crate::BitReader;
#[doc = "Field `HLEV` writer - Hysteresis Level"]
pub type HLEV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEL` reader - Hysteresis Select"]
pub type HSEL_R = crate::BitReader;
#[doc = "Field `HSEL` writer - Hysteresis Select"]
pub type HSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Analog Comparator Positive Input Multiplexer Bits 1:0"]
    #[inline(always)]
    pub fn acpmux(&self) -> ACPMUX_R {
        ACPMUX_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Analog Comparator Negative Input Multiplexer"]
    #[inline(always)]
    pub fn acnmux(&self) -> ACNMUX_R {
        ACNMUX_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 6 - Hysteresis Level"]
    #[inline(always)]
    pub fn hlev(&self) -> HLEV_R {
        HLEV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Hysteresis Select"]
    #[inline(always)]
    pub fn hsel(&self) -> HSEL_R {
        HSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Comparator Positive Input Multiplexer Bits 1:0"]
    #[inline(always)]
    #[must_use]
    pub fn acpmux(&mut self) -> ACPMUX_W<ACSRB_SPEC> {
        ACPMUX_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Analog Comparator Negative Input Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn acnmux(&mut self) -> ACNMUX_W<ACSRB_SPEC> {
        ACNMUX_W::new(self, 2)
    }
    #[doc = "Bit 6 - Hysteresis Level"]
    #[inline(always)]
    #[must_use]
    pub fn hlev(&mut self) -> HLEV_W<ACSRB_SPEC> {
        HLEV_W::new(self, 6)
    }
    #[doc = "Bit 7 - Hysteresis Select"]
    #[inline(always)]
    #[must_use]
    pub fn hsel(&mut self) -> HSEL_W<ACSRB_SPEC> {
        HSEL_W::new(self, 7)
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
#[doc = "Analog Comparator Control And Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACSRB_SPEC;
impl crate::RegisterSpec for ACSRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`acsrb::R`](R) reader structure"]
impl crate::Readable for ACSRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acsrb::W`](W) writer structure"]
impl crate::Writable for ACSRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSRB to value 0"]
impl crate::Resettable for ACSRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
