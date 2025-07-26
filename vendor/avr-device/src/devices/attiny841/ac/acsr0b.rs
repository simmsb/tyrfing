#[doc = "Register `ACSR0B` reader"]
pub type R = crate::R<ACSR0B_SPEC>;
#[doc = "Register `ACSR0B` writer"]
pub type W = crate::W<ACSR0B_SPEC>;
#[doc = "Field `ACPMUX` reader - Analog Comparator 0 Positive Input Multiplexer Bits 1:0"]
pub type ACPMUX_R = crate::FieldReader;
#[doc = "Field `ACPMUX` writer - Analog Comparator 0 Positive Input Multiplexer Bits 1:0"]
pub type ACPMUX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `ACNMUX` reader - Analog Comparator 0 Negative Input Multiplexer"]
pub type ACNMUX_R = crate::FieldReader;
#[doc = "Field `ACNMUX` writer - Analog Comparator 0 Negative Input Multiplexer"]
pub type ACNMUX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `ACOE0` reader - Analog Comparator 0 Output Pin Enable"]
pub type ACOE0_R = crate::BitReader;
#[doc = "Field `ACOE0` writer - Analog Comparator 0 Output Pin Enable"]
pub type ACOE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLEV0` reader - Analog Comparator 0 Hysteresis Level"]
pub type HLEV0_R = crate::BitReader;
#[doc = "Field `HLEV0` writer - Analog Comparator 0 Hysteresis Level"]
pub type HLEV0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEL0` reader - Analog Comparator 0 Hysteresis Select"]
pub type HSEL0_R = crate::BitReader;
#[doc = "Field `HSEL0` writer - Analog Comparator 0 Hysteresis Select"]
pub type HSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Analog Comparator 0 Positive Input Multiplexer Bits 1:0"]
    #[inline(always)]
    pub fn acpmux(&self) -> ACPMUX_R {
        ACPMUX_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Analog Comparator 0 Negative Input Multiplexer"]
    #[inline(always)]
    pub fn acnmux(&self) -> ACNMUX_R {
        ACNMUX_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Analog Comparator 0 Output Pin Enable"]
    #[inline(always)]
    pub fn acoe0(&self) -> ACOE0_R {
        ACOE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator 0 Hysteresis Level"]
    #[inline(always)]
    pub fn hlev0(&self) -> HLEV0_R {
        HLEV0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 0 Hysteresis Select"]
    #[inline(always)]
    pub fn hsel0(&self) -> HSEL0_R {
        HSEL0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Comparator 0 Positive Input Multiplexer Bits 1:0"]
    #[inline(always)]
    #[must_use]
    pub fn acpmux(&mut self) -> ACPMUX_W<ACSR0B_SPEC> {
        ACPMUX_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Analog Comparator 0 Negative Input Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn acnmux(&mut self) -> ACNMUX_W<ACSR0B_SPEC> {
        ACNMUX_W::new(self, 2)
    }
    #[doc = "Bit 4 - Analog Comparator 0 Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acoe0(&mut self) -> ACOE0_W<ACSR0B_SPEC> {
        ACOE0_W::new(self, 4)
    }
    #[doc = "Bit 6 - Analog Comparator 0 Hysteresis Level"]
    #[inline(always)]
    #[must_use]
    pub fn hlev0(&mut self) -> HLEV0_W<ACSR0B_SPEC> {
        HLEV0_W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog Comparator 0 Hysteresis Select"]
    #[inline(always)]
    #[must_use]
    pub fn hsel0(&mut self) -> HSEL0_W<ACSR0B_SPEC> {
        HSEL0_W::new(self, 7)
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
#[doc = "Analog Comparator 0 Control And Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr0b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr0b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACSR0B_SPEC;
impl crate::RegisterSpec for ACSR0B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`acsr0b::R`](R) reader structure"]
impl crate::Readable for ACSR0B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acsr0b::W`](W) writer structure"]
impl crate::Writable for ACSR0B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSR0B to value 0"]
impl crate::Resettable for ACSR0B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
