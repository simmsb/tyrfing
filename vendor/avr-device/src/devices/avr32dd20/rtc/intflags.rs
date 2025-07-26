#[doc = "Register `INTFLAGS` reader"]
pub type R = crate::R<INTFLAGS_SPEC>;
#[doc = "Register `INTFLAGS` writer"]
pub type W = crate::W<INTFLAGS_SPEC>;
#[doc = "Field `OVF` reader - Overflow Interrupt Flag"]
pub type OVF_R = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow Interrupt Flag"]
pub type OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP` reader - Compare Match Interrupt"]
pub type CMP_R = crate::BitReader;
#[doc = "Field `CMP` writer - Compare Match Interrupt"]
pub type CMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Match Interrupt"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<INTFLAGS_SPEC> {
        OVF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare Match Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<INTFLAGS_SPEC> {
        CMP_W::new(self, 1)
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
#[doc = "Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflags::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflags::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFLAGS_SPEC;
impl crate::RegisterSpec for INTFLAGS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflags::R`](R) reader structure"]
impl crate::Readable for INTFLAGS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intflags::W`](W) writer structure"]
impl crate::Writable for INTFLAGS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGS to value 0"]
impl crate::Resettable for INTFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
