#[doc = "Register `CMPASET` reader"]
pub type R = crate::R<CMPASET_SPEC>;
#[doc = "Register `CMPASET` writer"]
pub type W = crate::W<CMPASET_SPEC>;
#[doc = "Field `CMPASET` reader - Compare A Set"]
pub type CMPASET_R = crate::FieldReader<u16>;
#[doc = "Field `CMPASET` writer - Compare A Set"]
pub type CMPASET_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Compare A Set"]
    #[inline(always)]
    pub fn cmpaset(&self) -> CMPASET_R {
        CMPASET_R::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - Compare A Set"]
    #[inline(always)]
    #[must_use]
    pub fn cmpaset(&mut self) -> CMPASET_W<CMPASET_SPEC> {
        CMPASET_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Compare A Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpaset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpaset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPASET_SPEC;
impl crate::RegisterSpec for CMPASET_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cmpaset::R`](R) reader structure"]
impl crate::Readable for CMPASET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpaset::W`](W) writer structure"]
impl crate::Writable for CMPASET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPASET to value 0"]
impl crate::Resettable for CMPASET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
