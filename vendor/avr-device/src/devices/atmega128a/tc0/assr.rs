#[doc = "Register `ASSR` reader"]
pub type R = crate::R<ASSR_SPEC>;
#[doc = "Register `ASSR` writer"]
pub type W = crate::W<ASSR_SPEC>;
#[doc = "Field `TCR0UB` reader - Timer/Counter Control Register 0 Update Busy"]
pub type TCR0UB_R = crate::BitReader;
#[doc = "Field `TCR0UB` writer - Timer/Counter Control Register 0 Update Busy"]
pub type TCR0UB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCR0UB` reader - Output Compare register 0 Busy"]
pub type OCR0UB_R = crate::BitReader;
#[doc = "Field `OCR0UB` writer - Output Compare register 0 Busy"]
pub type OCR0UB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCN0UB` reader - Timer/Counter0 Update Busy"]
pub type TCN0UB_R = crate::BitReader;
#[doc = "Field `TCN0UB` writer - Timer/Counter0 Update Busy"]
pub type TCN0UB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AS0` reader - Asynchronus Timer/Counter 0"]
pub type AS0_R = crate::BitReader;
#[doc = "Field `AS0` writer - Asynchronus Timer/Counter 0"]
pub type AS0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter Control Register 0 Update Busy"]
    #[inline(always)]
    pub fn tcr0ub(&self) -> TCR0UB_R {
        TCR0UB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Compare register 0 Busy"]
    #[inline(always)]
    pub fn ocr0ub(&self) -> OCR0UB_R {
        OCR0UB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter0 Update Busy"]
    #[inline(always)]
    pub fn tcn0ub(&self) -> TCN0UB_R {
        TCN0UB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Asynchronus Timer/Counter 0"]
    #[inline(always)]
    pub fn as0(&self) -> AS0_R {
        AS0_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter Control Register 0 Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcr0ub(&mut self) -> TCR0UB_W<ASSR_SPEC> {
        TCR0UB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Compare register 0 Busy"]
    #[inline(always)]
    #[must_use]
    pub fn ocr0ub(&mut self) -> OCR0UB_W<ASSR_SPEC> {
        OCR0UB_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter0 Update Busy"]
    #[inline(always)]
    #[must_use]
    pub fn tcn0ub(&mut self) -> TCN0UB_W<ASSR_SPEC> {
        TCN0UB_W::new(self, 2)
    }
    #[doc = "Bit 3 - Asynchronus Timer/Counter 0"]
    #[inline(always)]
    #[must_use]
    pub fn as0(&mut self) -> AS0_W<ASSR_SPEC> {
        AS0_W::new(self, 3)
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
#[doc = "Asynchronus Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`assr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
