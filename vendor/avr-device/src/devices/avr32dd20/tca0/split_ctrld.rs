#[doc = "Register `CTRLD` reader"]
pub type R = crate::R<SPLIT_CTRLD_SPEC>;
#[doc = "Register `CTRLD` writer"]
pub type W = crate::W<SPLIT_CTRLD_SPEC>;
#[doc = "Field `SPLITM` reader - Split Mode Enable"]
pub type SPLITM_R = crate::BitReader;
#[doc = "Field `SPLITM` writer - Split Mode Enable"]
pub type SPLITM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Split Mode Enable"]
    #[inline(always)]
    pub fn splitm(&self) -> SPLITM_R {
        SPLITM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Split Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn splitm(&mut self) -> SPLITM_W<SPLIT_CTRLD_SPEC> {
        SPLITM_W::new(self, 0)
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
#[doc = "Control D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_ctrld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_ctrld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPLIT_CTRLD_SPEC;
impl crate::RegisterSpec for SPLIT_CTRLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`split_ctrld::R`](R) reader structure"]
impl crate::Readable for SPLIT_CTRLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`split_ctrld::W`](W) writer structure"]
impl crate::Writable for SPLIT_CTRLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::Resettable for SPLIT_CTRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
