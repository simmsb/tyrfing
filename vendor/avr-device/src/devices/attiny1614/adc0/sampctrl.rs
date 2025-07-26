#[doc = "Register `SAMPCTRL` reader"]
pub type R = crate::R<SAMPCTRL_SPEC>;
#[doc = "Register `SAMPCTRL` writer"]
pub type W = crate::W<SAMPCTRL_SPEC>;
#[doc = "Field `SAMPLEN` reader - Sample lenght"]
pub type SAMPLEN_R = crate::FieldReader;
#[doc = "Field `SAMPLEN` writer - Sample lenght"]
pub type SAMPLEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Sample lenght"]
    #[inline(always)]
    pub fn samplen(&self) -> SAMPLEN_R {
        SAMPLEN_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sample lenght"]
    #[inline(always)]
    #[must_use]
    pub fn samplen(&mut self) -> SAMPLEN_W<SAMPCTRL_SPEC> {
        SAMPLEN_W::new(self, 0)
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
#[doc = "Sample Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMPCTRL_SPEC;
impl crate::RegisterSpec for SAMPCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sampctrl::R`](R) reader structure"]
impl crate::Readable for SAMPCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sampctrl::W`](W) writer structure"]
impl crate::Writable for SAMPCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPCTRL to value 0"]
impl crate::Resettable for SAMPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
