#[doc = "Register `LINDLR` reader"]
pub type R = crate::R<LINDLR_SPEC>;
#[doc = "Register `LINDLR` writer"]
pub type W = crate::W<LINDLR_SPEC>;
#[doc = "Field `LRXDL` reader - LIN Receive Data Length bits"]
pub type LRXDL_R = crate::FieldReader;
#[doc = "Field `LRXDL` writer - LIN Receive Data Length bits"]
pub type LRXDL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `LTXDL` reader - LIN Transmit Data Length bits"]
pub type LTXDL_R = crate::FieldReader;
#[doc = "Field `LTXDL` writer - LIN Transmit Data Length bits"]
pub type LTXDL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - LIN Receive Data Length bits"]
    #[inline(always)]
    pub fn lrxdl(&self) -> LRXDL_R {
        LRXDL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - LIN Transmit Data Length bits"]
    #[inline(always)]
    pub fn ltxdl(&self) -> LTXDL_R {
        LTXDL_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - LIN Receive Data Length bits"]
    #[inline(always)]
    #[must_use]
    pub fn lrxdl(&mut self) -> LRXDL_W<LINDLR_SPEC> {
        LRXDL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - LIN Transmit Data Length bits"]
    #[inline(always)]
    #[must_use]
    pub fn ltxdl(&mut self) -> LTXDL_W<LINDLR_SPEC> {
        LTXDL_W::new(self, 4)
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
#[doc = "LIN Data Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lindlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lindlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINDLR_SPEC;
impl crate::RegisterSpec for LINDLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lindlr::R`](R) reader structure"]
impl crate::Readable for LINDLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lindlr::W`](W) writer structure"]
impl crate::Writable for LINDLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINDLR to value 0"]
impl crate::Resettable for LINDLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
