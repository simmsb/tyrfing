#[doc = "Register `ASSR` reader"]
pub type R = crate::R<ASSR_SPEC>;
#[doc = "Register `ASSR` writer"]
pub type W = crate::W<ASSR_SPEC>;
#[doc = "Field `TCR2BUB` reader - Timer/Counter2 Control Register B Update Busy"]
pub type TCR2BUB_R = crate::BitReader;
#[doc = "Field `TCR2BUB` writer - Timer/Counter2 Control Register B Update Busy"]
pub type TCR2BUB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCR2AUB` reader - Timer/Counter2 Control Register A Update Busy"]
pub type TCR2AUB_R = crate::BitReader;
#[doc = "Field `TCR2AUB` writer - Timer/Counter2 Control Register A Update Busy"]
pub type TCR2AUB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCR2BUB` reader - Timer/Counter2 Output Compare Register B Update Busy"]
pub type OCR2BUB_R = crate::BitReader;
#[doc = "Field `OCR2BUB` writer - Timer/Counter2 Output Compare Register B Update Busy"]
pub type OCR2BUB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCR2AUB` reader - Timer/Counter2 Output Compare Register A Update Busy"]
pub type OCR2AUB_R = crate::BitReader;
#[doc = "Field `OCR2AUB` writer - Timer/Counter2 Output Compare Register A Update Busy"]
pub type OCR2AUB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCN2UB` reader - Timer/Counter2 Update Busy"]
pub type TCN2UB_R = crate::BitReader;
#[doc = "Field `TCN2UB` writer - Timer/Counter2 Update Busy"]
pub type TCN2UB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AS2` reader - Timer/Counter2 Asynchronous Mode"]
pub type AS2_R = crate::BitReader;
#[doc = "Field `AS2` writer - Timer/Counter2 Asynchronous Mode"]
pub type AS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCLK` reader - Enable External Clock Input"]
pub type EXCLK_R = crate::BitReader;
#[doc = "Field `EXCLK` writer - Enable External Clock Input"]
pub type EXCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCLKAMR` reader - Enable External Clock Input for AMR"]
pub type EXCLKAMR_R = crate::BitReader;
#[doc = "Field `EXCLKAMR` writer - Enable External Clock Input for AMR"]
pub type EXCLKAMR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter2 Control Register B Update Busy"]
    #[inline(always)]
    pub fn tcr2bub(&self) -> TCR2BUB_R {
        TCR2BUB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter2 Control Register A Update Busy"]
    #[inline(always)]
    pub fn tcr2aub(&self) -> TCR2AUB_R {
        TCR2AUB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter2 Output Compare Register B Update Busy"]
    #[inline(always)]
    pub fn ocr2bub(&self) -> OCR2BUB_R {
        OCR2BUB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter2 Output Compare Register A Update Busy"]
    #[inline(always)]
    pub fn ocr2aub(&self) -> OCR2AUB_R {
        OCR2AUB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer/Counter2 Update Busy"]
    #[inline(always)]
    pub fn tcn2ub(&self) -> TCN2UB_R {
        TCN2UB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter2 Asynchronous Mode"]
    #[inline(always)]
    pub fn as2(&self) -> AS2_R {
        AS2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable External Clock Input"]
    #[inline(always)]
    pub fn exclk(&self) -> EXCLK_R {
        EXCLK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable External Clock Input for AMR"]
    #[inline(always)]
    pub fn exclkamr(&self) -> EXCLKAMR_R {
        EXCLKAMR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter2 Control Register B Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcr2bub(&mut self) -> TCR2BUB_W<ASSR_SPEC> {
        TCR2BUB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer/Counter2 Control Register A Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcr2aub(&mut self) -> TCR2AUB_W<ASSR_SPEC> {
        TCR2AUB_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter2 Output Compare Register B Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn ocr2bub(&mut self) -> OCR2BUB_W<ASSR_SPEC> {
        OCR2BUB_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer/Counter2 Output Compare Register A Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn ocr2aub(&mut self) -> OCR2AUB_W<ASSR_SPEC> {
        OCR2AUB_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timer/Counter2 Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcn2ub(&mut self) -> TCN2UB_W<ASSR_SPEC> {
        TCN2UB_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer/Counter2 Asynchronous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn as2(&mut self) -> AS2_W<ASSR_SPEC> {
        AS2_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable External Clock Input"]
    #[inline(always)]
    #[must_use]
    pub fn exclk(&mut self) -> EXCLK_W<ASSR_SPEC> {
        EXCLK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable External Clock Input for AMR"]
    #[inline(always)]
    #[must_use]
    pub fn exclkamr(&mut self) -> EXCLKAMR_W<ASSR_SPEC> {
        EXCLKAMR_W::new(self, 7)
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
#[doc = "Asynchronous Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`assr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASSR_SPEC;
impl crate::RegisterSpec for ASSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`assr::R`](R) reader structure"]
impl crate::Readable for ASSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`assr::W`](W) writer structure"]
impl crate::Writable for ASSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASSR to value 0"]
impl crate::Resettable for ASSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
