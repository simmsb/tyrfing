#[doc = "Register `PHDE` reader"]
pub type R = crate::R<PHDE_SPEC>;
#[doc = "Register `PHDE` writer"]
pub type W = crate::W<PHDE_SPEC>;
#[doc = "Field `PHDEA` reader - PortA High Drive Enable"]
pub type PHDEA_R = crate::FieldReader;
#[doc = "Field `PHDEA` writer - PortA High Drive Enable"]
pub type PHDEA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PortA High Drive Enable"]
    #[inline(always)]
    pub fn phdea(&self) -> PHDEA_R {
        PHDEA_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - PortA High Drive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn phdea(&mut self) -> PHDEA_W<PHDE_SPEC> {
        PHDEA_W::new(self, 0)
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
#[doc = "Port High Drive Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phde::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phde::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHDE_SPEC;
impl crate::RegisterSpec for PHDE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phde::R`](R) reader structure"]
impl crate::Readable for PHDE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phde::W`](W) writer structure"]
impl crate::Writable for PHDE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHDE to value 0"]
impl crate::Resettable for PHDE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
