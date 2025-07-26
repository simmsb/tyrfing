#[doc = "Register `AMISCR` reader"]
pub type R = crate::R<AMISCR_SPEC>;
#[doc = "Register `AMISCR` writer"]
pub type W = crate::W<AMISCR_SPEC>;
#[doc = "Field `ISRCEN` reader - Current Source Enable"]
pub type ISRCEN_R = crate::BitReader;
#[doc = "Field `ISRCEN` writer - Current Source Enable"]
pub type ISRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current Source Enable"]
    #[inline(always)]
    pub fn isrcen(&self) -> ISRCEN_R {
        ISRCEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Current Source Enable"]
    #[inline(always)]
    #[must_use]
    pub fn isrcen(&mut self) -> ISRCEN_W<AMISCR_SPEC> {
        ISRCEN_W::new(self, 0)
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
#[doc = "Analog Miscellaneous Control Register (Shared with AD_CONVERTER IO_MODULE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amiscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amiscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AMISCR_SPEC;
impl crate::RegisterSpec for AMISCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`amiscr::R`](R) reader structure"]
impl crate::Readable for AMISCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`amiscr::W`](W) writer structure"]
impl crate::Writable for AMISCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMISCR to value 0"]
impl crate::Resettable for AMISCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
