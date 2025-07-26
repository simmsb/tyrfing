#[doc = "Register `UDMFN` reader"]
pub type R = crate::R<UDMFN_SPEC>;
#[doc = "Register `UDMFN` writer"]
pub type W = crate::W<UDMFN_SPEC>;
#[doc = "Field `FNCERR` reader - Frame Number CRC Error Flag"]
pub type FNCERR_R = crate::BitReader;
#[doc = "Field `FNCERR` writer - Frame Number CRC Error Flag"]
pub type FNCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Frame Number CRC Error Flag"]
    #[inline(always)]
    pub fn fncerr(&self) -> FNCERR_R {
        FNCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Frame Number CRC Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fncerr(&mut self) -> FNCERR_W<UDMFN_SPEC> {
        FNCERR_W::new(self, 4)
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
#[doc = "USB Device Micro Frame Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmfn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmfn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDMFN_SPEC;
impl crate::RegisterSpec for UDMFN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`udmfn::R`](R) reader structure"]
impl crate::Readable for UDMFN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udmfn::W`](W) writer structure"]
impl crate::Writable for UDMFN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDMFN to value 0"]
impl crate::Resettable for UDMFN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
