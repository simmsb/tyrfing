#[doc = "Register `UERST` reader"]
pub type R = crate::R<UERST_SPEC>;
#[doc = "Register `UERST` writer"]
pub type W = crate::W<UERST_SPEC>;
#[doc = "Field `EPRST` reader - Endpoint FIFO Reset Bits"]
pub type EPRST_R = crate::FieldReader;
#[doc = "Field `EPRST` writer - Endpoint FIFO Reset Bits"]
pub type EPRST_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Endpoint FIFO Reset Bits"]
    #[inline(always)]
    pub fn eprst(&self) -> EPRST_R {
        EPRST_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Endpoint FIFO Reset Bits"]
    #[inline(always)]
    #[must_use]
    pub fn eprst(&mut self) -> EPRST_W<UERST_SPEC> {
        EPRST_W::new(self, 0)
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
#[doc = "USB Endpoint Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uerst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uerst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UERST_SPEC;
impl crate::RegisterSpec for UERST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uerst::R`](R) reader structure"]
impl crate::Readable for UERST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uerst::W`](W) writer structure"]
impl crate::Writable for UERST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UERST to value 0"]
impl crate::Resettable for UERST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
