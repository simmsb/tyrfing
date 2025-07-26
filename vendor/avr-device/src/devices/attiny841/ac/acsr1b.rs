#[doc = "Register `ACSR1B` reader"]
pub type R = crate::R<ACSR1B_SPEC>;
#[doc = "Register `ACSR1B` writer"]
pub type W = crate::W<ACSR1B_SPEC>;
#[doc = "Field `ACME1` reader - Analog Comparator 1 Multiplexer Enable"]
pub type ACME1_R = crate::BitReader;
#[doc = "Field `ACME1` writer - Analog Comparator 1 Multiplexer Enable"]
pub type ACME1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACOE1` reader - Analog Comparator 1 Output Pin Enable"]
pub type ACOE1_R = crate::BitReader;
#[doc = "Field `ACOE1` writer - Analog Comparator 1 Output Pin Enable"]
pub type ACOE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLEV1` reader - Analog Comparator 1 Hysteresis Level"]
pub type HLEV1_R = crate::BitReader;
#[doc = "Field `HLEV1` writer - Analog Comparator 1 Hysteresis Level"]
pub type HLEV1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEL1` reader - Analog Comparator 1 Hysteresis Select"]
pub type HSEL1_R = crate::BitReader;
#[doc = "Field `HSEL1` writer - Analog Comparator 1 Hysteresis Select"]
pub type HSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Analog Comparator 1 Multiplexer Enable"]
    #[inline(always)]
    pub fn acme1(&self) -> ACME1_R {
        ACME1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator 1 Output Pin Enable"]
    #[inline(always)]
    pub fn acoe1(&self) -> ACOE1_R {
        ACOE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator 1 Hysteresis Level"]
    #[inline(always)]
    pub fn hlev1(&self) -> HLEV1_R {
        HLEV1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 1 Hysteresis Select"]
    #[inline(always)]
    pub fn hsel1(&self) -> HSEL1_R {
        HSEL1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Analog Comparator 1 Multiplexer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acme1(&mut self) -> ACME1_W<ACSR1B_SPEC> {
        ACME1_W::new(self, 2)
    }
    #[doc = "Bit 4 - Analog Comparator 1 Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acoe1(&mut self) -> ACOE1_W<ACSR1B_SPEC> {
        ACOE1_W::new(self, 4)
    }
    #[doc = "Bit 6 - Analog Comparator 1 Hysteresis Level"]
    #[inline(always)]
    #[must_use]
    pub fn hlev1(&mut self) -> HLEV1_W<ACSR1B_SPEC> {
        HLEV1_W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog Comparator 1 Hysteresis Select"]
    #[inline(always)]
    #[must_use]
    pub fn hsel1(&mut self) -> HSEL1_W<ACSR1B_SPEC> {
        HSEL1_W::new(self, 7)
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
#[doc = "Analog Comparator 1 Control And Status Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr1b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr1b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACSR1B_SPEC;
impl crate::RegisterSpec for ACSR1B_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`acsr1b::R`](R) reader structure"]
impl crate::Readable for ACSR1B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acsr1b::W`](W) writer structure"]
impl crate::Writable for ACSR1B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSR1B to value 0"]
impl crate::Resettable for ACSR1B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
