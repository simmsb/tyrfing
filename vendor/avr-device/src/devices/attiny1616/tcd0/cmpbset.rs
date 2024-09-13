#[doc = "Register `CMPBSET` reader"]
pub type R = crate::R<CMPBSET_SPEC>;
#[doc = "Register `CMPBSET` writer"]
pub type W = crate::W<CMPBSET_SPEC>;
#[doc = "Field `CMPBSET` reader - Compare B Set"]
pub type CMPBSET_R = crate::FieldReader<u16>;
#[doc = "Field `CMPBSET` writer - Compare B Set"]
pub type CMPBSET_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Compare B Set"]
    #[inline(always)]
    pub fn cmpbset(&self) -> CMPBSET_R {
        CMPBSET_R::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - Compare B Set"]
    #[inline(always)]
    #[must_use]
    pub fn cmpbset(&mut self) -> CMPBSET_W<CMPBSET_SPEC> {
        CMPBSET_W::new(self, 0)
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
#[doc = "Compare B Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpbset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpbset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPBSET_SPEC;
impl crate::RegisterSpec for CMPBSET_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cmpbset::R`](R) reader structure"]
impl crate::Readable for CMPBSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpbset::W`](W) writer structure"]
impl crate::Writable for CMPBSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPBSET to value 0"]
impl crate::Resettable for CMPBSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
