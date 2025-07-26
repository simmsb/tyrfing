#[doc = "Register `ASSR` reader"]
pub type R = crate::R<ASSR_SPEC>;
#[doc = "Register `ASSR` writer"]
pub type W = crate::W<ASSR_SPEC>;
#[doc = "Field `TCR2UB` reader - Timer/counter Control Register2 Update Busy"]
pub type TCR2UB_R = crate::BitReader;
#[doc = "Field `TCR2UB` writer - Timer/counter Control Register2 Update Busy"]
pub type TCR2UB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCR2UB` reader - Output Compare Register2 Update Busy"]
pub type OCR2UB_R = crate::BitReader;
#[doc = "Field `OCR2UB` writer - Output Compare Register2 Update Busy"]
pub type OCR2UB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCN2UB` reader - Timer/Counter2 Update Busy"]
pub type TCN2UB_R = crate::BitReader;
#[doc = "Field `TCN2UB` writer - Timer/Counter2 Update Busy"]
pub type TCN2UB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AS2` reader - Asynchronous Timer/counter2"]
pub type AS2_R = crate::BitReader;
#[doc = "Field `AS2` writer - Asynchronous Timer/counter2"]
pub type AS2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/counter Control Register2 Update Busy"]
    #[inline(always)]
    pub fn tcr2ub(&self) -> TCR2UB_R {
        TCR2UB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Compare Register2 Update Busy"]
    #[inline(always)]
    pub fn ocr2ub(&self) -> OCR2UB_R {
        OCR2UB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter2 Update Busy"]
    #[inline(always)]
    pub fn tcn2ub(&self) -> TCN2UB_R {
        TCN2UB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Asynchronous Timer/counter2"]
    #[inline(always)]
    pub fn as2(&self) -> AS2_R {
        AS2_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/counter Control Register2 Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcr2ub(&mut self) -> TCR2UB_W<ASSR_SPEC> {
        TCR2UB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Compare Register2 Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn ocr2ub(&mut self) -> OCR2UB_W<ASSR_SPEC> {
        OCR2UB_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter2 Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcn2ub(&mut self) -> TCN2UB_W<ASSR_SPEC> {
        TCN2UB_W::new(self, 2)
    }
    #[doc = "Bit 3 - Asynchronous Timer/counter2"]
    #[inline(always)]
    #[must_use]
    pub fn as2(&mut self) -> AS2_W<ASSR_SPEC> {
        AS2_W::new(self, 3)
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
